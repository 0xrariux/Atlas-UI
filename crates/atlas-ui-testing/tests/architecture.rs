//! Workspace dependency and public-contract checks.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("testing crate remains under crates/")
}

fn ttf_u16_at(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn ttf_i16_at(bytes: &[u8], offset: usize) -> i16 {
    i16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn ttf_table_offset(bytes: &[u8], tag: [u8; 4]) -> usize {
    let table_count = usize::from(ttf_u16_at(bytes, 4));
    (0..table_count)
        .find_map(|index| {
            let record = 12 + index * 16;
            (bytes[record..record + 4] == tag).then(|| {
                u32::from_be_bytes([
                    bytes[record + 8],
                    bytes[record + 9],
                    bytes[record + 10],
                    bytes[record + 11],
                ]) as usize
            })
        })
        .expect("registered font table")
}

fn ttf_line_height_ratios(bytes: &[u8]) -> (f32, f32) {
    let head = ttf_table_offset(bytes, *b"head");
    let hhea = ttf_table_offset(bytes, *b"hhea");
    let os2 = ttf_table_offset(bytes, *b"OS/2");
    let units_per_em = f32::from(ttf_u16_at(bytes, head + 18));
    let hhea_ratio = f32::from(
        ttf_i16_at(bytes, hhea + 4) - ttf_i16_at(bytes, hhea + 6) + ttf_i16_at(bytes, hhea + 8),
    ) / units_per_em;
    let typo_ratio = f32::from(
        ttf_i16_at(bytes, os2 + 68) - ttf_i16_at(bytes, os2 + 70) + ttf_i16_at(bytes, os2 + 72),
    ) / units_per_em;
    (hhea_ratio, typo_ratio)
}

#[test]
fn public_slint_facades_exist() {
    let root = workspace_root();
    for relative in [
        "crates/atlas-ui-tokens/ui/tokens.slint",
        "crates/atlas-ui-core/ui/core.slint",
        "crates/atlas-ui-icons/ui/icons.slint",
        "crates/atlas-ui-components/ui/stable.slint",
        "crates/atlas-ui-components/ui/preview-nonresponsive.slint",
        "crates/atlas-ui-components/ui/preview.slint",
        "crates/atlas-ui-components/ui/components.slint",
    ] {
        assert!(root.join(relative).is_file(), "missing facade: {relative}");
    }
}

fn collect_slint_imports(root: &Path, path: &Path, visited: &mut BTreeSet<PathBuf>) {
    if !visited.insert(path.to_owned()) {
        return;
    }
    let source = fs::read_to_string(path).expect("public Slint dependency");
    assert!(
        !source.contains("FlexboxLayout")
            && !source.contains("responsive-layout.slint")
            && !source.contains("@atlas-ui-core/core.slint"),
        "{} loads an experimental responsive contract",
        path.strip_prefix(root).unwrap_or(path).display()
    );

    let imports = source.split('"').skip(1).step_by(2).filter(|candidate| {
        Path::new(candidate)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("slint"))
    });
    for import in imports {
        let dependency = if let Some(relative) = import.strip_prefix("@atlas-ui-core/") {
            root.join("crates/atlas-ui-core/ui").join(relative)
        } else if let Some(relative) = import.strip_prefix("@atlas-ui-icons/") {
            root.join("crates/atlas-ui-icons/ui").join(relative)
        } else if let Some(relative) = import.strip_prefix("@atlas-ui-tokens/") {
            root.join("crates/atlas-ui-tokens/ui").join(relative)
        } else if import == "std-widgets.slint" {
            continue;
        } else {
            path.parent().expect("Slint source parent").join(import)
        };
        collect_slint_imports(root, &dependency, visited);
    }
}

#[test]
fn nonresponsive_preview_facade_has_no_experimental_transitive_dependency() {
    let root = workspace_root();
    let facade = root.join("crates/atlas-ui-components/ui/preview-nonresponsive.slint");
    let source = fs::read_to_string(&facade).expect("non-responsive preview facade");
    for contract in [
        "AtlasProgressBar",
        "AtlasSpinner",
        "AtlasTab",
        "AtlasTabPanel",
    ] {
        assert!(
            source.contains(contract),
            "missing non-responsive preview contract: {contract}"
        );
    }
    collect_slint_imports(root, &facade, &mut BTreeSet::new());
}

#[test]
fn icon_system_uses_registered_vectors_instead_of_font_glyphs() {
    let root = workspace_root();
    let source =
        fs::read_to_string(root.join("crates/atlas-ui-icons/ui/icons.slint")).expect("icon facade");
    for contract in [
        "Image",
        "colorize",
        "@image-url",
        "IconName.chevron-down",
        "IconTone",
        "IconSize",
        "decorative",
        "accessible-name",
        "accessibility-valid",
        "AtlasTheme.success",
        "AtlasTheme.warning",
        "AtlasTheme.danger",
    ] {
        assert!(
            source.contains(contract),
            "missing vector icon contract: {contract}"
        );
    }
    assert!(
        !source.contains("glyph:"),
        "font glyph fallback is forbidden"
    );
    assert!(
        root.join("crates/atlas-ui-icons/icons.registry.json")
            .is_file()
    );
    assert!(
        root.join("crates/atlas-ui-icons/ASSET_LICENSE.md")
            .is_file()
    );
}

#[test]
fn token_layers_and_component_template_exist() {
    let root = workspace_root();
    for relative in [
        "crates/atlas-ui-tokens/ui/palette.slint",
        "crates/atlas-ui-tokens/ui/settings.slint",
        "crates/atlas-ui-tokens/ui/semantic.slint",
        "crates/atlas-ui-tokens/ui/geometry.slint",
        "crates/atlas-ui-tokens/ui/density.slint",
        "crates/atlas-ui-tokens/ui/typography.slint",
        "crates/atlas-ui-tokens/ui/motion.slint",
        "crates/atlas-ui-tokens/ui/component-tokens.slint",
        "crates/atlas-ui-core/ui/component-frame.slint",
        "crates/atlas-ui-core/ui/interaction.slint",
        "crates/atlas-ui-core/templates/component.slint.template",
    ] {
        assert!(
            root.join(relative).is_file(),
            "missing foundation: {relative}"
        );
    }
}

#[test]
fn typography_embeds_versioned_sans_and_mono_fonts() {
    let root = workspace_root();
    let source = fs::read_to_string(root.join("crates/atlas-ui-tokens/ui/typography.slint"))
        .expect("typography tokens");
    for contract in [
        "Inter-Variable.ttf",
        "AtlasText-Variable.ttf",
        "JetBrainsMono-Variable.ttf",
        "font-sans: \"Atlas\"",
        "font-display: \"Inter\"",
        "font-mono: \"JetBrains Mono\"",
        "TypographyScale",
        "scale-factor",
        "block-gap-normal",
        "mono-size-inline",
        "underline-thickness",
    ] {
        assert!(
            source.contains(contract),
            "missing embedded font contract: {contract}"
        );
    }
    let editorial = fs::read_to_string(root.join("crates/atlas-ui-components/ui/editorial.slint"))
        .expect("editorial components");
    for contract in [
        "AtlasStyledText",
        "EditorialEmphasis",
        "underlined",
        "AtlasTypography.underline-offset",
        "AtlasTypography.mono-size-inline",
    ] {
        assert!(
            editorial.contains(contract),
            "missing editorial token consumer: {contract}"
        );
    }
    for relative in [
        "crates/atlas-ui-tokens/fonts.registry.json",
        "crates/atlas-ui-tokens/assets/fonts/inter/OFL.txt",
        "crates/atlas-ui-tokens/assets/fonts/jetbrains-mono/OFL.txt",
    ] {
        assert!(
            root.join(relative).is_file(),
            "missing font provenance: {relative}"
        );
    }

    for relative in [
        "crates/atlas-ui-tokens/assets/fonts/inter/AtlasText-Variable.ttf",
        "crates/atlas-ui-tokens/assets/fonts/jetbrains-mono/JetBrainsMono-Variable.ttf",
    ] {
        let bytes = fs::read(root.join(relative)).expect("registered font asset");
        let (hhea_ratio, typo_ratio) = ttf_line_height_ratios(&bytes);
        assert!(
            (hhea_ratio - 1.6).abs() < 0.001,
            "tight hhea leading: {relative}"
        );
        assert!(
            (typo_ratio - 1.6).abs() < 0.001,
            "tight typo leading: {relative}"
        );
    }
}

#[test]
fn interaction_contract_covers_input_accessibility_and_group_navigation() {
    let source =
        fs::read_to_string(workspace_root().join("crates/atlas-ui-core/ui/interaction.slint"))
            .expect("interaction primitives");
    for contract in [
        "FocusReason.tab-navigation",
        "Key.Return",
        "Key.Space",
        "public function trigger",
        "SelectionController",
        "selection-requested",
        "RovingFocusController",
        "focus-requested",
    ] {
        assert!(
            source.contains(contract),
            "missing interaction contract: {contract}"
        );
    }
}

fn slint_files(directory: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory).expect("foundation directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            files.extend(slint_files(&path));
        } else if path
            .extension()
            .is_some_and(|extension| extension == "slint")
        {
            files.push(path);
        }
    }
    files
}

#[test]
fn visual_literals_stay_in_the_token_layer() {
    let root = workspace_root();
    for relative in [
        "crates/atlas-ui-core/ui",
        "crates/atlas-ui-icons/ui",
        "crates/atlas-ui-components/ui",
    ] {
        for file in slint_files(&root.join(relative)) {
            let source = fs::read_to_string(&file).expect("Slint source");
            assert!(
                !source.contains('#'),
                "color literal outside tokens: {}",
                file.display()
            );
            for word in source.split_whitespace() {
                let candidate = word.trim_matches(|character: char| {
                    matches!(character, ';' | ':' | ',' | '(' | ')' | '?' | '+' | '*')
                });
                assert!(
                    !candidate.ends_with("px") || candidate.chars().all(char::is_alphabetic),
                    "length literal outside tokens: {} ({candidate})",
                    file.display()
                );
            }
        }
    }
}

#[test]
fn lower_layers_do_not_depend_on_component_or_gallery_packages() {
    let root = workspace_root();
    for relative in [
        "crates/atlas-ui-tokens/Cargo.toml",
        "crates/atlas-ui-core/Cargo.toml",
        "crates/atlas-ui-icons/Cargo.toml",
    ] {
        let manifest = fs::read_to_string(root.join(relative)).expect("package manifest");
        assert!(
            !manifest.contains("atlas-ui-components"),
            "reverse dependency in {relative}"
        );
        assert!(
            !manifest.contains("atlas-ui-gallery"),
            "gallery dependency in {relative}"
        );
    }
}

#[test]
fn responsive_recipes_share_tokens_and_expose_observable_breakpoints() {
    let root = workspace_root();
    let source = fs::read_to_string(root.join("crates/atlas-ui-core/ui/responsive-layout.slint"))
        .expect("responsive layout recipes");
    for contract in [
        "AtlasStack",
        "AtlasCluster",
        "AtlasSidebar",
        "collapsed",
        "AtlasSwitcher",
        "stacked",
        "AtlasAutoGrid",
        "column-count",
        "item-basis",
        "AtlasColumnGrid",
        "AtlasGridItem",
        "compact-span",
        "normal-span",
        "wide-span",
        "active-span",
        "resolved-width",
        "AtlasGrid.gutter-compact",
        "AtlasGrid.gutter-normal",
        "AtlasGrid.gutter-wide",
        "LayoutGap",
        "@children",
    ] {
        assert!(
            source.contains(contract),
            "missing responsive contract: {contract}"
        );
    }
    assert!(
        source.contains("FlexboxLayout"),
        "wrapping must use the shared flex engine"
    );
    let cargo_config = fs::read_to_string(root.join(".cargo/config.toml"))
        .expect("explicit Slint preview configuration");
    assert!(cargo_config.contains("SLINT_ENABLE_EXPERIMENTAL_FEATURES"));

    let edge = fs::read_to_string(root.join("crates/atlas-ui-core/ui/edge-surface.slint"))
        .expect("stable edge surface");
    for contract in [
        "AtlasEdgeSurface",
        "DividerEdge",
        "divider-edge",
        "divider-width",
        "border-width: AtlasGrid.zero",
    ] {
        assert!(edge.contains(contract), "missing edge contract: {contract}");
    }
    assert!(!edge.contains("FlexboxLayout"));
}

#[test]
fn stable_facade_does_not_load_experimental_responsive_layouts() {
    let root = workspace_root();
    let core_stable = fs::read_to_string(root.join("crates/atlas-ui-core/ui/stable.slint"))
        .expect("stable Core facade");
    assert!(!core_stable.contains("responsive-layout.slint"));
    assert!(!core_stable.contains("FlexboxLayout"));

    let component_root = root.join("crates/atlas-ui-components/ui");
    for file in [
        "stable.slint",
        "button.slint",
        "icon-button.slint",
        "tabs.slint",
        "metric-card.slint",
        "checkbox.slint",
        "switch.slint",
    ] {
        let source =
            fs::read_to_string(component_root.join(file)).expect("stable component source");
        assert!(
            !source.contains("@atlas-ui-core/core.slint"),
            "{file} must import the non-experimental Core facade"
        );
    }
}

#[test]
fn data_contract_covers_planned_table_capabilities() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/data-contracts.slint"),
    )
    .expect("data contracts");
    for contract in [
        "SelectionMode",
        "CellEditorKind",
        "ColumnPin",
        "resizable",
        "filterable",
        "expanded",
        "tooltip",
        "DataViewState",
    ] {
        assert!(
            source.contains(contract),
            "missing data contract: {contract}"
        );
    }
}

#[test]
fn foundational_components_share_public_contracts() {
    let root = workspace_root();
    let contracts = [
        (
            "button.slint",
            ["ActionArea", "loading", "accessible-action-default"],
        ),
        (
            "text-field.slint",
            ["TextInput", "error", "accessible-label"],
        ),
        (
            "checkbox.slint",
            ["ActionArea", "AccessibleRole.checkbox", "toggled"],
        ),
        (
            "switch.slint",
            ["ActionArea", "AccessibleRole.switch", "toggled"],
        ),
        ("badge.slint", ["BadgeTone", "accessible-label", "dot"]),
        (
            "icon-button.slint",
            ["AtlasIconButton", "accessible-label", "pointer-target-min"],
        ),
    ];
    for (file, required) in contracts {
        let source = fs::read_to_string(root.join("crates/atlas-ui-components/ui").join(file))
            .expect("foundational component");
        for contract in required {
            assert!(source.contains(contract), "missing {contract} in {file}");
        }
    }
}

#[test]
fn interaction_loading_primitives_are_accessible_and_motion_aware() {
    let root = workspace_root();
    let spinner =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/activity-indicator.slint"))
            .expect("activity indicator");
    for contract in [
        "AtlasSpinner",
        "AccessibleRole.progress-indicator",
        "accessible-label: root.label",
        "accessible-value: root.value-text",
        "MotionPreference.reduced",
        "AtlasMotion.spinner-cycle",
    ] {
        assert!(
            spinner.contains(contract),
            "missing spinner contract: {contract}"
        );
    }

    let progress =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/migration-wave.slint"))
            .expect("progress controls");
    for contract in [
        "in property <bool> indeterminate: false",
        "in property <bool> show-labels: true",
        "accessible-value: root.value-text",
        "MotionPreference.reduced",
        "AtlasMotion.indeterminate-cycle",
    ] {
        assert!(
            progress.contains(contract),
            "missing activity rail contract: {contract}"
        );
    }

    for file in [
        "button.slint",
        "icon-button.slint",
        "checkbox.slint",
        "switch.slint",
        "tabs.slint",
    ] {
        let source = fs::read_to_string(root.join("crates/atlas-ui-components/ui").join(file))
            .expect("interactive component");
        assert!(
            source.contains("AtlasMotion.fast"),
            "missing token-driven transition in {file}"
        );
    }
}

#[test]
fn editorial_foundations_expose_semantic_hierarchy_and_readable_measure() {
    let root = workspace_root();
    let source = fs::read_to_string(root.join("crates/atlas-ui-components/ui/editorial.slint"))
        .expect("editorial components");
    for contract in [
        "AtlasHeading",
        "HeadingLevel",
        "AtlasParagraph",
        "AtlasSelectableText",
        "copy-selection-requested(string)",
        "copy-all-requested(string)",
        "measure-readable",
        "AtlasInlineCode",
        "AtlasCodeBlock",
        "copy-requested",
        "AtlasBlockQuote",
        "AtlasDivider",
    ] {
        assert!(
            source.contains(contract),
            "missing editorial contract: {contract}"
        );
    }
}

#[test]
fn rich_content_components_keep_actions_controlled_and_semantic() {
    let root = workspace_root();
    let source = fs::read_to_string(root.join("crates/atlas-ui-components/ui/rich-content.slint"))
        .expect("rich content components");
    for contract in [
        "AtlasAdmonition",
        "AdmonitionTone { note, tip, info, caution, warning, danger, success }",
        "AtlasCallout",
        "action-requested(string)",
        "AtlasLink",
        "AtlasControlTokens.gap",
        "AtlasLinkCard",
        "in property <bool> selected: false",
        "action.hovered ? AtlasTheme.border-strong",
        "AtlasRichText",
        "RichTextFragment",
        "link-activated(string)",
        "AtlasDocumentList",
        "DocumentListItem",
        "AtlasCaption",
        "AtlasCrossReference",
        "CaptionKind",
        "reference-requested(string)",
        "AtlasFootnoteReference",
        "AtlasFootnoteList",
        "FootnoteItem",
        "note-requested(string, string)",
        "return-requested(string)",
        "activated(string)",
        "AtlasDocumentTable",
        "DocumentTableColumn",
        "AtlasFigure",
        "MediaState { loading, ready, empty, error }",
        "alternative-text",
        "retry-requested(string)",
        "AtlasSkeleton",
        "AtlasEmptyState",
        "AtlasErrorState",
        "AtlasTerminalBlock",
        "copy-requested(string)",
        "AtlasContentTabs",
        "selection-requested(string)",
        "@children",
    ] {
        assert!(
            source.contains(contract),
            "missing rich content contract: {contract}"
        );
    }
    assert!(
        !source.contains("https://") && !source.contains("http://"),
        "generic rich content must not perform or embed remote actions"
    );
}

#[test]
fn documentation_shell_exposes_controlled_responsive_navigation() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/documentation-shell.slint"),
    )
    .expect("documentation shell");
    for contract in [
        "AtlasDocumentationShell",
        "DocumentationNavItem",
        "BreadcrumbItem",
        "TocItem",
        "FooterLink",
        "navigation-open",
        "toc-open",
        "navigate-requested(string)",
        "section-requested(string)",
        "skip-to-content-requested",
        "back-to-top-requested",
        "navigation-focus-requested(string, int)",
        "toc-focus-requested(string, int)",
        "navigation-focus-restore-requested",
        "toc-focus-restore-requested",
        "public function focus-content()",
        "public function focus-skip-link()",
        "contained-focus: true",
        "AccessibleRole.navigation",
        "AtlasEdgeSurface",
        "DividerEdge.bottom",
        "DividerEdge.right",
        "DividerEdge.left",
        "@children",
    ] {
        assert!(
            source.contains(contract),
            "missing documentation shell contract: {contract}"
        );
    }
}

#[test]
fn documentation_keyboard_paths_are_visible_and_addressable() {
    let root = workspace_root();
    let shell =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/documentation-shell.slint"))
            .expect("documentation shell");
    for contract in [
        "preview-skip-focus",
        "preview-content-focus",
        "preview-back-to-top-focus",
        "preview-navigation-focus-id",
        "preview-toc-focus-id",
        "navigation-requested(direction)",
        "content-focus.focus()",
    ] {
        assert!(
            shell.contains(contract),
            "missing keyboard proof: {contract}"
        );
    }

    let anchors =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/document-tools.slint"))
            .expect("document tools");
    for contract in [
        "AtlasAnchorAction",
        "preview-focus-visible",
        "public function focus-from-keyboard()",
    ] {
        assert!(
            anchors.contains(contract),
            "missing anchor focus proof: {contract}"
        );
    }
}

#[test]
fn document_scroll_sync_separates_user_and_programmatic_history() {
    let source = fs::read_to_string(workspace_root().join("crates/atlas-ui-documents/src/lib.rs"))
        .expect("document infrastructure");
    for contract in [
        "ScrollSyncController",
        "ScrollSection",
        "ScrollCommand",
        "AnchorHistoryUpdate",
        "observe_user_scroll",
        "complete_programmatic_scroll",
        "replace_current_anchor",
        "remember_current_scroll",
    ] {
        assert!(
            source.contains(contract),
            "missing scroll sync contract: {contract}"
        );
    }
}

#[test]
fn destination_policy_requires_an_explicit_host_decision() {
    let source = fs::read_to_string(workspace_root().join("crates/atlas-ui-documents/src/lib.rs"))
        .expect("document infrastructure");
    for contract in [
        "DestinationKind",
        "DestinationValidationError",
        "ExternalDestinationPolicy",
        "DestinationAction",
        "DestinationDecision",
        "DestinationPolicy",
        "classify_destination",
        "ConfirmExternal",
        "with_allowed_web_host",
        "with_denied_web_host",
    ] {
        assert!(
            source.contains(contract),
            "missing destination policy contract: {contract}"
        );
    }
    for forbidden_effect in ["std::process::Command", "webbrowser::", "open::that"] {
        assert!(
            !source.contains(forbidden_effect),
            "destination policy must remain free of host effects: {forbidden_effect}"
        );
    }
}

#[test]
fn document_references_are_numbered_before_presentation_and_routing() {
    let source = fs::read_to_string(workspace_root().join("crates/atlas-ui-documents/src/lib.rs"))
        .expect("document infrastructure");
    for contract in [
        "DocumentReferenceKind",
        "DocumentReferenceTarget",
        "DocumentReferenceRegistry",
        "DocumentReferenceError",
        "ReferenceNavigationRequest",
        "navigation_request",
        "focus_destination",
        "CitationMetadata",
        "FootnoteRegistry",
        "FootnoteNavigationController",
        "ReturnToCaller",
        "return_to_caller",
    ] {
        assert!(
            source.contains(contract),
            "missing document reference contract: {contract}"
        );
    }
}

#[test]
fn text_copy_uses_unicode_ranges_and_an_explicit_clipboard_port() {
    let source = fs::read_to_string(workspace_root().join("crates/atlas-ui-documents/src/lib.rs"))
        .expect("document infrastructure");
    for contract in [
        "TextSelectionRange",
        "TextCopyScope",
        "TextCopyRequest",
        "TextSelectionController",
        "ClipboardPort",
        "EmptySelection",
        ".chars()",
    ] {
        assert!(
            source.contains(contract),
            "missing text copy contract: {contract}"
        );
    }
}

#[test]
fn lazy_document_loading_is_bounded_and_runtime_neutral() {
    let source = fs::read_to_string(workspace_root().join("crates/atlas-ui-documents/src/lib.rs"))
        .expect("document infrastructure");
    for contract in [
        "AsyncDocumentSource",
        "DocumentLoadController",
        "DocumentCacheBudget",
        "DocumentLoadingState",
        "DocumentLoadCompletion",
        "Arc<str>",
        "Stale",
        "BudgetExceeded",
        "max_cache_bytes",
        "max_document_bytes",
    ] {
        assert!(
            source.contains(contract),
            "missing lazy loading contract: {contract}"
        );
    }
    for forbidden_runtime in ["tokio::", "async_std::", "reqwest::", "ureq::"] {
        assert!(
            !source.contains(forbidden_runtime),
            "document core must not select a runtime or remote adapter: {forbidden_runtime}"
        );
    }
}

#[test]
fn document_viewport_is_tokenized_scrollable_and_route_aware() {
    let root = workspace_root();
    let viewport = fs::read_to_string(root.join("crates/atlas-ui-core/ui/scroll-viewport.slint"))
        .expect("scroll viewport");
    for contract in [
        "AtlasScrollViewport",
        "AtlasViewportTokens",
        "Flickable",
        "viewport-y",
        "Key.PageDown",
        "Key.PageUp",
        "Key.Home",
        "Key.End",
        "public function scroll-to",
        "@children",
    ] {
        assert!(
            viewport.contains(contract),
            "missing viewport contract: {contract}"
        );
    }

    let shell =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/documentation-shell.slint"))
            .expect("documentation shell");
    for contract in [
        "content-scroll-y",
        "content-scrolled(length)",
        "route-transition-focus-requested(string)",
        "public function restore-route",
    ] {
        assert!(
            shell.contains(contract),
            "missing shell viewport contract: {contract}"
        );
    }
}

#[test]
fn document_tools_expose_intentions_without_remote_effects() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/document-tools.slint"),
    )
    .expect("document tools");
    for contract in [
        "AtlasAnchorAction",
        "copy-deep-link-requested(string)",
        "AtlasDocumentSearch",
        "query-edited(string)",
        "result-requested(string)",
        "AtlasCommandPalette",
        "command-requested(string)",
        "OverlayFocusController",
    ] {
        assert!(
            source.contains(contract),
            "missing document tool contract: {contract}"
        );
    }
    assert!(!source.contains("https://") && !source.contains("http://"));
}

#[test]
fn roadmap_template_composes_shared_content_and_responsive_states() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/documentation-templates.slint"),
    )
    .expect("documentation templates");
    for contract in [
        "AtlasRoadmapContentTemplate",
        "RoadmapStatusRow",
        "reference-width",
        "AtlasDocumentTable",
        "AtlasAdmonition",
        "AtlasStepper",
        "AtlasTerminalBlock",
        "phase-requested(int)",
    ] {
        assert!(
            source.contains(contract),
            "missing roadmap template contract: {contract}"
        );
    }
}

#[test]
fn application_templates_compose_shared_controls_and_keep_actions_controlled() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/application-templates.slint"),
    )
    .expect("application templates");
    for contract in [
        "AtlasSettingsTemplate",
        "SettingsSectionItem",
        "section-requested(string)",
        "save-requested",
        "discard-requested",
        "AtlasDashboardTemplate",
        "DashboardMetricItem",
        "DashboardActivityItem",
        "refresh-requested",
        "metric-requested(string)",
        "AtlasAutoGrid",
        "AtlasMetricCard",
        "@children",
    ] {
        assert!(
            source.contains(contract),
            "missing application template contract: {contract}"
        );
    }
}

#[test]
fn intrinsic_split_and_sticky_layouts_share_bounded_controlled_contracts() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-core/ui/responsive-layout.slint"),
    )
    .expect("responsive layout primitives");
    for contract in [
        "AtlasIntrinsicFrame",
        "min-content-width",
        "max-content-width",
        "AtlasSplitView",
        "AtlasSplitPane",
        "AtlasResizeHandle",
        "resize-requested(length)",
        "keyboard-step",
        "AccessibleRole.slider",
        "AtlasStickyRegion",
        "AtlasZOrder.sticky",
    ] {
        assert!(
            source.contains(contract),
            "missing bounded layout contract: {contract}"
        );
    }
    assert!(
        source.contains("@children"),
        "layout recipes must remain composable"
    );
    assert!(!source.contains("http://") && !source.contains("https://"));
}

#[test]
fn theme_control_keeps_system_resolution_and_persistence_explicit() {
    let root = workspace_root();
    let settings = fs::read_to_string(root.join("crates/atlas-ui-tokens/ui/settings.slint"))
        .expect("theme settings");
    for contract in [
        "ThemeMode { system, dark, light }",
        "system-dark",
        "resolved-theme-mode",
    ] {
        assert!(
            settings.contains(contract),
            "missing system theme contract: {contract}"
        );
    }
    let control =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/theme-control.slint"))
            .expect("theme control");
    for contract in [
        "AtlasThemeControl",
        "preference-requested(ThemeMode)",
        "ThemeMode.system",
    ] {
        assert!(
            control.contains(contract),
            "missing theme control contract: {contract}"
        );
    }
}

#[test]
fn overlay_and_navigation_contracts_cover_focus_and_accessibility() {
    let root = workspace_root();
    let contracts = [
        (
            "tabs.slint",
            [
                "AccessibleRole.tab",
                "navigation-requested",
                "focus-from-group",
            ],
        ),
        (
            "tooltip.slint",
            ["force-open", "action.focused", "accessible-description"],
        ),
        (
            "menu.slint",
            ["OverlayFocusController", "active-index", "dismissed"],
        ),
        (
            "modal.slint",
            [
                "OverlayFocusController",
                "accessible-expanded",
                "focus-restore-requested",
            ],
        ),
    ];
    for (file, required) in contracts {
        let source = fs::read_to_string(root.join("crates/atlas-ui-components/ui").join(file))
            .expect("overlay or navigation component");
        for contract in required {
            assert!(source.contains(contract), "missing {contract} in {file}");
        }
    }
    let core = fs::read_to_string(root.join("crates/atlas-ui-core/ui/interaction.slint"))
        .expect("interaction primitives");
    for contract in [
        "OverlayFocusController",
        "Key.Escape",
        "Key.Tab",
        "Key.Delete",
        "removal-requested",
        "restore-focus-requested",
    ] {
        assert!(
            core.contains(contract),
            "missing focus boundary contract: {contract}"
        );
    }
}

#[test]
fn stable_workspace_tabs_cover_close_roving_focus_and_overflow() {
    let source =
        fs::read_to_string(workspace_root().join("crates/atlas-ui-components/ui/tabs.slint"))
            .expect("tabs");
    for contract in [
        "AtlasWorkspaceTab",
        "AtlasWorkspaceTabList",
        "AccessibleRole.tab-list",
        "AccessibleRole.tab",
        "AccessibleRole.button",
        "close-requested",
        "navigation-requested",
        "settle-after-close",
        "overflow-requested",
        "overflow: elide",
        "accessible-item-index",
        "accessible-item-count",
        "atlas-workspace-tab-",
    ] {
        assert!(
            source.contains(contract),
            "missing workspace tab contract: {contract}"
        );
    }
}

#[test]
fn categorical_tokens_are_ordinal_and_separate_from_status() {
    let source =
        fs::read_to_string(workspace_root().join("crates/atlas-ui-tokens/ui/categorical.slint"))
            .expect("categorical tokens");
    for index in 1..=6 {
        assert!(
            source.contains(&format!("category-{index}")),
            "missing category {index}"
        );
    }
    for semantic in ["success", "warning", "danger", "healthy"] {
        assert!(
            !source.contains(&format!("category-{semantic}")),
            "categorical tokens must not encode {semantic}"
        );
    }
}

#[test]
fn data_components_expose_scalable_intentions_and_states() {
    let root = workspace_root();
    let table = fs::read_to_string(root.join("crates/atlas-ui-components/ui/data-table.slint"))
        .expect("data table");
    for contract in [
        "AtlasDataList",
        "DataColumnTrack",
        "DataCellView",
        "DataCellKind.stacked-with-badge",
        "DataCellKind.tags",
        "structured-accessible-label",
        "generated-accessible-label",
        "horizontal-overflow",
        "rich-row-height",
        "cell-padding-x",
        "column-gap",
        "accessible-item-selected",
        "Key.UpArrow",
        "Key.DownArrow",
        "Key.Return",
        "Key.Space",
        "FocusRing",
        "AtlasTheme.surface-selected",
        "column-resize-requested",
        "filter-requested",
        "edit-requested",
        "context-menu-requested",
        "expansion-requested",
        "ListView",
        "compact",
    ] {
        assert!(
            table.contains(contract),
            "missing data capability: {contract}"
        );
    }
    assert_eq!(
        table
            .matches("for column in root.columns: DataColumnTrack")
            .count(),
        1,
        "the header must declare one canonical column-track sequence"
    );
    assert_eq!(
        table
            .matches("for cell[cell-index] in row.cells: DataColumnTrack")
            .count(),
        1,
        "desktop rows must reuse the canonical column-track constraints"
    );
    assert_eq!(
        table.matches("DataCellView {").count(),
        2,
        "desktop rows and compact cards must render the same semantic cell component"
    );
    let contracts =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/data-contracts.slint"))
            .expect("data contracts");
    for contract in [
        "DataCellKind",
        "DataCellAlignment",
        "DataCellTag",
        "primary-text",
        "secondary-text",
        "badge-text",
        "badge-tone",
        "tags: [DataCellTag]",
        "wrap: bool",
        "tag cells use it as one spoken label",
    ] {
        assert!(
            contracts.contains(contract),
            "missing rich-cell contract: {contract}"
        );
    }
    let states = fs::read_to_string(root.join("crates/atlas-ui-components/ui/data-states.slint"))
        .expect("data states");
    for contract in [
        "AtlasSkeleton",
        "AtlasEmptyState",
        "AtlasErrorState",
        "AtlasMotion.normal",
    ] {
        assert!(states.contains(contract), "missing data state: {contract}");
    }
}

#[test]
fn shared_visual_geometry_covers_reported_alignment_contracts() {
    let root = workspace_root();
    let core = fs::read_to_string(root.join("crates/atlas-ui-core/ui/stable.slint"))
        .expect("stable core facade");
    assert!(core.contains("in property <length> radius"));

    let switch = fs::read_to_string(root.join("crates/atlas-ui-components/ui/switch.slint"))
        .expect("switch component");
    assert!(switch.contains("radius: AtlasShape.radius-round"));

    let badge = fs::read_to_string(root.join("crates/atlas-ui-components/ui/badge.slint"))
        .expect("badge component");
    for contract in [
        "width: parent.width - self.x - AtlasGrid.space-2",
        "horizontal-alignment: center",
        "background: root.tone-color.with-alpha(0.08)",
        "border-color: root.tone-color.with-alpha(0.28)",
        "border-radius: AtlasShape.radius-small",
        "max-width: self.min-width",
        "horizontal-stretch: 0",
    ] {
        assert!(
            badge.contains(contract),
            "missing badge geometry: {contract}"
        );
    }

    let rich_content =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/rich-content.slint"))
            .expect("rich content components");
    for contract in [
        "clip: true",
        "y: AtlasGrid.space-10",
        "root.compact ? AtlasTheme.canvas : AtlasTheme.surface",
    ] {
        assert!(
            rich_content.contains(contract),
            "missing callout or compact-card geometry: {contract}"
        );
    }
    assert!(rich_content.contains("x: AtlasGrid.space-4; y: AtlasGrid.space-8"));
    for contract in [
        "width: min(label.preferred-width, parent.width - AtlasGrid.space-4)",
        "spacing: AtlasGrid.space-2",
        "horizontal-stretch: 1",
    ] {
        assert!(
            rich_content.contains(contract),
            "missing reference or content-tab geometry: {contract}"
        );
    }

    let states = fs::read_to_string(root.join("crates/atlas-ui-components/ui/data-states.slint"))
        .expect("data states");
    assert!(states.contains("horizontal-alignment: right"));

    let documentation =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/documentation-shell.slint"))
            .expect("documentation shell");
    for contract in [
        "padding-left: root.compact ? AtlasGrid.space-3 : AtlasGrid.space-4",
        "horizontal-stretch: root.compact ? 1 : 0",
        "x: root.main-x + AtlasGrid.space-2",
        "AtlasGrid.space-12 * 2 - AtlasGrid.space-2",
        "padding-top: AtlasGrid.space-1",
        "if root.compact: HorizontalLayout",
        "width: parent.width;",
    ] {
        assert!(
            documentation.contains(contract),
            "missing documentation spacing contract: {contract}"
        );
    }

    let final_wave =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/final-wave.slint"))
            .expect("final wave components");
    for contract in [
        "root.contained ? AtlasTheme.transparent : AtlasOverlayTokens.backdrop",
        "root.compact || root.contained",
    ] {
        assert!(
            final_wave.contains(contract),
            "missing contained drawer contract: {contract}"
        );
    }

    let migration =
        fs::read_to_string(root.join("crates/atlas-ui-components/ui/migration-wave.slint"))
            .expect("migration wave components");
    for contract in [
        "AtlasDensity.control-height + AtlasGrid.space-2",
        "width: root.indeterminate ? self.segment-width : parent.width * root.percent",
    ] {
        assert!(
            migration.contains(contract),
            "missing field or progress geometry: {contract}"
        );
    }

    let gallery =
        fs::read_to_string(root.join("apps/gallery/ui/gallery.slint")).expect("gallery fixture");
    assert!(gallery.matches("padding-top: AtlasGrid.space-2").count() >= 2);
}

#[test]
fn migration_wave_exports_bounded_controls_and_surfaces() {
    let source = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/migration-wave.slint"),
    )
    .expect("migration wave");
    for contract in [
        "AtlasPanel",
        "AtlasSelectField",
        "AtlasSegmentedControl",
        "AtlasProgressBar",
        "AtlasRadialProgress",
        "AtlasRangeControl",
        "AtlasPagination",
        "AtlasKeyValueList",
    ] {
        assert!(
            source.contains(contract),
            "missing migration contract: {contract}"
        );
    }

    let metric = fs::read_to_string(
        workspace_root().join("crates/atlas-ui-components/ui/metric-card.slint"),
    )
    .expect("stable metric card");
    for contract in [
        "AtlasMetricCard",
        "accessible-label",
        "accessible-value",
        "accessible-description",
        "compact",
        "overflow: elide",
    ] {
        assert!(
            metric.contains(contract),
            "missing metric contract: {contract}"
        );
    }
}

#[test]
fn final_migration_wave_covers_every_remaining_contract() {
    let source =
        fs::read_to_string(workspace_root().join("crates/atlas-ui-components/ui/final-wave.slint"))
            .expect("final migration wave");
    for contract in [
        "AtlasSparkline",
        "AtlasInlineAlert",
        "AtlasNoticeStack",
        "AtlasWorkflowBanner",
        "AtlasStepper",
        "AtlasDrawer",
        "AtlasErrorPage",
        "OverlayFocusController",
        "accessible-role",
    ] {
        assert!(
            source.contains(contract),
            "missing final migration contract: {contract}"
        );
    }
    let icons = fs::read_to_string(workspace_root().join("crates/atlas-ui-icons/ui/icons.slint"))
        .expect("icons");
    for icon in [
        "add",
        "back",
        "lock",
        "logs",
        "profile",
        "refresh",
        "shield",
        "warning",
        "grid",
        "terminal",
        "gamepad",
        "cpu",
        "memory",
        "play",
        "stop",
        "chevron-right",
        "layers",
    ] {
        assert!(icons.contains(icon), "missing icon: {icon}");
    }
}
