//! UI-independent document, navigation and search infrastructure for Atlas UI.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
    future::Future,
    pin::Pin,
    sync::Arc,
};

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Metadata required to route and describe a native document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentMetadata {
    /// Human-readable title.
    pub title: String,
    /// Short description used by indexes and previews.
    pub summary: String,
    /// BCP-47 language identifier.
    pub language: String,
    /// Canonical internal route. It never triggers navigation by itself.
    pub canonical_route: String,
    /// Ordered breadcrumb labels from root to document.
    pub breadcrumbs: Vec<String>,
}

/// Inline document content independent of any parser or renderer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum InlineContent {
    /// Unstyled text.
    Text(String),
    /// Emphasized content.
    Emphasis(Vec<Self>),
    /// Strong content.
    Strong(Vec<Self>),
    /// Inline code.
    Code(String),
    /// A controlled link target.
    Link {
        label: Vec<Self>,
        destination: String,
    },
}

/// Semantic admonition variants supported by the document model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmonitionKind {
    /// Neutral note.
    Note,
    /// Helpful recommendation.
    Tip,
    /// Informational message.
    Info,
    /// Caution requiring attention.
    Caution,
    /// Warning with elevated risk.
    Warning,
    /// Dangerous or destructive condition.
    Danger,
}

/// A semantic block consumed by presentation adapters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DocumentBlock {
    /// Heading with deterministic deep-link anchor.
    Heading {
        level: u8,
        anchor: String,
        content: Vec<InlineContent>,
    },
    /// Paragraph content.
    Paragraph(Vec<InlineContent>),
    /// Ordered or unordered list.
    List {
        ordered: bool,
        items: Vec<Vec<InlineContent>>,
    },
    /// Fenced or indented code.
    Code { language: String, source: String },
    /// Documentation table.
    Table {
        headers: Vec<Vec<InlineContent>>,
        rows: Vec<Vec<Vec<InlineContent>>>,
    },
    /// Image whose URL is data only; loading remains a host decision.
    Image {
        source: String,
        alternative: String,
        title: String,
    },
    /// Quoted content.
    Quote(Vec<InlineContent>),
    /// Explicit callout.
    Admonition {
        kind: AdmonitionKind,
        title: String,
        body: Vec<InlineContent>,
    },
}

/// Complete parser-independent document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentModel {
    /// Routing and discovery metadata.
    pub metadata: DocumentMetadata,
    /// Ordered semantic blocks.
    pub blocks: Vec<DocumentBlock>,
}

/// Stable, flattened block for a Slint or other presentation adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentationBlock {
    /// Stable block kind such as `heading` or `paragraph`.
    pub kind: String,
    /// Optional anchor.
    pub anchor: String,
    /// Plain accessible content.
    pub text: String,
    /// Optional language, destination or media source.
    pub detail: String,
}

impl DocumentModel {
    /// Flattens semantic blocks without leaking parser types into the UI.
    #[must_use]
    pub fn presentation_blocks(&self) -> Vec<PresentationBlock> {
        self.blocks.iter().map(PresentationBlock::from).collect()
    }
}

impl From<&DocumentBlock> for PresentationBlock {
    fn from(block: &DocumentBlock) -> Self {
        match block {
            DocumentBlock::Heading {
                anchor, content, ..
            } => Self::new("heading", anchor, plain(content), ""),
            DocumentBlock::Paragraph(content) => Self::new("paragraph", "", plain(content), ""),
            DocumentBlock::List { items, ordered } => Self::new(
                "list",
                "",
                items
                    .iter()
                    .map(|item| plain(item))
                    .collect::<Vec<_>>()
                    .join("\n"),
                if *ordered { "ordered" } else { "unordered" },
            ),
            DocumentBlock::Code { language, source } => {
                Self::new("code", "", source.clone(), language.clone())
            }
            DocumentBlock::Table { headers, rows } => {
                let mut lines = vec![
                    headers
                        .iter()
                        .map(|cell| plain(cell))
                        .collect::<Vec<_>>()
                        .join(" | "),
                ];
                lines.extend(rows.iter().map(|row| {
                    row.iter()
                        .map(|cell| plain(cell))
                        .collect::<Vec<_>>()
                        .join(" | ")
                }));
                Self::new("table", "", lines.join("\n"), "")
            }
            DocumentBlock::Image {
                source,
                alternative,
                title,
            } => Self::new(
                "image",
                "",
                alternative.clone(),
                format!("{source}\n{title}"),
            ),
            DocumentBlock::Quote(content) => Self::new("quote", "", plain(content), ""),
            DocumentBlock::Admonition { kind, title, body } => Self::new(
                "admonition",
                "",
                format!("{title}\n{}", plain(body)),
                format!("{kind:?}").to_lowercase(),
            ),
        }
    }
}

impl PresentationBlock {
    fn new(kind: &str, anchor: &str, text: String, detail: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            anchor: anchor.into(),
            text,
            detail: detail.into(),
        }
    }
}

fn plain(content: &[InlineContent]) -> String {
    content
        .iter()
        .map(|item| match item {
            InlineContent::Text(text) | InlineContent::Code(text) => text.clone(),
            InlineContent::Emphasis(children) | InlineContent::Strong(children) => plain(children),
            InlineContent::Link { label, .. } => plain(label),
        })
        .collect()
}

/// Flattened inline span preserving semantic formatting for presentation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // Orthogonal modifiers map directly to Slint fields.
pub struct PresentationInline {
    /// Text including any meaningful trailing spacing.
    pub text: String,
    /// Whether emphasis applies to this span.
    pub emphasis: bool,
    /// Whether strong emphasis applies to this span.
    pub strong: bool,
    /// Whether the span uses the inline-code treatment.
    pub code: bool,
    /// Controlled destination, empty for non-link spans.
    pub destination: String,
    /// Whether layout should begin a new visual line after this span.
    pub line_break_after: bool,
}

/// Presentation item for ordered and unordered document lists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentationListItem {
    /// Stable marker such as `1.` or `•`.
    pub marker: String,
    /// Plain accessible label for the complete item.
    pub accessible_text: String,
    /// Styled spans retained for richer presentation adapters.
    pub content: Vec<PresentationInline>,
}

/// Flattens nested inline semantics into bounded, wrapping-friendly spans.
#[must_use]
pub fn presentation_inlines(content: &[InlineContent]) -> Vec<PresentationInline> {
    let mut output = Vec::new();
    flatten_inlines(content, &InlineContext::default(), &mut output);
    output
}

/// Converts one semantic list block into numbered or bulleted presentation rows.
#[must_use]
pub fn presentation_list_items(block: &DocumentBlock) -> Vec<PresentationListItem> {
    let DocumentBlock::List { ordered, items } = block else {
        return Vec::new();
    };
    items
        .iter()
        .enumerate()
        .map(|(index, content)| PresentationListItem {
            marker: if *ordered {
                format!("{}.", index + 1)
            } else {
                "•".into()
            },
            accessible_text: plain(content),
            content: presentation_inlines(content),
        })
        .collect()
}

/// Semantic kind with an independent deterministic numbering sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocumentReferenceKind {
    /// Figure, diagram or other visual media.
    Figure,
    /// Documentation table rather than an interactive data grid.
    Table,
    /// Explanatory note referenced from document content.
    Note,
}

impl DocumentReferenceKind {
    fn id_prefix(self) -> &'static str {
        match self {
            Self::Figure => "figure",
            Self::Table => "table",
            Self::Note => "note",
        }
    }
}

/// Resolved reference target owned by the parser-independent document layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentReferenceTarget {
    /// Unique anchor used for navigation and deep links.
    pub id: String,
    /// Semantic target kind.
    pub kind: DocumentReferenceKind,
    /// One-based number within this kind.
    pub number: usize,
    /// Plain accessible caption.
    pub caption: String,
}

/// Explicit failure when declaring or resolving a document reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentReferenceError {
    /// A caption is required for an autogenerated accessible identity.
    EmptyCaption,
    /// An explicit identifier is already registered.
    DuplicateId(String),
    /// No target exists for the requested identifier.
    UnknownId(String),
}

/// Controlled navigation intention resolved from a cross-reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceNavigationRequest {
    /// Target anchor to pass to the document navigation controller.
    pub target_id: String,
    /// Target kind used by accessibility and presentation adapters.
    pub kind: DocumentReferenceKind,
    /// One-based target number.
    pub number: usize,
    /// Whether the destination should receive focus after scrolling.
    pub focus_destination: bool,
}

/// Deterministic registry for captions, numbering and cross-references.
#[derive(Clone, Debug, Default)]
pub struct DocumentReferenceRegistry {
    targets: BTreeMap<String, DocumentReferenceTarget>,
    counts: BTreeMap<DocumentReferenceKind, usize>,
}

impl DocumentReferenceRegistry {
    /// Registers one target in document order.
    ///
    /// An empty `requested_id` derives a stable identifier from kind and
    /// caption, adding a numeric suffix when another generated id collides.
    ///
    /// # Errors
    ///
    /// Returns `EmptyCaption` when no accessible caption exists, or
    /// `DuplicateId` when a caller-provided identifier is already registered.
    pub fn register(
        &mut self,
        kind: DocumentReferenceKind,
        requested_id: &str,
        caption: impl Into<String>,
    ) -> Result<DocumentReferenceTarget, DocumentReferenceError> {
        let caption = caption.into();
        if caption.trim().is_empty() {
            return Err(DocumentReferenceError::EmptyCaption);
        }
        let explicit = !requested_id.trim().is_empty();
        let base = if explicit {
            slug(requested_id)
        } else {
            format!("{}-{}", kind.id_prefix(), slug(&caption))
        };
        if explicit && self.targets.contains_key(&base) {
            return Err(DocumentReferenceError::DuplicateId(base));
        }
        let id = if explicit {
            base
        } else {
            unique_reference_id(&base, &self.targets)
        };
        let number = self.counts.get(&kind).copied().unwrap_or(0) + 1;
        let target = DocumentReferenceTarget {
            id: id.clone(),
            kind,
            number,
            caption,
        };
        self.counts.insert(kind, number);
        self.targets.insert(id, target.clone());
        Ok(target)
    }

    /// Resolves a target without causing navigation.
    ///
    /// # Errors
    ///
    /// Returns `UnknownId` when no declaration owns this identifier.
    pub fn resolve(&self, id: &str) -> Result<&DocumentReferenceTarget, DocumentReferenceError> {
        self.targets
            .get(id)
            .ok_or_else(|| DocumentReferenceError::UnknownId(id.into()))
    }

    /// Produces an explicit navigation request for a registered target.
    ///
    /// # Errors
    ///
    /// Returns `UnknownId` when the cross-reference cannot be resolved.
    pub fn navigation_request(
        &self,
        id: &str,
        focus_destination: bool,
    ) -> Result<ReferenceNavigationRequest, DocumentReferenceError> {
        let target = self.resolve(id)?;
        Ok(ReferenceNavigationRequest {
            target_id: target.id.clone(),
            kind: target.kind,
            number: target.number,
            focus_destination,
        })
    }

    /// Returns targets in deterministic identifier order for indexes or audits.
    pub fn targets(&self) -> impl Iterator<Item = &DocumentReferenceTarget> {
        self.targets.values()
    }
}

fn unique_reference_id(base: &str, targets: &BTreeMap<String, DocumentReferenceTarget>) -> String {
    if !targets.contains_key(base) {
        return base.into();
    }
    let mut suffix = 2;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !targets.contains_key(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

/// Optional bibliographic metadata attached to a footnote.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CitationMetadata {
    /// Author or responsible organization.
    pub author: String,
    /// Work title.
    pub title: String,
    /// Publisher, journal or site name.
    pub source: String,
    /// Display year or publication date.
    pub date: String,
    /// Destination retained as data until destination policy is applied.
    pub destination: String,
}

/// One declared note and every stable caller that references it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FootnoteDefinition {
    /// Stable note anchor.
    pub id: String,
    /// One-based document order.
    pub number: usize,
    /// Plain accessible note content.
    pub content: String,
    /// Optional source metadata.
    pub citation: Option<CitationMetadata>,
    /// Caller anchors in registration order.
    pub callers: Vec<String>,
}

/// Resolved inline call to a footnote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FootnoteCall {
    /// Referenced note anchor.
    pub note_id: String,
    /// Stable caller anchor used for focus return.
    pub caller_id: String,
    /// Display number shared by every call to this note.
    pub number: usize,
}

/// Explicit footnote declaration or call failure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FootnoteError {
    /// Note content is required.
    EmptyContent,
    /// A note anchor is already declared.
    DuplicateNote(String),
    /// A caller anchor is already associated with the note.
    DuplicateCaller(String),
    /// The requested note is unknown.
    UnknownNote(String),
}

/// Direction of a controlled note navigation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FootnoteNavigationDirection {
    /// Move from an inline caller to its note.
    ToNote,
    /// Return from the note to the most recently activated caller.
    ReturnToCaller,
}

/// Navigation intention that never scrolls or changes focus by itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FootnoteNavigationRequest {
    /// Anchor to resolve through the document navigation host.
    pub target_id: String,
    /// Navigation direction for announcements and history policy.
    pub direction: FootnoteNavigationDirection,
    /// Destination receives focus after the host scroll completes.
    pub focus_destination: bool,
}

/// Registry for note declarations and repeated callers.
#[derive(Clone, Debug, Default)]
pub struct FootnoteRegistry {
    notes: BTreeMap<String, FootnoteDefinition>,
    order: Vec<String>,
}

impl FootnoteRegistry {
    /// Declares one note in document order.
    ///
    /// # Errors
    ///
    /// Returns an error for empty content or a duplicate note identifier.
    pub fn declare(
        &mut self,
        requested_id: &str,
        content: impl Into<String>,
        citation: Option<CitationMetadata>,
    ) -> Result<FootnoteDefinition, FootnoteError> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err(FootnoteError::EmptyContent);
        }
        let id = if requested_id.trim().is_empty() {
            unique_footnote_id(&format!("note-{}", slug(&content)), &self.notes)
        } else {
            slug(requested_id)
        };
        if self.notes.contains_key(&id) {
            return Err(FootnoteError::DuplicateNote(id));
        }
        let note = FootnoteDefinition {
            id: id.clone(),
            number: self.order.len() + 1,
            content,
            citation,
            callers: Vec::new(),
        };
        self.order.push(id.clone());
        self.notes.insert(id, note.clone());
        Ok(note)
    }

    /// Registers a stable caller for an existing note.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown note or duplicate caller anchor.
    pub fn register_call(
        &mut self,
        note_id: &str,
        caller_id: &str,
    ) -> Result<FootnoteCall, FootnoteError> {
        let note = self
            .notes
            .get_mut(note_id)
            .ok_or_else(|| FootnoteError::UnknownNote(note_id.into()))?;
        let caller_id = slug(caller_id);
        if note.callers.contains(&caller_id) {
            return Err(FootnoteError::DuplicateCaller(caller_id));
        }
        note.callers.push(caller_id.clone());
        Ok(FootnoteCall {
            note_id: note.id.clone(),
            caller_id,
            number: note.number,
        })
    }

    /// Iterates notes in declaration rather than map order.
    pub fn notes(&self) -> impl Iterator<Item = &FootnoteDefinition> {
        self.order.iter().filter_map(|id| self.notes.get(id))
    }
}

/// Focus-return controller preventing note/caller navigation loops.
#[derive(Clone, Debug, Default)]
pub struct FootnoteNavigationController {
    return_target: Option<(String, String)>,
}

impl FootnoteNavigationController {
    /// Records the active caller and requests focus on its note.
    #[must_use]
    pub fn open_note(&mut self, call: &FootnoteCall) -> FootnoteNavigationRequest {
        self.return_target = Some((call.note_id.clone(), call.caller_id.clone()));
        FootnoteNavigationRequest {
            target_id: call.note_id.clone(),
            direction: FootnoteNavigationDirection::ToNote,
            focus_destination: true,
        }
    }

    /// Consumes the stored caller once, avoiding a focus feedback loop.
    #[must_use]
    pub fn return_to_caller(&mut self, note_id: &str) -> Option<FootnoteNavigationRequest> {
        let (active_note, caller_id) = self.return_target.take()?;
        if active_note != note_id {
            self.return_target = Some((active_note, caller_id));
            return None;
        }
        Some(FootnoteNavigationRequest {
            target_id: caller_id,
            direction: FootnoteNavigationDirection::ReturnToCaller,
            focus_destination: true,
        })
    }
}

fn unique_footnote_id(base: &str, notes: &BTreeMap<String, FootnoteDefinition>) -> String {
    if !notes.contains_key(base) {
        return base.into();
    }
    let mut suffix = 2;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !notes.contains_key(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

/// Character-indexed selection range; end is exclusive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextSelectionRange {
    /// Unicode scalar index of the first selected character.
    pub start: usize,
    /// Exclusive Unicode scalar index after the final selected character.
    pub end: usize,
}

/// Explicit scope of a copy request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextCopyScope {
    /// Copy only the current non-empty selection.
    Selection,
    /// Copy the complete source.
    All,
}

/// Data passed to a host clipboard adapter after user intent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextCopyRequest {
    /// Requested scope for telemetry and announcements.
    pub scope: TextCopyScope,
    /// UTF-8 text already sliced at valid character boundaries.
    pub text: String,
}

/// Selection/copy failure that never triggers an implicit fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextSelectionError {
    /// Copy-selection was requested without a non-empty range.
    EmptySelection,
}

/// Replaceable host port. Calling it is always an explicit application action.
pub trait ClipboardPort {
    /// Writes text after the host accepts a `TextCopyRequest`.
    ///
    /// # Errors
    ///
    /// Returns an adapter-defined message when the platform clipboard fails.
    fn write_text(&mut self, text: &str) -> Result<(), String>;
}

/// Unicode-safe, UI-independent selection controller.
#[derive(Clone, Debug)]
pub struct TextSelectionController {
    source: Arc<str>,
    selection: Option<TextSelectionRange>,
}

impl TextSelectionController {
    /// Creates a controller sharing the immutable source allocation.
    #[must_use]
    pub fn new(source: impl Into<Arc<str>>) -> Self {
        Self {
            source: source.into(),
            selection: None,
        }
    }

    /// Normalizes reversed and out-of-range character indices.
    pub fn select(&mut self, start: usize, end: usize) -> Option<TextSelectionRange> {
        let length = self.source.chars().count();
        let (start, end) = if start <= end {
            (start.min(length), end.min(length))
        } else {
            (end.min(length), start.min(length))
        };
        self.selection = (start < end).then_some(TextSelectionRange { start, end });
        self.selection
    }

    /// Clears selection without changing the underlying source.
    pub fn clear(&mut self) {
        self.selection = None;
    }

    /// Returns the current normalized range.
    #[must_use]
    pub fn selection(&self) -> Option<TextSelectionRange> {
        self.selection
    }

    /// Builds a copy request without accessing the clipboard.
    ///
    /// # Errors
    ///
    /// Returns `EmptySelection` rather than silently copying all text when the
    /// requested selection is absent.
    pub fn copy_request(
        &self,
        scope: TextCopyScope,
    ) -> Result<TextCopyRequest, TextSelectionError> {
        let text = match scope {
            TextCopyScope::All => self.source.to_string(),
            TextCopyScope::Selection => {
                let range = self.selection.ok_or(TextSelectionError::EmptySelection)?;
                self.source
                    .chars()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .collect()
            }
        };
        Ok(TextCopyRequest { scope, text })
    }
}

#[derive(Clone, Debug, Default)]
struct InlineContext {
    emphasis: bool,
    strong: bool,
    code: bool,
    destination: String,
}

fn flatten_inlines(
    nodes: &[InlineContent],
    style: &InlineContext,
    output: &mut Vec<PresentationInline>,
) {
    for item in nodes {
        match item {
            InlineContent::Text(text) => push_inline_text(text, style, output),
            InlineContent::Code(text) => {
                let mut nested = style.clone();
                nested.code = true;
                push_inline_text(text, &nested, output);
            }
            InlineContent::Emphasis(children) => {
                let mut nested = style.clone();
                nested.emphasis = true;
                flatten_inlines(children, &nested, output);
            }
            InlineContent::Strong(children) => {
                let mut nested = style.clone();
                nested.strong = true;
                flatten_inlines(children, &nested, output);
            }
            InlineContent::Link { label, destination } => {
                let mut nested = style.clone();
                nested.destination.clone_from(destination);
                flatten_inlines(label, &nested, output);
            }
        }
    }
}

fn push_inline_text(text: &str, context: &InlineContext, output: &mut Vec<PresentationInline>) {
    for segment in text.split_inclusive(char::is_whitespace) {
        let line_break_after = segment.contains('\n');
        let text = segment.replace(['\r', '\n'], "");
        if text.is_empty() && !line_break_after {
            continue;
        }
        output.push(PresentationInline {
            text,
            emphasis: context.emphasis,
            strong: context.strong,
            code: context.code,
            destination: context.destination.clone(),
            line_break_after,
        });
    }
}

/// Explicit parsing failure. Unsafe HTML is rejected rather than interpreted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentParseError {
    /// Raw HTML was encountered.
    UnsafeHtml,
    /// Markdown events did not form a supported document structure.
    UnsupportedStructure(String),
}

impl std::fmt::Display for DocumentParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsafeHtml => formatter.write_str("raw HTML is forbidden in Atlas documents"),
            Self::UnsupportedStructure(reason) => {
                write!(formatter, "unsupported document structure: {reason}")
            }
        }
    }
}

impl std::error::Error for DocumentParseError {}

/// Replaceable document parser port.
pub trait DocumentParser {
    /// Parses trusted syntax into the parser-independent model.
    ///
    /// # Errors
    ///
    /// Returns an explicit error for unsafe HTML or unsupported structure.
    fn parse(
        &self,
        metadata: DocumentMetadata,
        source: &str,
    ) -> Result<DocumentModel, DocumentParseError>;
}

/// `pulldown-cmark` adapter with unsafe HTML disabled by rejection.
#[derive(Clone, Copy, Debug, Default)]
pub struct MarkdownParser;

impl DocumentParser for MarkdownParser {
    fn parse(
        &self,
        metadata: DocumentMetadata,
        source: &str,
    ) -> Result<DocumentModel, DocumentParseError> {
        let options =
            Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
        let events = Parser::new_ext(source, options).collect::<Vec<_>>();
        if events
            .iter()
            .any(|event| matches!(event, Event::Html(_) | Event::InlineHtml(_)))
        {
            return Err(DocumentParseError::UnsafeHtml);
        }
        let mut builder = MarkdownBuilder::default();
        for event in events {
            builder.event(event)?;
        }
        Ok(DocumentModel {
            metadata,
            blocks: builder.blocks,
        })
    }
}

#[derive(Default)]
struct MarkdownBuilder {
    blocks: Vec<DocumentBlock>,
    inline: Vec<InlineContent>,
    text: String,
    heading: Option<u8>,
    code_language: Option<String>,
    list: Option<(bool, Vec<Vec<InlineContent>>)>,
    table: Option<TableState>,
    anchors: BTreeMap<String, usize>,
    quote: bool,
    link: Option<(String, usize)>,
    image: Option<(String, String)>,
    style: Vec<&'static str>,
}

#[derive(Default)]
struct TableState {
    headers: Vec<Vec<InlineContent>>,
    rows: Vec<Vec<Vec<InlineContent>>>,
    current_row: Vec<Vec<InlineContent>>,
    in_head: bool,
}

impl MarkdownBuilder {
    #[allow(clippy::too_many_lines)]
    fn event(&mut self, event: Event<'_>) -> Result<(), DocumentParseError> {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                self.heading = Some(heading_level(level));
                self.inline.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                self.flush_text();
                let content = std::mem::take(&mut self.inline);
                let anchor = unique_anchor(&plain(&content), &mut self.anchors);
                self.blocks.push(DocumentBlock::Heading {
                    level: self.heading.take().unwrap_or(2),
                    anchor,
                    content,
                });
            }
            Event::Start(Tag::Paragraph | Tag::Item) => self.inline.clear(),
            Event::End(TagEnd::Paragraph) => {
                self.flush_text();
                let content = std::mem::take(&mut self.inline);
                if self.quote {
                    self.blocks.push(DocumentBlock::Quote(content));
                } else if let Some((_, items)) = &mut self.list {
                    items.push(content);
                } else if !content.is_empty() {
                    self.blocks.push(DocumentBlock::Paragraph(content));
                }
            }
            Event::Start(Tag::BlockQuote(_)) => self.quote = true,
            Event::End(TagEnd::BlockQuote(_)) => {
                self.quote = false;
                self.promote_admonition();
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                self.code_language = Some(match kind {
                    CodeBlockKind::Fenced(value) => value.into_string(),
                    CodeBlockKind::Indented => String::new(),
                });
                self.text.clear();
            }
            Event::End(TagEnd::CodeBlock) => self.blocks.push(DocumentBlock::Code {
                language: self.code_language.take().unwrap_or_default(),
                source: std::mem::take(&mut self.text),
            }),
            Event::Start(Tag::List(start)) => self.list = Some((start.is_some(), Vec::new())),
            Event::End(TagEnd::List(_)) => {
                if let Some((ordered, items)) = self.list.take() {
                    self.blocks.push(DocumentBlock::List { ordered, items });
                }
            }
            Event::End(TagEnd::Item) => {
                self.flush_text();
                if let Some((_, items)) = &mut self.list
                    && !self.inline.is_empty()
                {
                    items.push(std::mem::take(&mut self.inline));
                }
            }
            Event::Start(Tag::Emphasis) => {
                self.flush_text();
                self.style.push("emphasis");
            }
            Event::End(TagEnd::Emphasis) => {
                self.flush_text();
                self.wrap_last("emphasis");
            }
            Event::Start(Tag::Strong) => {
                self.flush_text();
                self.style.push("strong");
            }
            Event::End(TagEnd::Strong) => {
                self.flush_text();
                self.wrap_last("strong");
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                self.flush_text();
                self.link = Some((dest_url.into_string(), self.inline.len()));
            }
            Event::End(TagEnd::Link) => {
                self.flush_text();
                if let Some((destination, start)) = self.link.take() {
                    let label = self.inline.split_off(start);
                    self.inline.push(InlineContent::Link { label, destination });
                }
            }
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                self.image = Some((dest_url.into_string(), title.into_string()));
                self.text.clear();
            }
            Event::End(TagEnd::Image) => {
                if let Some((source, title)) = self.image.take() {
                    self.blocks.push(DocumentBlock::Image {
                        source,
                        alternative: std::mem::take(&mut self.text),
                        title,
                    });
                }
            }
            Event::Start(Tag::Table(_)) => self.table = Some(TableState::default()),
            Event::Start(Tag::TableHead) => {
                if let Some(table) = &mut self.table {
                    table.in_head = true;
                }
            }
            Event::End(TagEnd::TableHead) => {
                if let Some(table) = &mut self.table {
                    table.headers = std::mem::take(&mut table.current_row);
                    table.in_head = false;
                }
            }
            Event::Start(Tag::TableRow) => {
                if let Some(table) = &mut self.table {
                    table.current_row.clear();
                }
            }
            Event::Start(Tag::TableCell) => {
                self.inline.clear();
                self.text.clear();
            }
            Event::End(TagEnd::TableCell) => {
                self.flush_text();
                if let Some(table) = &mut self.table {
                    table.current_row.push(std::mem::take(&mut self.inline));
                }
            }
            Event::End(TagEnd::TableRow) => {
                if let Some(table) = &mut self.table {
                    let row = std::mem::take(&mut table.current_row);
                    if table.in_head {
                        table.headers = row;
                    } else {
                        table.rows.push(row);
                    }
                }
            }
            Event::End(TagEnd::Table) => {
                if let Some(table) = self.table.take() {
                    self.blocks.push(DocumentBlock::Table {
                        headers: table.headers,
                        rows: table.rows,
                    });
                }
            }
            Event::Code(value) => {
                self.flush_text();
                self.inline.push(InlineContent::Code(value.into_string()));
            }
            Event::Text(value) => self.text.push_str(&value),
            Event::SoftBreak | Event::HardBreak => self.text.push('\n'),
            Event::Rule => self.blocks.push(DocumentBlock::Paragraph(Vec::new())),
            Event::Html(_) | Event::InlineHtml(_) => return Err(DocumentParseError::UnsafeHtml),
            _ => {}
        }
        Ok(())
    }

    fn flush_text(&mut self) {
        if !self.text.is_empty() {
            self.inline
                .push(InlineContent::Text(std::mem::take(&mut self.text)));
        }
    }

    fn wrap_last(&mut self, expected: &'static str) {
        let _ = self.style.pop().filter(|style| *style == expected);
        if let Some(item) = self.inline.pop() {
            self.inline.push(if expected == "strong" {
                InlineContent::Strong(vec![item])
            } else {
                InlineContent::Emphasis(vec![item])
            });
        }
    }

    fn promote_admonition(&mut self) {
        let Some(DocumentBlock::Quote(content)) = self.blocks.last() else {
            return;
        };
        let text = plain(content);
        let Some(end) = text.find(']') else {
            return;
        };
        let Some(marker) = text.get(2..end) else {
            return;
        };
        let kind = match marker.to_ascii_uppercase().as_str() {
            "NOTE" => AdmonitionKind::Note,
            "TIP" => AdmonitionKind::Tip,
            "INFO" => AdmonitionKind::Info,
            "CAUTION" => AdmonitionKind::Caution,
            "WARNING" => AdmonitionKind::Warning,
            "DANGER" => AdmonitionKind::Danger,
            _ => return,
        };
        let body = text.get(end + 1..).unwrap_or_default().trim().to_owned();
        let title = marker.to_ascii_lowercase();
        self.blocks.pop();
        self.blocks.push(DocumentBlock::Admonition {
            kind,
            title,
            body: vec![InlineContent::Text(body)],
        });
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn unique_anchor(text: &str, anchors: &mut BTreeMap<String, usize>) -> String {
    let base = slug(text);
    let count = anchors.entry(base.clone()).or_default();
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}-{}", *count)
    }
}

/// Produces a deterministic Unicode anchor while normalizing separators.
#[must_use]
pub fn slug(text: &str) -> String {
    let mut result = String::new();
    let mut separator = false;
    for character in text.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if separator && !result.is_empty() {
                result.push('-');
            }
            result.push(character);
            separator = false;
        } else {
            separator = true;
        }
    }
    if result.is_empty() {
        "section".into()
    } else {
        result
    }
}

/// Internal route with optional deep-link anchor.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct DocumentRoute {
    pub path: String,
    pub anchor: Option<String>,
    pub scroll_offset: f32,
}

/// Destination category resolved without performing navigation or I/O.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationKind {
    /// Application-owned route, fragment or relative path.
    Internal,
    /// Valid `http` or `https` URL.
    Web,
    /// Valid `mailto` destination.
    Email,
}

/// Validation failure for a destination supplied by document content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationValidationError {
    /// The destination is empty after trimming.
    Empty,
    /// Control characters, whitespace or backslashes make the value ambiguous.
    UnsafeCharacters,
    /// A network-path reference such as `//example.test` has no explicit scheme.
    AmbiguousNetworkPath,
    /// The scheme is unsupported and must never be delegated implicitly.
    UnsupportedScheme,
    /// A supported external scheme has an invalid authority or payload.
    InvalidExternalDestination,
}

/// Validated destination with normalized scheme and host metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassifiedDestination {
    /// Trimmed value to pass back to the host after an allow decision.
    pub destination: String,
    /// Resolved destination category.
    pub kind: DestinationKind,
    /// Lowercase web host, absent for internal and e-mail destinations.
    pub host: Option<String>,
}

/// Host policy applied to a valid external destination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExternalDestinationPolicy {
    /// The host may execute the destination without another prompt.
    Allow,
    /// The host must request explicit user confirmation before execution.
    Confirm,
    /// The host must refuse execution.
    Deny,
}

/// Action returned to the host. Evaluating it never performs navigation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationAction {
    /// Resolve through Atlas application routing.
    NavigateInternal,
    /// The host may open the validated external destination.
    OpenExternal,
    /// The host must display confirmation before opening the destination.
    ConfirmExternal,
    /// The destination must not be executed.
    Deny,
}

/// Stable reason accompanying a destination decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DestinationDecisionReason {
    /// The destination belongs to application routing.
    InternalRoute,
    /// The configured policy explicitly permits this destination.
    PolicyAllowed,
    /// The configured policy requires confirmation.
    ConfirmationRequired,
    /// The configured policy or host deny-list refuses this destination.
    PolicyDenied,
    /// Classification failed before any policy was applied.
    Invalid(DestinationValidationError),
}

/// Complete, auditable result to consume before any host-side navigation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DestinationDecision {
    /// Trimmed original destination.
    pub destination: String,
    /// Category when validation succeeded.
    pub kind: Option<DestinationKind>,
    /// Lowercase web host when available.
    pub host: Option<String>,
    /// Explicit action expected from the host.
    pub action: DestinationAction,
    /// Explanation suitable for logs and deterministic UI states.
    pub reason: DestinationDecisionReason,
}

/// Pure policy engine for document destinations.
///
/// External destinations require confirmation by default. Exact lowercase host
/// entries may override the web policy; deny entries always take precedence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DestinationPolicy {
    web: ExternalDestinationPolicy,
    email: ExternalDestinationPolicy,
    allowed_web_hosts: BTreeSet<String>,
    denied_web_hosts: BTreeSet<String>,
}

impl Default for DestinationPolicy {
    fn default() -> Self {
        Self {
            web: ExternalDestinationPolicy::Confirm,
            email: ExternalDestinationPolicy::Confirm,
            allowed_web_hosts: BTreeSet::new(),
            denied_web_hosts: BTreeSet::new(),
        }
    }
}

impl DestinationPolicy {
    /// Replaces the fallback policy for valid web URLs.
    #[must_use]
    pub fn with_web_policy(mut self, policy: ExternalDestinationPolicy) -> Self {
        self.web = policy;
        self
    }

    /// Replaces the fallback policy for valid e-mail destinations.
    #[must_use]
    pub fn with_email_policy(mut self, policy: ExternalDestinationPolicy) -> Self {
        self.email = policy;
        self
    }

    /// Allows one exact DNS host regardless of the fallback web policy.
    #[must_use]
    pub fn with_allowed_web_host(mut self, host: impl AsRef<str>) -> Self {
        if let Some(host) = normalized_policy_host(host.as_ref()) {
            self.allowed_web_hosts.insert(host);
        }
        self
    }

    /// Denies one exact DNS host, taking precedence over the allow-list.
    #[must_use]
    pub fn with_denied_web_host(mut self, host: impl AsRef<str>) -> Self {
        if let Some(host) = normalized_policy_host(host.as_ref()) {
            self.denied_web_hosts.insert(host);
        }
        self
    }

    /// Classifies and evaluates a destination without executing it.
    #[must_use]
    pub fn decide(&self, destination: &str) -> DestinationDecision {
        let trimmed = destination.trim().to_owned();
        let classified = match classify_destination(destination) {
            Ok(classified) => classified,
            Err(error) => {
                return DestinationDecision {
                    destination: trimmed,
                    kind: None,
                    host: None,
                    action: DestinationAction::Deny,
                    reason: DestinationDecisionReason::Invalid(error),
                };
            }
        };
        if classified.kind == DestinationKind::Internal {
            return decision(
                classified,
                DestinationAction::NavigateInternal,
                DestinationDecisionReason::InternalRoute,
            );
        }
        let policy = if classified.kind == DestinationKind::Web {
            let host = classified.host.as_deref().unwrap_or_default();
            if self.denied_web_hosts.contains(host) {
                ExternalDestinationPolicy::Deny
            } else if self.allowed_web_hosts.contains(host) {
                ExternalDestinationPolicy::Allow
            } else {
                self.web
            }
        } else {
            self.email
        };
        let (action, reason) = match policy {
            ExternalDestinationPolicy::Allow => (
                DestinationAction::OpenExternal,
                DestinationDecisionReason::PolicyAllowed,
            ),
            ExternalDestinationPolicy::Confirm => (
                DestinationAction::ConfirmExternal,
                DestinationDecisionReason::ConfirmationRequired,
            ),
            ExternalDestinationPolicy::Deny => (
                DestinationAction::Deny,
                DestinationDecisionReason::PolicyDenied,
            ),
        };
        decision(classified, action, reason)
    }
}

fn decision(
    classified: ClassifiedDestination,
    action: DestinationAction,
    reason: DestinationDecisionReason,
) -> DestinationDecision {
    DestinationDecision {
        destination: classified.destination,
        kind: Some(classified.kind),
        host: classified.host,
        action,
        reason,
    }
}

/// Validates and classifies a destination without consulting host policy.
///
/// # Errors
///
/// Returns a precise validation error for empty or ambiguous values,
/// unsupported schemes, and malformed supported external destinations.
pub fn classify_destination(
    destination: &str,
) -> Result<ClassifiedDestination, DestinationValidationError> {
    let destination = destination.trim();
    if destination.is_empty() {
        return Err(DestinationValidationError::Empty);
    }
    if destination
        .chars()
        .any(|character| character.is_control() || character.is_whitespace() || character == '\\')
    {
        return Err(DestinationValidationError::UnsafeCharacters);
    }
    if destination.starts_with("//") {
        return Err(DestinationValidationError::AmbiguousNetworkPath);
    }
    let Some(colon) = destination.find(':') else {
        return Ok(ClassifiedDestination {
            destination: destination.into(),
            kind: DestinationKind::Internal,
            host: None,
        });
    };
    let scheme = &destination[..colon];
    if !valid_scheme(scheme) {
        return Err(DestinationValidationError::UnsupportedScheme);
    }
    match scheme.to_ascii_lowercase().as_str() {
        "http" | "https" => classify_web_destination(destination, colon),
        "mailto" => classify_email_destination(destination, colon),
        _ => Err(DestinationValidationError::UnsupportedScheme),
    }
}

fn valid_scheme(scheme: &str) -> bool {
    let mut characters = scheme.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
}

fn classify_web_destination(
    destination: &str,
    colon: usize,
) -> Result<ClassifiedDestination, DestinationValidationError> {
    let remainder = &destination[colon + 1..];
    let authority_and_path = remainder
        .strip_prefix("//")
        .ok_or(DestinationValidationError::InvalidExternalDestination)?;
    let authority_end = authority_and_path
        .find(['/', '?', '#'])
        .unwrap_or(authority_and_path.len());
    let authority = &authority_and_path[..authority_end];
    if authority.is_empty() || authority.contains('@') {
        return Err(DestinationValidationError::InvalidExternalDestination);
    }
    let host = web_host(authority)?;
    Ok(ClassifiedDestination {
        destination: destination.into(),
        kind: DestinationKind::Web,
        host: Some(host),
    })
}

fn web_host(authority: &str) -> Result<String, DestinationValidationError> {
    let (host, port) = if authority.starts_with('[') {
        let end = authority
            .find(']')
            .ok_or(DestinationValidationError::InvalidExternalDestination)?;
        let host = &authority[..=end];
        let suffix = &authority[end + 1..];
        let port = if suffix.is_empty() {
            None
        } else {
            Some(
                suffix
                    .strip_prefix(':')
                    .ok_or(DestinationValidationError::InvalidExternalDestination)?,
            )
        };
        (host, port)
    } else {
        let mut parts = authority.rsplitn(2, ':');
        let last = parts.next().unwrap_or_default();
        match parts.next() {
            Some(host) => (host, Some(last)),
            None => (last, None),
        }
    };
    if host.is_empty()
        || !host.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '[' | ']' | ':')
        })
        || port.is_some_and(|port| port.is_empty() || !port.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(DestinationValidationError::InvalidExternalDestination);
    }
    Ok(host.to_ascii_lowercase())
}

fn classify_email_destination(
    destination: &str,
    colon: usize,
) -> Result<ClassifiedDestination, DestinationValidationError> {
    let payload = &destination[colon + 1..];
    let address = payload.split(['?', '#']).next().unwrap_or_default();
    let mut parts = address.split('@');
    if parts.next().is_none_or(str::is_empty)
        || parts.next().is_none_or(str::is_empty)
        || parts.next().is_some()
    {
        return Err(DestinationValidationError::InvalidExternalDestination);
    }
    Ok(ClassifiedDestination {
        destination: destination.into(),
        kind: DestinationKind::Email,
        host: None,
    })
}

fn normalized_policy_host(host: &str) -> Option<String> {
    let host = host.trim().to_ascii_lowercase();
    (!host.is_empty()
        && host
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '-')))
    .then_some(host)
}

/// Stable identity of a lazily loaded document revision.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DocumentLoadKey {
    /// Internal canonical route.
    pub route: String,
    /// BCP-47 content language.
    pub language: String,
    /// Host-defined revision used to invalidate cached content.
    pub revision: String,
}

/// Numbered request passed to an asynchronous source adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentLoadRequest {
    /// Monotonic request identity used to reject late completions.
    pub request_id: u64,
    /// Requested document revision.
    pub key: DocumentLoadKey,
}

/// Shared source payload; cache and ready state retain the same allocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedDocument {
    /// Loaded document revision.
    pub key: DocumentLoadKey,
    /// Unparsed UTF-8 source owned by the adapter response.
    pub source: Arc<str>,
}

impl LoadedDocument {
    /// Creates a payload while converting the source into shared storage once.
    #[must_use]
    pub fn new(key: DocumentLoadKey, source: impl Into<Arc<str>>) -> Self {
        Self {
            key,
            source: source.into(),
        }
    }
}

/// Failure category supplied explicitly by a document source adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLoadErrorKind {
    /// No document exists for the requested key.
    NotFound,
    /// The configured source is temporarily unavailable.
    Unavailable,
    /// The adapter returned data that cannot satisfy its contract.
    InvalidSource,
    /// The document exceeds the configured per-document memory budget.
    BudgetExceeded,
}

/// Structured loading failure suitable for deterministic presentation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentLoadError {
    /// Stable failure category.
    pub kind: DocumentLoadErrorKind,
    /// Human-readable detail supplied by the adapter or controller.
    pub message: String,
    /// Whether the UI may offer an explicit retry intention.
    pub retryable: bool,
}

/// Runtime-neutral asynchronous source port.
///
/// Implementations may use a filesystem, embedded bundle, database or remote
/// client. Atlas never selects an adapter and never falls back to another one.
pub trait AsyncDocumentSource {
    /// Starts one request. Dropping the returned future may cancel host work.
    fn load(
        &self,
        request: DocumentLoadRequest,
    ) -> Pin<Box<dyn Future<Output = Result<LoadedDocument, DocumentLoadError>> + Send + '_>>;
}

/// Bounded cache configuration for source documents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentCacheBudget {
    /// Maximum number of cached revisions.
    pub max_entries: usize,
    /// Maximum combined UTF-8 bytes retained by the cache.
    pub max_cache_bytes: usize,
    /// Maximum accepted UTF-8 bytes for one document.
    pub max_document_bytes: usize,
}

impl Default for DocumentCacheBudget {
    fn default() -> Self {
        Self {
            max_entries: 64,
            max_cache_bytes: 8 * 1024 * 1024,
            max_document_bytes: 512 * 1024,
        }
    }
}

/// Observable loading state consumed by a host presentation adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentLoadingState {
    /// No request or ready document is active.
    Idle,
    /// One numbered request is expected to complete.
    Loading(DocumentLoadRequest),
    /// A document is ready, either from its source or cache.
    Ready {
        /// Shared payload.
        document: LoadedDocument,
        /// Whether this transition was served without source work.
        from_cache: bool,
    },
    /// The latest request failed explicitly.
    Error {
        /// Requested document revision.
        key: DocumentLoadKey,
        /// Structured failure.
        error: DocumentLoadError,
    },
}

/// Immediate outcome when loading a key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentLoadStart {
    /// The host must submit this request to its configured source.
    Request(DocumentLoadRequest),
    /// The bounded cache served the document synchronously.
    Cached(LoadedDocument),
}

/// Result of returning a source completion to the controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLoadCompletion {
    /// The result became the observable ready or error state.
    Applied,
    /// A newer request or cancellation made this completion obsolete.
    Stale,
    /// The adapter answered with a different document key.
    MismatchedKey,
    /// The source exceeded the configured per-document budget.
    BudgetExceeded,
}

/// Runtime-neutral lazy loading controller with a bounded LRU cache.
#[derive(Clone, Debug)]
pub struct DocumentLoadController {
    budget: DocumentCacheBudget,
    state: DocumentLoadingState,
    next_request_id: u64,
    cache: BTreeMap<DocumentLoadKey, LoadedDocument>,
    recency: VecDeque<DocumentLoadKey>,
    cached_bytes: usize,
}

impl DocumentLoadController {
    /// Creates an idle controller with explicit cache budgets.
    #[must_use]
    pub fn new(budget: DocumentCacheBudget) -> Self {
        Self {
            budget,
            state: DocumentLoadingState::Idle,
            next_request_id: 0,
            cache: BTreeMap::new(),
            recency: VecDeque::new(),
            cached_bytes: 0,
        }
    }

    /// Returns the current deterministic loading state.
    #[must_use]
    pub fn state(&self) -> &DocumentLoadingState {
        &self.state
    }

    /// Starts a load or resolves it immediately from the bounded cache.
    pub fn load(&mut self, key: DocumentLoadKey) -> DocumentLoadStart {
        if let Some(document) = self.cache.get(&key).cloned() {
            self.touch(&key);
            self.state = DocumentLoadingState::Ready {
                document: document.clone(),
                from_cache: true,
            };
            return DocumentLoadStart::Cached(document);
        }
        self.next_request_id = self.next_request_id.wrapping_add(1).max(1);
        let request = DocumentLoadRequest {
            request_id: self.next_request_id,
            key,
        };
        self.state = DocumentLoadingState::Loading(request.clone());
        DocumentLoadStart::Request(request)
    }

    /// Invalidates the active request. A late future completion becomes stale.
    pub fn cancel(&mut self) {
        if matches!(self.state, DocumentLoadingState::Loading(_)) {
            self.state = DocumentLoadingState::Idle;
        }
    }

    /// Applies a source result only when its request is still current.
    pub fn complete(
        &mut self,
        request_id: u64,
        result: Result<LoadedDocument, DocumentLoadError>,
    ) -> DocumentLoadCompletion {
        let DocumentLoadingState::Loading(active) = &self.state else {
            return DocumentLoadCompletion::Stale;
        };
        if active.request_id != request_id {
            return DocumentLoadCompletion::Stale;
        }
        let expected_key = active.key.clone();
        match result {
            Ok(document) if document.key != expected_key => DocumentLoadCompletion::MismatchedKey,
            Ok(document) if document.source.len() > self.budget.max_document_bytes => {
                self.state = DocumentLoadingState::Error {
                    key: expected_key,
                    error: DocumentLoadError {
                        kind: DocumentLoadErrorKind::BudgetExceeded,
                        message: "document exceeds max_document_bytes".into(),
                        retryable: false,
                    },
                };
                DocumentLoadCompletion::BudgetExceeded
            }
            Ok(document) => {
                self.insert_cache(document.clone());
                self.state = DocumentLoadingState::Ready {
                    document,
                    from_cache: false,
                };
                DocumentLoadCompletion::Applied
            }
            Err(error) => {
                self.state = DocumentLoadingState::Error {
                    key: expected_key,
                    error,
                };
                DocumentLoadCompletion::Applied
            }
        }
    }

    /// Removes every cached revision while preserving the active state.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.recency.clear();
        self.cached_bytes = 0;
    }

    /// Returns cache usage for budget instrumentation.
    #[must_use]
    pub fn cache_usage(&self) -> (usize, usize) {
        (self.cache.len(), self.cached_bytes)
    }

    fn insert_cache(&mut self, document: LoadedDocument) {
        if self.budget.max_entries == 0 || self.budget.max_cache_bytes == 0 {
            return;
        }
        let size = document.source.len();
        if size > self.budget.max_cache_bytes {
            return;
        }
        if let Some(previous) = self.cache.remove(&document.key) {
            self.cached_bytes = self.cached_bytes.saturating_sub(previous.source.len());
            self.recency.retain(|key| key != &document.key);
        }
        self.cached_bytes += size;
        self.recency.push_back(document.key.clone());
        self.cache.insert(document.key.clone(), document);
        while self.cache.len() > self.budget.max_entries
            || self.cached_bytes > self.budget.max_cache_bytes
        {
            let Some(oldest) = self.recency.pop_front() else {
                break;
            };
            if let Some(removed) = self.cache.remove(&oldest) {
                self.cached_bytes = self.cached_bytes.saturating_sub(removed.source.len());
            }
        }
    }

    fn touch(&mut self, key: &DocumentLoadKey) {
        self.recency.retain(|cached| cached != key);
        self.recency.push_back(key.clone());
    }
}

/// UI-independent history with per-route scroll restoration.
#[derive(Clone, Debug, Default)]
pub struct NavigationHistory {
    entries: Vec<DocumentRoute>,
    cursor: Option<usize>,
    scroll: BTreeMap<String, f32>,
}

impl NavigationHistory {
    /// Pushes a route, truncating forward history.
    pub fn navigate(&mut self, route: DocumentRoute) {
        if let Some(cursor) = self.cursor {
            self.entries.truncate(cursor + 1);
        }
        self.entries.push(route);
        self.cursor = Some(self.entries.len() - 1);
    }
    /// Returns the previous route.
    pub fn back(&mut self) -> Option<&DocumentRoute> {
        let cursor = self.cursor?;
        if cursor == 0 {
            return None;
        }
        self.cursor = Some(cursor - 1);
        self.current()
    }
    /// Returns the next route.
    pub fn forward(&mut self) -> Option<&DocumentRoute> {
        let cursor = self.cursor?;
        if cursor + 1 >= self.entries.len() {
            return None;
        }
        self.cursor = Some(cursor + 1);
        self.current()
    }
    /// Returns the current route.
    #[must_use]
    pub fn current(&self) -> Option<&DocumentRoute> {
        self.cursor.and_then(|index| self.entries.get(index))
    }
    /// Stores scroll without changing history.
    pub fn remember_scroll(&mut self, path: &str, offset: f32) {
        self.scroll.insert(path.into(), offset.max(0.0));
    }
    /// Retrieves the last known scroll position.
    #[must_use]
    pub fn restored_scroll(&self, path: &str) -> f32 {
        self.scroll.get(path).copied().unwrap_or(0.0)
    }

    /// Replaces the current deep-link anchor without pushing another history
    /// entry. Scrollspy uses this for URL synchronization without loops.
    pub fn replace_current_anchor(&mut self, anchor: Option<String>) {
        if let Some(cursor) = self.cursor
            && let Some(route) = self.entries.get_mut(cursor)
        {
            route.anchor = anchor;
        }
    }

    /// Stores the current entry's exact offset and its route-level fallback.
    pub fn remember_current_scroll(&mut self, offset: f32) {
        let Some(cursor) = self.cursor else { return };
        let Some(route) = self.entries.get_mut(cursor) else {
            return;
        };
        let offset = finite_offset(offset);
        route.scroll_offset = offset;
        self.scroll.insert(route.path.clone(), offset);
    }
}

/// Measured document section supplied by the host layout adapter.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollSection {
    /// Stable heading anchor.
    pub id: String,
    /// Section start in document coordinates.
    pub start: f32,
    /// Section end in document coordinates.
    pub end: f32,
}

/// How a section change should affect the current history entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorHistoryUpdate {
    /// A programmatic scroll is settling; do not write history again.
    None,
    /// User scrolling changed the visible section; replace the current anchor.
    Replace,
}

/// Observable active-section change returned to the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectionChange {
    /// Newly active heading anchor.
    pub section_id: String,
    /// Explicit history policy preventing push/scroll feedback loops.
    pub history_update: AnchorHistoryUpdate,
}

/// Transactional command for an anchor jump or route scroll restoration.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollCommand {
    /// Monotonic transaction checked when the host reports completion.
    pub transaction: u64,
    /// Destination anchor, absent for an offset-only restoration.
    pub anchor: Option<String>,
    /// Target offset clamped to a finite, non-negative value.
    pub offset: f32,
    /// Whether the destination should receive keyboard focus after scrolling.
    pub focus_destination: bool,
}

/// UI-independent scrollspy and anchor synchronization state machine.
///
/// User observations may replace the current URL anchor. Programmatic scrolls
/// carry a transaction and settle with `AnchorHistoryUpdate::None`, preventing
/// the resulting viewport observation from starting another navigation.
#[derive(Clone, Debug, Default)]
pub struct ScrollSyncController {
    active_section: Option<String>,
    next_transaction: u64,
    pending: Option<(u64, Option<String>)>,
}

impl ScrollSyncController {
    /// Returns the currently resolved visible section.
    #[must_use]
    pub fn active_section(&self) -> Option<&str> {
        self.active_section.as_deref()
    }

    /// Starts an explicit anchor navigation.
    #[must_use]
    pub fn request_anchor(
        &mut self,
        anchor: impl Into<String>,
        offset: f32,
        focus_destination: bool,
    ) -> Option<ScrollCommand> {
        let anchor = anchor.into();
        if anchor.is_empty() {
            return None;
        }
        Some(self.begin(Some(anchor), offset, focus_destination))
    }

    /// Starts an offset-only restoration after back/forward or route loading.
    #[must_use]
    pub fn request_restore(&mut self, offset: f32) -> ScrollCommand {
        self.begin(None, offset, false)
    }

    /// Observes direct user scrolling and requests at most one anchor replace
    /// per effective section change.
    pub fn observe_user_scroll(
        &mut self,
        offset: f32,
        viewport_extent: f32,
        sections: &[ScrollSection],
    ) -> Option<SectionChange> {
        self.pending = None;
        self.update_active(
            visible_section(offset, viewport_extent, sections)?,
            AnchorHistoryUpdate::Replace,
        )
    }

    /// Settles a host scroll command. Stale transaction completions are
    /// ignored, and successful settlement never writes history again.
    pub fn complete_programmatic_scroll(
        &mut self,
        transaction: u64,
        offset: f32,
        viewport_extent: f32,
        sections: &[ScrollSection],
    ) -> Option<SectionChange> {
        let (pending_transaction, requested_anchor) = self.pending.as_ref()?;
        if *pending_transaction != transaction {
            return None;
        }
        let requested_anchor = requested_anchor.clone();
        self.pending = None;
        let section = requested_anchor
            .filter(|anchor| sections.iter().any(|section| section.id == *anchor))
            .or_else(|| visible_section(offset, viewport_extent, sections))?;
        self.update_active(section, AnchorHistoryUpdate::None)
    }

    fn begin(
        &mut self,
        anchor: Option<String>,
        offset: f32,
        focus_destination: bool,
    ) -> ScrollCommand {
        self.next_transaction = self.next_transaction.wrapping_add(1).max(1);
        let transaction = self.next_transaction;
        self.pending = Some((transaction, anchor.clone()));
        ScrollCommand {
            transaction,
            anchor,
            offset: finite_offset(offset),
            focus_destination,
        }
    }

    fn update_active(
        &mut self,
        section_id: String,
        history_update: AnchorHistoryUpdate,
    ) -> Option<SectionChange> {
        if self.active_section.as_deref() == Some(section_id.as_str()) {
            return None;
        }
        self.active_section = Some(section_id.clone());
        Some(SectionChange {
            section_id,
            history_update,
        })
    }
}

fn finite_offset(offset: f32) -> f32 {
    if offset.is_finite() {
        offset.max(0.0)
    } else {
        0.0
    }
}

fn visible_section(
    offset: f32,
    viewport_extent: f32,
    sections: &[ScrollSection],
) -> Option<String> {
    let probe = finite_offset(offset) + finite_offset(viewport_extent) * 0.25;
    let mut valid = sections
        .iter()
        .filter(|section| {
            !section.id.is_empty()
                && section.start.is_finite()
                && section.end.is_finite()
                && section.end >= section.start
        })
        .collect::<Vec<_>>();
    valid.sort_by(|left, right| left.start.total_cmp(&right.start));
    valid
        .iter()
        .rev()
        .copied()
        .find(|section| section.start <= probe)
        .or_else(|| valid.first().copied())
        .map(|section| section.id.clone())
}

/// Searchable document supplied by an application or fixture adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct SearchDocument {
    pub route: String,
    pub title: String,
    pub section: String,
    pub body: String,
}

/// Deterministic in-memory full-text index.
#[derive(Clone, Debug, Default)]
pub struct SearchIndex {
    documents: Vec<SearchDocument>,
    terms: HashMap<String, Vec<usize>>,
}

/// Ranked search result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct SearchResult {
    pub route: String,
    pub title: String,
    pub section: String,
    pub excerpt: String,
    pub score: usize,
}

impl SearchIndex {
    /// Rebuilds the index explicitly; no background I/O is performed.
    #[must_use]
    pub fn build(documents: Vec<SearchDocument>) -> Self {
        let mut terms: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, document) in documents.iter().enumerate() {
            for term in tokenize(&format!(
                "{} {} {}",
                document.title, document.section, document.body
            )) {
                // `tokenize` deduplicates terms within one document. Appending
                // therefore keeps each posting list unique and in document
                // order without the logarithmic cost of a tree set.
                terms.entry(term).or_default().push(index);
            }
        }
        Self { documents, terms }
    }
    /// Searches all terms with stable score and route ordering.
    #[must_use]
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        let query_terms = tokenize(query);
        if query_terms.is_empty() {
            return Vec::new();
        }
        let mut scores = BTreeMap::<usize, usize>::new();
        for term in query_terms {
            if let Some(matches) = self.terms.get(&term) {
                for index in matches {
                    *scores.entry(*index).or_default() += 1;
                }
            }
        }
        let mut results = scores
            .into_iter()
            .map(|(index, score)| {
                let document = &self.documents[index];
                SearchResult {
                    route: document.route.clone(),
                    title: document.title.clone(),
                    section: document.section.clone(),
                    excerpt: excerpt(&document.body, query),
                    score,
                }
            })
            .collect::<Vec<_>>();
        results.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.route.cmp(&right.route))
        });
        results.truncate(limit);
        results
    }
}

fn tokenize(value: &str) -> BTreeSet<String> {
    value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|word| word.len() > 1)
        .map(str::to_lowercase)
        .collect()
}
fn excerpt(body: &str, query: &str) -> String {
    let query = query.to_lowercase();
    let lower = body.to_lowercase();
    let start = lower.find(&query).unwrap_or(0).saturating_sub(32);
    body.get(start..)
        .unwrap_or(body)
        .chars()
        .take(128)
        .collect()
}

/// Searchable command palette item. Execution remains a host concern.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub keywords: String,
}

/// Deterministic command palette matcher.
#[must_use]
pub fn match_commands(items: &[CommandItem], query: &str, limit: usize) -> Vec<CommandItem> {
    let words = tokenize(query);
    let mut matches = items
        .iter()
        .filter(|item| {
            let haystack = tokenize(&format!("{} {} {}", item.label, item.group, item.keywords));
            words.iter().all(|word| haystack.contains(word))
        })
        .cloned()
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        left.group
            .cmp(&right.group)
            .then_with(|| left.label.cmp(&right.label))
    });
    matches.truncate(limit);
    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn markdown_is_semantic_and_rejects_html() {
        let document = MarkdownParser
            .parse(
                DocumentMetadata::default(),
                "# Hello world\n\nA **safe** [link](/guide).\n\n```rust\nfn main() {}\n```",
            )
            .unwrap();
        assert!(
            matches!(&document.blocks[0], DocumentBlock::Heading { anchor, .. } if anchor == "hello-world")
        );
        assert!(
            document
                .presentation_blocks()
                .iter()
                .any(|block| block.kind == "code")
        );
        assert_eq!(
            MarkdownParser.parse(DocumentMetadata::default(), "<script>x</script>"),
            Err(DocumentParseError::UnsafeHtml)
        );
    }

    #[test]
    fn anchors_are_unique_and_deterministic() {
        let document = MarkdownParser
            .parse(DocumentMetadata::default(), "## API\n## API")
            .unwrap();
        assert!(
            matches!(&document.blocks[0], DocumentBlock::Heading { anchor, .. } if anchor == "api")
        );
        assert!(
            matches!(&document.blocks[1], DocumentBlock::Heading { anchor, .. } if anchor == "api-2")
        );
    }

    #[test]
    fn localized_anchors_and_search_preserve_unicode() {
        assert_eq!(slug("Health status"), "health-status");
        assert_eq!(slug("مخاطر الإصدار"), "مخاطر-الإصدار");
        assert_eq!(slug("品質保証"), "品質保証");
        let index = SearchIndex::build(vec![SearchDocument {
            route: "/qualite".into(),
            title: "Quality and accessibility".into(),
            section: "Status".into(),
            body: "واجهة موثوقة 品質保証".into(),
        }]);
        assert_eq!(index.search("品質保証", 5)[0].route, "/qualite");
        assert_eq!(index.search("موثوقة", 5)[0].route, "/qualite");
    }

    #[test]
    fn markdown_tables_images_and_admonitions_are_data_only() {
        let source = "| Name | State |\n| --- | --- |\n| Grid | Stable |\n\n![Diagram](asset.svg \"Architecture\")\n\n> [!WARNING]\n> Review before upgrading.";
        let document = MarkdownParser
            .parse(DocumentMetadata::default(), source)
            .unwrap();
        assert!(document.blocks.iter().any(
            |block| matches!(block, DocumentBlock::Table { headers, rows } if headers.len() == 2 && rows.len() == 1)
        ), "blocks: {:?}", document.blocks);
        assert!(document.blocks.iter().any(|block| matches!(block, DocumentBlock::Image { source, alternative, .. } if source == "asset.svg" && alternative == "Diagram")));
        assert!(document.blocks.iter().any(|block| matches!(
            block,
            DocumentBlock::Admonition {
                kind: AdmonitionKind::Warning,
                ..
            }
        )));
    }

    #[test]
    fn inline_presentation_preserves_nested_semantics_and_destinations() {
        let content = vec![
            InlineContent::Text("Read ".into()),
            InlineContent::Strong(vec![InlineContent::Emphasis(vec![
                InlineContent::Text("carefully ".into()),
                InlineContent::Code("cargo test".into()),
            ])]),
            InlineContent::Link {
                label: vec![InlineContent::Text("before release".into())],
                destination: "/release#checks".into(),
            },
        ];
        let spans = presentation_inlines(&content);
        assert_eq!(
            spans
                .iter()
                .map(|span| span.text.as_str())
                .collect::<String>(),
            "Read carefully cargo testbefore release"
        );
        assert!(spans.iter().any(|span| span.strong && span.emphasis));
        assert!(
            spans
                .iter()
                .any(|span| span.code && span.strong && span.emphasis)
        );
        assert!(
            spans
                .iter()
                .filter(|span| !span.destination.is_empty())
                .all(|span| span.destination == "/release#checks")
        );
    }

    #[test]
    fn list_presentation_is_numbered_accessible_and_keeps_inline_styles() {
        let ordered = DocumentBlock::List {
            ordered: true,
            items: vec![
                vec![InlineContent::Text("Install".into())],
                vec![InlineContent::Strong(vec![InlineContent::Text(
                    "Verify output".into(),
                )])],
            ],
        };
        let items = presentation_list_items(&ordered);
        assert_eq!(items[0].marker, "1.");
        assert_eq!(items[1].marker, "2.");
        assert_eq!(items[1].accessible_text, "Verify output");
        assert!(items[1].content.iter().all(|span| span.strong));

        let unordered = DocumentBlock::List {
            ordered: false,
            items: vec![vec![InlineContent::Text("Token".into())]],
        };
        assert_eq!(presentation_list_items(&unordered)[0].marker, "•");
        assert!(presentation_list_items(&DocumentBlock::Paragraph(Vec::new())).is_empty());
    }

    #[test]
    fn reference_registry_numbers_each_kind_in_document_order() {
        let mut registry = DocumentReferenceRegistry::default();
        let first_figure = registry
            .register(
                DocumentReferenceKind::Figure,
                "architecture",
                "Layer architecture",
            )
            .expect("figure");
        let table = registry
            .register(DocumentReferenceKind::Table, "", "Compatibility matrix")
            .expect("table");
        let second_figure = registry
            .register(DocumentReferenceKind::Figure, "", "Responsive states")
            .expect("second figure");
        assert_eq!(first_figure.number, 1);
        assert_eq!(table.number, 1);
        assert_eq!(table.id, "table-compatibility-matrix");
        assert_eq!(second_figure.number, 2);
        assert_eq!(registry.targets().count(), 3);
    }

    #[test]
    fn generated_reference_ids_are_stable_and_explicit_duplicates_fail() {
        let mut registry = DocumentReferenceRegistry::default();
        let first = registry
            .register(DocumentReferenceKind::Note, "", "Keyboard behavior")
            .expect("first note");
        let second = registry
            .register(DocumentReferenceKind::Note, "", "Keyboard behavior")
            .expect("generated collision receives suffix");
        assert_eq!(first.id, "note-keyboard-behavior");
        assert_eq!(second.id, "note-keyboard-behavior-2");
        assert_eq!(
            registry.register(DocumentReferenceKind::Figure, &first.id, "Duplicate"),
            Err(DocumentReferenceError::DuplicateId(first.id))
        );
        assert_eq!(
            registry.register(DocumentReferenceKind::Figure, "", "  "),
            Err(DocumentReferenceError::EmptyCaption)
        );
    }

    #[test]
    fn cross_reference_navigation_is_resolved_without_routing_side_effects() {
        let mut registry = DocumentReferenceRegistry::default();
        let target = registry
            .register(
                DocumentReferenceKind::Table,
                "platform-support",
                "Platform support",
            )
            .expect("table");
        let request = registry
            .navigation_request(&target.id, true)
            .expect("resolved cross reference");
        assert_eq!(request.target_id, "platform-support");
        assert_eq!(request.kind, DocumentReferenceKind::Table);
        assert_eq!(request.number, 1);
        assert!(request.focus_destination);
        assert_eq!(
            registry.navigation_request("missing", false),
            Err(DocumentReferenceError::UnknownId("missing".into()))
        );
    }

    #[test]
    fn one_footnote_supports_multiple_stable_callers_and_citation_metadata() {
        let mut registry = FootnoteRegistry::default();
        let note = registry
            .declare(
                "slint-accessibility",
                "Platform accessibility depends on the selected backend.",
                Some(CitationMetadata {
                    author: "Slint Team".into(),
                    title: "Accessibility".into(),
                    source: "Slint documentation".into(),
                    date: "2026".into(),
                    destination: "https://docs.slint.dev".into(),
                }),
            )
            .expect("note");
        let first = registry
            .register_call(&note.id, "call-overview")
            .expect("first caller");
        let second = registry
            .register_call(&note.id, "call-summary")
            .expect("second caller");
        assert_eq!(first.number, second.number);
        let stored = registry.notes().next().expect("stored note");
        assert_eq!(stored.callers, ["call-overview", "call-summary"]);
        assert_eq!(
            stored.citation.as_ref().map(|item| item.author.as_str()),
            Some("Slint Team")
        );
        assert_eq!(
            registry.register_call(&note.id, "call-summary"),
            Err(FootnoteError::DuplicateCaller("call-summary".into()))
        );
    }

    #[test]
    fn footnote_return_is_single_use_and_tracks_the_latest_caller() {
        let call = FootnoteCall {
            note_id: "note-one".into(),
            caller_id: "call-two".into(),
            number: 1,
        };
        let mut controller = FootnoteNavigationController::default();
        let open = controller.open_note(&call);
        assert_eq!(open.target_id, "note-one");
        assert_eq!(open.direction, FootnoteNavigationDirection::ToNote);
        assert!(controller.return_to_caller("another-note").is_none());
        let back = controller
            .return_to_caller("note-one")
            .expect("return to active caller");
        assert_eq!(back.target_id, "call-two");
        assert_eq!(back.direction, FootnoteNavigationDirection::ReturnToCaller);
        assert!(controller.return_to_caller("note-one").is_none());
    }

    #[test]
    fn text_selection_uses_character_indices_for_unicode_sources() {
        let mut controller = TextSelectionController::new("A🙂واجهة品質Z");
        assert_eq!(
            controller.select(5, 1),
            Some(TextSelectionRange { start: 1, end: 5 })
        );
        let request = controller
            .copy_request(TextCopyScope::Selection)
            .expect("unicode selection");
        assert_eq!(request.text, "🙂واج");
        assert_eq!(request.scope, TextCopyScope::Selection);
        assert_eq!(
            controller.select(200, 3),
            Some(TextSelectionRange { start: 3, end: 10 })
        );
    }

    #[test]
    fn empty_selection_never_falls_back_to_copy_all() {
        let mut controller = TextSelectionController::new("complete source");
        assert_eq!(
            controller.copy_request(TextCopyScope::Selection),
            Err(TextSelectionError::EmptySelection)
        );
        controller.select(2, 2);
        assert_eq!(
            controller.copy_request(TextCopyScope::Selection),
            Err(TextSelectionError::EmptySelection)
        );
        assert_eq!(
            controller
                .copy_request(TextCopyScope::All)
                .expect("explicit copy all")
                .text,
            "complete source"
        );
    }

    #[test]
    fn navigation_restores_scroll_and_truncates_forward_history() {
        let mut history = NavigationHistory::default();
        history.navigate(DocumentRoute {
            path: "/a".into(),
            anchor: None,
            scroll_offset: 0.0,
        });
        history.navigate(DocumentRoute {
            path: "/b".into(),
            anchor: Some("api".into()),
            scroll_offset: 0.0,
        });
        history.remember_scroll("/a", 128.0);
        assert_eq!(history.back().map(|route| route.path.as_str()), Some("/a"));
        assert!((history.restored_scroll("/a") - 128.0).abs() < f32::EPSILON);
        history.navigate(DocumentRoute {
            path: "/c".into(),
            anchor: None,
            scroll_offset: 0.0,
        });
        assert!(history.forward().is_none());
    }

    #[test]
    fn destinations_are_classified_without_executing_them() {
        assert_eq!(
            classify_destination("/guides/tokens#color")
                .expect("internal route")
                .kind,
            DestinationKind::Internal
        );
        let web = classify_destination("HTTPS://Docs.Example.test:443/guide")
            .expect("valid web destination");
        assert_eq!(web.kind, DestinationKind::Web);
        assert_eq!(web.host.as_deref(), Some("docs.example.test"));
        assert_eq!(
            classify_destination("mailto:team@example.test")
                .expect("valid email")
                .kind,
            DestinationKind::Email
        );
    }

    #[test]
    fn unsafe_or_ambiguous_destinations_are_denied_during_validation() {
        assert_eq!(
            classify_destination("javascript:alert(1)"),
            Err(DestinationValidationError::UnsupportedScheme)
        );
        assert_eq!(
            classify_destination("//example.test/path"),
            Err(DestinationValidationError::AmbiguousNetworkPath)
        );
        assert_eq!(
            classify_destination("https://user@example.test/private"),
            Err(DestinationValidationError::InvalidExternalDestination)
        );
        assert_eq!(
            classify_destination("https://example.test/a path"),
            Err(DestinationValidationError::UnsafeCharacters)
        );
    }

    #[test]
    fn external_destinations_require_confirmation_by_default() {
        let policy = DestinationPolicy::default();
        assert_eq!(
            policy.decide("/internal").action,
            DestinationAction::NavigateInternal
        );
        assert_eq!(
            policy.decide("https://example.test").action,
            DestinationAction::ConfirmExternal
        );
        assert_eq!(
            policy.decide("mailto:team@example.test").action,
            DestinationAction::ConfirmExternal
        );
        let invalid = policy.decide("data:text/plain,unsafe");
        assert_eq!(invalid.action, DestinationAction::Deny);
        assert!(matches!(
            invalid.reason,
            DestinationDecisionReason::Invalid(DestinationValidationError::UnsupportedScheme)
        ));
    }

    #[test]
    fn exact_host_overrides_are_deterministic_and_deny_wins() {
        let policy = DestinationPolicy::default()
            .with_web_policy(ExternalDestinationPolicy::Deny)
            .with_allowed_web_host("docs.example.test")
            .with_denied_web_host("blocked.example.test")
            .with_allowed_web_host("blocked.example.test")
            .with_email_policy(ExternalDestinationPolicy::Allow);
        assert_eq!(
            policy.decide("https://docs.example.test/guide").action,
            DestinationAction::OpenExternal
        );
        assert_eq!(
            policy.decide("https://sub.docs.example.test/guide").action,
            DestinationAction::Deny,
            "host matching is exact rather than implicit wildcard matching"
        );
        assert_eq!(
            policy.decide("https://blocked.example.test").action,
            DestinationAction::Deny
        );
        assert_eq!(
            policy.decide("mailto:team@example.test").action,
            DestinationAction::OpenExternal
        );
    }

    fn load_key(route: &str) -> DocumentLoadKey {
        DocumentLoadKey {
            route: route.into(),
            language: "en".into(),
            revision: "v1".into(),
        }
    }

    fn requested(start: DocumentLoadStart) -> DocumentLoadRequest {
        match start {
            DocumentLoadStart::Request(request) => request,
            DocumentLoadStart::Cached(_) => panic!("expected a source request"),
        }
    }

    #[test]
    fn lazy_loading_rejects_obsolete_completions() {
        let mut controller = DocumentLoadController::new(DocumentCacheBudget::default());
        let old = requested(controller.load(load_key("/old")));
        let current = requested(controller.load(load_key("/current")));
        assert_eq!(
            controller.complete(
                old.request_id,
                Ok(LoadedDocument::new(old.key, "obsolete source"))
            ),
            DocumentLoadCompletion::Stale
        );
        assert_eq!(
            controller.complete(
                current.request_id,
                Ok(LoadedDocument::new(current.key.clone(), "current source"))
            ),
            DocumentLoadCompletion::Applied
        );
        assert!(matches!(
            controller.state(),
            DocumentLoadingState::Ready {
                document,
                from_cache: false
            } if document.key == current.key
        ));
    }

    #[test]
    fn cancellation_and_key_mismatch_never_replace_current_state() {
        let mut controller = DocumentLoadController::new(DocumentCacheBudget::default());
        let cancelled = requested(controller.load(load_key("/cancelled")));
        controller.cancel();
        assert_eq!(
            controller.complete(
                cancelled.request_id,
                Ok(LoadedDocument::new(cancelled.key, "late"))
            ),
            DocumentLoadCompletion::Stale
        );
        assert_eq!(controller.state(), &DocumentLoadingState::Idle);

        let active = requested(controller.load(load_key("/expected")));
        assert_eq!(
            controller.complete(
                active.request_id,
                Ok(LoadedDocument::new(load_key("/unexpected"), "wrong"))
            ),
            DocumentLoadCompletion::MismatchedKey
        );
        assert_eq!(controller.state(), &DocumentLoadingState::Loading(active));
    }

    #[test]
    fn cache_is_lru_bounded_and_reuses_the_same_source_allocation() {
        let budget = DocumentCacheBudget {
            max_entries: 2,
            max_cache_bytes: 9,
            max_document_bytes: 8,
        };
        let mut controller = DocumentLoadController::new(budget);
        for (route, source) in [("/a", "aaaa"), ("/b", "bbbb")] {
            let request = requested(controller.load(load_key(route)));
            assert_eq!(
                controller.complete(
                    request.request_id,
                    Ok(LoadedDocument::new(request.key, source))
                ),
                DocumentLoadCompletion::Applied
            );
        }
        let cached = match controller.load(load_key("/a")) {
            DocumentLoadStart::Cached(document) => document,
            DocumentLoadStart::Request(_) => panic!("expected cache hit"),
        };
        let DocumentLoadingState::Ready {
            document: ready, ..
        } = controller.state()
        else {
            panic!("expected ready state");
        };
        assert!(Arc::ptr_eq(&cached.source, &ready.source));

        let request = requested(controller.load(load_key("/c")));
        controller.complete(
            request.request_id,
            Ok(LoadedDocument::new(request.key, "cccc")),
        );
        assert_eq!(controller.cache_usage(), (2, 8));
        assert!(matches!(
            controller.load(load_key("/b")),
            DocumentLoadStart::Request(_)
        ));
    }

    #[test]
    fn oversized_documents_become_explicit_non_retryable_errors() {
        let mut controller = DocumentLoadController::new(DocumentCacheBudget {
            max_entries: 1,
            max_cache_bytes: 8,
            max_document_bytes: 4,
        });
        let request = requested(controller.load(load_key("/large")));
        assert_eq!(
            controller.complete(
                request.request_id,
                Ok(LoadedDocument::new(request.key.clone(), "12345"))
            ),
            DocumentLoadCompletion::BudgetExceeded
        );
        assert!(matches!(
            controller.state(),
            DocumentLoadingState::Error { key, error }
                if key == &request.key
                    && error.kind == DocumentLoadErrorKind::BudgetExceeded
                    && !error.retryable
        ));
        assert_eq!(controller.cache_usage(), (0, 0));
    }

    fn scroll_sections() -> Vec<ScrollSection> {
        vec![
            ScrollSection {
                id: "overview".into(),
                start: 0.0,
                end: 399.0,
            },
            ScrollSection {
                id: "recipes".into(),
                start: 400.0,
                end: 899.0,
            },
            ScrollSection {
                id: "accessibility".into(),
                start: 900.0,
                end: 1_400.0,
            },
        ]
    }

    #[test]
    fn user_scroll_replaces_anchor_only_when_the_section_changes() {
        let mut controller = ScrollSyncController::default();
        let sections = scroll_sections();
        let first = controller
            .observe_user_scroll(0.0, 400.0, &sections)
            .expect("initial visible section");
        assert_eq!(first.section_id, "overview");
        assert_eq!(first.history_update, AnchorHistoryUpdate::Replace);
        assert!(
            controller
                .observe_user_scroll(120.0, 400.0, &sections)
                .is_none()
        );

        let next = controller
            .observe_user_scroll(350.0, 400.0, &sections)
            .expect("next visible section");
        assert_eq!(next.section_id, "recipes");
        assert_eq!(controller.active_section(), Some("recipes"));
    }

    #[test]
    fn programmatic_anchor_settlement_does_not_loop_into_history() {
        let mut controller = ScrollSyncController::default();
        let sections = scroll_sections();
        let stale = controller
            .request_anchor("overview", 0.0, true)
            .expect("valid anchor");
        let command = controller
            .request_anchor("accessibility", 900.0, true)
            .expect("valid anchor");
        assert!(
            controller
                .complete_programmatic_scroll(stale.transaction, 0.0, 400.0, &sections)
                .is_none()
        );

        let settled = controller
            .complete_programmatic_scroll(command.transaction, 900.0, 400.0, &sections)
            .expect("current transaction settles");
        assert_eq!(settled.section_id, "accessibility");
        assert_eq!(settled.history_update, AnchorHistoryUpdate::None);
        assert!(
            controller
                .observe_user_scroll(900.0, 400.0, &sections)
                .is_none()
        );
    }

    #[test]
    fn history_replaces_anchor_and_restores_exact_scroll_without_push() {
        let mut history = NavigationHistory::default();
        history.navigate(DocumentRoute {
            path: "/guide".into(),
            anchor: None,
            scroll_offset: 0.0,
        });
        history.replace_current_anchor(Some("recipes".into()));
        history.remember_current_scroll(512.0);
        let current = history.current().expect("current route");
        assert_eq!(current.anchor.as_deref(), Some("recipes"));
        assert!((current.scroll_offset - 512.0).abs() < f32::EPSILON);
        assert!((history.restored_scroll("/guide") - 512.0).abs() < f32::EPSILON);
        assert!(history.back().is_none(), "anchor replacement must not push");
    }

    #[test]
    fn search_and_commands_are_stable_and_bounded() {
        let index = SearchIndex::build(vec![
            SearchDocument {
                route: "/tokens".into(),
                title: "Tokens".into(),
                section: "Color".into(),
                body: "Semantic colors and themes".into(),
            },
            SearchDocument {
                route: "/layout".into(),
                title: "Layout".into(),
                section: "Grid".into(),
                body: "Responsive grid recipes".into(),
            },
        ]);
        assert_eq!(index.search("semantic colors", 5)[0].route, "/tokens");
        let commands = vec![CommandItem {
            id: "theme".into(),
            label: "Change theme".into(),
            group: "Appearance".into(),
            keywords: "dark light".into(),
        }];
        assert_eq!(match_commands(&commands, "dark", 5)[0].id, "theme");
    }

    #[test]
    fn ten_thousand_search_documents_stay_within_debug_budget() {
        use atlas_ui_testing::{PerformanceBudget, measure_performance};

        let documents: Vec<_> = (0..10_000)
            .map(|index| SearchDocument {
                route: format!("/reference/{index}"),
                title: format!("Component {index}"),
                section: "API reference".into(),
                body: "Responsive semantic tokens accessibility keyboard navigation".into(),
            })
            .collect();
        let summary = measure_performance(
            PerformanceBudget {
                warmup_iterations: 1,
                sample_count: 7,
                median_limit: Duration::from_millis(500),
            },
            || documents.clone(),
            |sample| {
                let index = SearchIndex::build(sample);
                index.search("accessibility keyboard", 20)
            },
        );
        eprintln!("atlas-performance search-index-10000 {summary:?}");
        assert!(
            summary.is_within_budget(),
            "search performance: {summary:?}"
        );
        let results = SearchIndex::build(documents).search("accessibility keyboard", 20);
        assert_eq!(results.len(), 20);
    }
}
