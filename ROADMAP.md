# Atlas UI roadmap

Atlas UI evolves from verified application needs and structured user feedback.
This roadmap records candidates for future work; it is not a delivery promise,
and ordering may change as compatibility, accessibility, rendering, and API
evidence develops.

## Selection principles

A candidate moves into implementation only when its intended use, ownership
boundary, accessibility contract, responsive behavior, and relationship to the
existing catalog are understood. New APIs begin in the preview facade and are
promoted to stable only after their contracts and migration risks are proven.

Atlas favors reusable primitives over narrowly branded screens. Rust continues
to own domain behavior and external effects, while Slint and Atlas own visual
composition and local interaction.

## Component candidates

The following components are candidates to be confirmed through real-world
feedback:

| Candidate | Intended scope |
|---|---|
| Radio group | Accessible single-choice selection with keyboard navigation and controlled state |
| Multiline text area | Long-form input, validation states, bounded growth, and scrolling |
| Combobox and autocomplete | Controlled suggestions, filtering intentions, keyboard navigation, and empty states |
| Tree view | Hierarchical disclosure, selection, focus movement, and scalable data contracts |
| Virtualized list | Bounded rendering for large collections with stable identity and host-owned data |
| File drop zone | Visual drag-and-drop intentions while filesystem access and validation remain host-controlled |
| Date and time picker | Locale-aware controlled selection without hidden persistence or system effects |
| Form validation primitives | Shared field messages, summaries, required-state semantics, and focus routing intentions |
| Advanced desktop navigation | Denser navigation patterns for multi-pane and information-rich native applications |
| Accordion and collapsible regions | Keyboard-operable disclosure groups with controlled expansion and nested-content contracts |
| Avatar and persona | Image, initials, presence, fallback, loading, and accessible identity presentation |
| Context menu and desktop menubar | Controlled menu models, nested navigation, shortcuts, focus restoration, and host-owned command execution |
| Floating and resizable panel | Bounded drag and resize intentions with host-owned placement persistence |
| Color picker | Controlled color selection with accessible numeric entry and explicit format conversion |
| Date-range picker | Locale-aware bounded range selection composed from shared date primitives |
| Data-visualization primitives | Tokenized sparklines, bars, lines, areas, and legends for host-provided data rather than a domain charting engine |
| Virtualized data table | Large-data presentation composed from existing table contracts, bounded rendering, stable identity, and host-owned sorting and pagination |

## Application block candidates

Application blocks compose Atlas components into reusable screen-level
structures:

| Candidate | Intended scope |
|---|---|
| Master-detail | Responsive collection and detail composition with controlled selection |
| Complete settings form | Sections, validation, save intentions, dirty state, and responsive navigation |
| CRUD workspace | Table, filtering, creation, editing, deletion confirmation, and explicit host callbacks |
| Onboarding flow | Ordered steps, progress, validation, optional actions, and resumable host-owned state |
| Search and command workflow | Search input, result navigation, commands, shortcuts, and controlled execution |
| Synchronization screen | Progress, conflicts, retry intentions, offline state, and last-known data presentation |
| Permission management | Roles, capabilities, grouped controls, warnings, and confirmation boundaries |
| Operational states | Composable offline, loading, empty, partial, stale, and error presentations |
| Responsive desktop shell | Top bar, navigation, content, contextual panels, overlays, and adaptive footers |
| Team and profile management | Reusable identity, membership, role, profile, and account compositions built from shared form and permission primitives |
| Metrics and monitoring workspace | Cards, status, trends, filters, alerts, and drill-down intentions for operational applications |

## Delivery and ecosystem roadmap

Atlas must strengthen delivery evidence and adoption alongside its component
catalog. The following work is prioritized independently from component count:

| Capability | Intended outcome |
|---|---|
| Public multi-platform CI | Compile and test the workspace and a clean consumer on Linux, Windows, and macOS |
| Registry-only smoke consumer | Prove each release using crates.io packages without workspace paths or inherited Cargo configuration |
| Interactive component gallery | Publish a browsable or WebAssembly-backed catalog with states, themes, density, responsive widths, and keyboard examples |
| Component API pages | Generate readable property, callback, maturity, accessibility, and usage documentation from authoritative manifests |
| Consumer examples | Maintain minimal and realistic applications that depend on Atlas exactly as external users do |
| Adoption evidence | Record known consumers, verified platforms, rendering profiles, and migration outcomes without overstating support |
| Agent protocol bridge | Expose the existing machine-readable manifest through a small MCP-compatible discovery layer without duplicating component source |
| Release compatibility matrix | Publish Stable and Preview changes, Slint requirements, platform evidence, and migration instructions for every release |

Atlas remains a centrally maintained shared dependency. Any future CLI, agent
bridge, or registry endpoint must install or reference canonical Atlas packages;
it must not silently turn the project into a copy-and-own component collection.

## Composition rule

Application blocks must compose existing Atlas components and tokens. They must
not copy or privately reimplement controls, interaction behavior, accessibility
contracts, or visual rules already owned by the component library. When a block
reveals a missing reusable primitive, that primitive is designed and validated
independently before the block depends on it.

## Feedback and prioritization

Prioritization considers demonstrated application demand, accessibility impact,
cross-platform feasibility, API maturity, maintenance cost, and compatibility
with the pinned Slint version. Use the repository's
[structured issue forms](https://github.com/rariux/Atlas-UI/issues/new/choose)
or the pinned
[Atlas UI 0.2 early-adopter feedback issue](https://github.com/rariux/Atlas-UI/issues/2)
to provide concrete use cases.
