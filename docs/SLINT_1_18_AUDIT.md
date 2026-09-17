# Slint 1.18.0 impact audit

Reviewed on 2026-09-16 against the [Slint 1.18.0 release](https://github.com/slint-ui/slint/releases/tag/v1.18.0) and its [1.18.0 changelog](https://github.com/slint-ui/slint/blob/master/CHANGELOG.md#1180---2026-09-16). The source checkout pins `slint = "=1.18.0"` and `slint-build = "=1.18.0"`. The 2026-09-17 release review approved the macOS arm64 software-renderer baselines and completed Rust 1.92 CI on Linux, Windows, and macOS. Native assistive-technology, IME, and touch behavior remain outside that evidence.

## Decision summary

Slint 1.18.0 is integrated into Atlas 0.2.0. `FlexboxLayout` compiles without the experimental gate, layout measurement is corrected, and text gains a line-height control. The local quality and release gates pass. All 77 visual references were reviewed and updated for the macOS arm64 software profile, and the Rust 1.92 CI matrix passes.

The upstream changelog contains 201 entries: 66 general, 58 language, 15 widgets, 23 Rust, 13 C++, 6 JavaScript, 8 Python, and 12 tooling. The inventory below covers each release section by subject and separates Atlas contact points from work that only affects other bindings or targets.

## Atlas impact register

| Priority | Upstream change | Atlas contact point | Assessment and required evidence |
|---|---|---|---|
| P0 | `FlexboxLayout` is released; per-child `cross-axis-self-alignment` and `layout-order` are added. Its former experimental `flex-basis`, `flex-grow`, and `flex-shrink` properties are removed; use preferred size, stretch and minimum constraints. | `core/ui/responsive-layout.slint`, `components/ui/rich-content.slint`, preview facades, former `.cargo/config.toml`, architecture tests. | Atlas now compiles its facades with 1.18 **without** `SLINT_ENABLE_EXPERIMENTAL_FEATURES`. Responsive recipes remain Atlas preview. `layout-order` changes visual order only when used; keyboard and accessibility order must be reviewed before Atlas exposes it. |
| P0 | `Flickable.viewport-x/y/width/height` become deprecated aliases of `content-x/y/width/height`; press propagation changes when nothing can pan. | `core/ui/scroll-viewport.slint`, `components/ui/data-table.slint`, gallery, and scroll architecture tests. | The old names still work in 1.18, so this is migration debt rather than an immediate break. Rename **only actual Flickable properties** and recheck sign conventions, offsets, pointer interaction, scrollbar synchronization, and exact public Atlas property names. Atlas's own `viewport-y` and `viewport-height` are distinct public contracts. |
| P0 | Wrapped text measurement changes for `Text`, padded layouts, and repeated `VerticalLayout`/`GridLayout` cells; min/max constraints are corrected. | `editorial.slint`, `data-table.slint`, `documentation-shell.slint`, responsive recipes, 77 documented screenshot baselines. | Expect geometry and screenshot changes even if compilation succeeds. Compare 1.17.1/1.18.0 at narrow widths, repeated wrapped rows, long words, density variants, and explicit size constraints. Classify intentional upstream corrections before updating baselines. |
| P0 | Software renderer changes glyph spacing, long-line safety, partial redraw, occlusion handling, mixed-script hit testing, and rotated `Path` rendering. | Gallery's `MinimalSoftwareWindow` capture path and macOS arm64 software-renderer evidence. | Capture and review representative scenarios, then the full approved set if differences appear; rerun performance budgets. Do not infer cross-renderer parity from the software profile. |
| P1 | `Text`, `TextInput`, and `StyledText` gain `line-height-factor`; `Text`/`StyledText` gain `max-lines`. | Atlas typography tokens and `AtlasHeading`, `AtlasParagraph`, `AtlasStyledText`, code blocks, table cells. | Native `Text` descendants already expose these properties; wrapped editorial controls now forward relevant inputs. Existing font-metric defaults remain. Cross-font measurement is required before binding new global leading tokens; the same factor does not guarantee the same absolute leading across fonts. |
| P1 | Accessibility exposes text-input content and selection; focus clearing, tree construction, and traversal improve. | `AtlasTextField`, `AtlasSelectableText`, `AtlasScrollViewport`, menus, tabs, collection roles. | Verify accessibility names, value/selection, password or private data behavior, focus loss on hiding, keyboard traversal and screen-reader output on each claimed backend. This is upstream behavior, not automatic proof of Atlas accessibility. |
| P1 | `TextInput` fixes programmatic text updates/undo, caret movement, ligature selection and moving IME geometry. | `text-field.slint`, read-only selectable text, search and command controls. | `AtlasTextField` now forwards optional `input-method-hints`. Controlled value changes, wrapped multiline text, selection, undo, and IME still require host and platform exercises. |
| P1 | `Slider.changed` no longer fires at an unchanged bound; `pressed` follows its touch area. | `AtlasRangeControl` in `migration-wave.slint`. | The software-window fixture verifies pointer focus, keyboard movement to both limits, and no repeated callback at an unchanged limit. Touch and native-platform behavior remain unverified. |
| P1 | Generated Rust bindings export only publicly reachable types; renamed exports retain deprecated aliases. | Gallery and getting-started generated bindings; public facade exports. | Compile the consumers and any external-consumer contract fixtures. The source audit cannot prove that generated public Rust names remain compatible. |
| P1 | `slint-build` makes software resource embedding a default-on feature; non-PNG/JPEG image formats require an opt-in feature. | Gallery build scripts and SVG Atlas icons. | Verify default feature resolution and capture SVG icons with the software renderer. The changelog does not establish that ordinary SVG icons need `image-default-formats`; inspect resolved features before changing them. |
| P1 | Build output and translations become deterministic; generated Rust becomes smaller and faster to build. | Reproducible package and binary-size checks. | Compare clean builds, release binary size, compile time, and resource embedding. These are prospective benefits, not measured Atlas improvements yet. |
| P1 | Rendering changes clipping at bordered edges, gradients, shadows, SVG fallback and cache behavior. | Surfaces, cards, focus rings, icons, screenshot baselines. | Review border radius and clipping, shadow transforms, gradients, SVG icons, and animation frames. Benefits or regressions depend on renderer and geometry. |
| P1 | A focused item becoming invisible now loses focus; `PopupWindow` offsets react to size changes. | Atlas overlays, menus, modal/drawer recipes, focus controllers. | Test close/restore focus, conditional items, resizing popups, and nested overlays. Atlas still owns focus restoration and placement policy. |
| P2 | Models gain `push`, `insert`, and `remove` in Slint and Rust `SharedVector` gains removal/insertion; `SortModel` bulk removal improves. | Data tables/lists, rich-content models and Rust document adapters. | Can simplify future local mutations. No existing Atlas data contract automatically gains stable item identity, paging, tree data, or virtualization guarantees. Test model mutation while pointer capture is active. |
| P2 | Dynamic `z`, struct defaults, string predicates/replacement, inferred enum/color literals, spring easing, path position/angle, and window movement become available. | Atlas tokens, overlays, animation and window recipes. | Candidate capabilities only. Prototype each against existing public contracts; spring needs reduced-motion policy, dynamic `z` needs hit-test/focus checks, and `WindowMoveArea` is relevant only for custom title bars. |
| P2 | `Window.x/y` are deprecated; `Window.close()` reports whether closure was accepted; untitled windows now use the app name. | Gallery and consumer window behavior. | No direct Atlas `Window.x/y` bindings were found. Inspect any consumer code and screenshots that assume a title or close result. |
| P2 | New Rust `Image::load_from_data`, `Keys::to_parts`, `DataTransfer` file paths, `update_all_translations`, event dispatch result, and event-loop hook. | Possible future host integrations. | No direct use found in Atlas's current Rust facade. Keep filesystem and drag payload validation in the host. |
| P2 | Vello renderer is experimental; WGPU 30, Skia Vulkan on macOS, LinuxKMS options, and other renderer changes. | Optional consumer targets only. | Atlas does not enable these explicitly and has no validation evidence for them. Keep renderer claims tied to tested profiles. |

## Complete upstream change inventory by section

The following is a compact subject inventory of the 201 changelog entries. Consult the linked changelog for issue numbers and exact upstream wording.

### General: 66 entries

- **Rendering and images:** FemtoVG on WGPU 30 and WebAssembly; experimental Vello; browser image decoding and smaller WebAssembly assets; long-text layout performance; bordered clipping; shadow caching and transforms; SVG cache growth and unknown-font fallback; gradient-stop normalization; Skia opaque-image and WGPU fixes; software-renderer occlusion, glyph spacing, long-line stability, mixed-script text hit testing and rotated paths; FemtoVG fractional SVG/path placement and snapshot freshness; partial repaint after removing elements; Vulkan texture import.
- **Build and runtime:** deterministic generated output and translations; font registration before first component; repeater count overflow; window-adapter error reporting; native menu-bar lifetime; callbacks on removed parent items; Android/custom-platform MCP startup; first frame before mapping and width before show; automatic window title.
- **Input, accessibility, animation and layout:** assistive access to input content/selection and accessibility-tree panic fix; unchanged-target animation and long cubic-bezier fixes; hidden-item focus, large-tree focus/visibility performance; `Flickable` press forwarding; bulk `SortModel` removal; layout min/max and percentage constraints; wrapped repeated layout row measurement; conditional repeated-grid opacity generation; IME repositioning.
- **Platforms:** Qt native outbound drag/drop; winit key repeat, Wayland sizing and software redraw, older portal settings; Windows dead keys; macOS older-system startup and title-bar transparency; Android refresh-rate animations, virtual-keyboard repeat and caret/selection handles, Back handling, cutout safe area and redraw wakeups; ESP-IDF MIPI-DSI panel selection; LinuxKMS cursor/software performance; WebAssembly modifier retention, accent color and first-touch Safari panic.

### Slint language: 58 entries

- **New capabilities:** `FlexboxLayout`, child self-alignment/order, struct defaults, mutable array/model operations, `starts-with`/`ends-with`/`replace-all`, spring easing, custom title-bar role and `WindowMoveArea`, path point/angle queries, input-method hints, maximum rendered lines, line-height factor, preview `Platform.uses-mock-data`, and mutable `z`.
- **Changed semantics:** `Flickable.mouse-drag-pan-enabled` and deprecated `viewport-*` aliases; private-member shadowing; inferred enum/color literals; deprecated window position properties and accepted-close result; Markdown inline-code appearance; improved text minimum width; proper selection/drop cursors and popup offsets.
- **Binding, compiler and runtime fixes:** compile-time two-way/`StyledText` loop detection and callback-alias warning; global two-way `changed`/animation behavior; generated-code global animation panic; timer/popup calls in repeated/conditional contexts; re-evaluated repeated callbacks; transformed absolute positions; repeated-grid visibility panic; system-tray show/hide; stale text cursor/undo, wrapped caret and ligature colors; image size recursion; path stroke positioning and inherited path commands; invalid identifiers, backslash and `builtin:` imports, better unknown-name suggestions, `ContextMenuArea`/custom `Row` compiler panics, duplicate struct/enum names across files, ternaries with arrays/structs, inherited private state properties, interpreter compound-assignment animation, pure-context native calls, stale property reads, inherited focus methods, same-id popup states and two grid-collapse/panic fixes. Menu clicks on an already open entry now close that menu.

### Standard widgets: 15 entries

`ComboBox.current-value` now selects or clears its matching row; explicit `ListView` viewport sizing is respected; direct `RadioGroup`/`TabWidget` construction and subclassed tabs no longer crash; `SpinBox` arrow keys, range acceptance and accessibility are corrected; compound-widget inner nodes leave the accessibility tree; `Slider` boundary events and pressed state change; tab-bar wheel direction, `DatePicker` year list, named `RadioGroup.selected` argument, Breeze `LineEdit` padding, and `LineEdit` undo/redo are fixed or added. Atlas composes its own controls for many of these and directly wraps a standard `Slider`; only the latter has a confirmed direct widget dependency.

### Rust: 23 entries

WGPU 30 API/feature replaces WGPU 28 while WGPU 29 remains for Skia; fontique 0.11 feature/module names replace 0.10; translation refresh, result-bearing window-event dispatch, memory-buffer image decode, big-endian RGB565, `SharedVector` insert/remove, shortcut decomposition, generated public-type reachability, Skia WGPU renderer renaming, file-path data transfer, software resource-embedding feature and optional extra image formats, smaller/faster generated code, sRGB WGPU render targets, macOS Skia Vulkan, optional LinuxKMS libseat/libinput, removal of no-std snapshots, borrow-check and grid pointer-grab fixes, winit active-loop invocation, transformed-center testing events, Skia rendering-state callbacks and translation rebuild tracking. Atlas directly uses `take_snapshot()` in a standard Rust build; the no-std restriction does not apply to that path.

### Other bindings and tools: 39 entries

- **C++ (13):** memory-buffer image decode, file-path data transfer, shortcut conversion and more flexible input ranges, deprecated renamed generated exports, model dirty-state fix, smaller generated code, LinuxKMS options, install-libdir behavior, early event-loop invocation, repeater-removal crash, system-tray build option and live-preview warning cleanup.
- **JavaScript (6):** event dispatch, Windows Node event-loop integration, constructor spelling flexibility, console logging for debug/warnings, strict default `Model.setRowData`, and Alpine/musl binaries.
- **Python (8):** model-wrapper lifetime, Ctrl-C interruption, asyncio leaks and CPU usage, shutdown traceback, `Color.mix` stub, builtin enum annotations, and Alpine/musl wheels.
- **Tooling (12):** persisted live-preview UI settings, resizable/corrected Edit Values window, remote-preview input/file validation plus paired encrypted connections, LSP cross-language rename/import/formatter diagnostics, viewer screenshot sizing/MCP/relative-path reload, and SlintPad import-demo panic correction.

Atlas's current Rust/Slint workspace does not directly consume the C++, JavaScript, Python, Qt, ESP-IDF, LinuxKMS, Android or WebAssembly bindings in its validated profile. Their changes matter only when Atlas expands target support or when an external consumer chooses those runtimes.

## Proposed validation order

1. Create an isolated 1.18.0 candidate with exact `slint` and `slint-build` pins. Record `cargo tree -e features`, `rustc --version`, and the resolved image/render features. Do not use `cargo update` alone while exact 1.17.1 constraints remain.
2. Compile stable, nonresponsive preview, responsive preview, gallery, and getting-started consumers. Check generated exports and all compiler deprecations. Test compilation without the experimental environment flag before retiring that flag.
3. Run `sh scripts/quality-gate.sh`. Update architecture assertions that encode 1.17.1 experimental status only after the 1.18 build proves the new boundary.
4. Compare responsive layouts, wrapped text, scroll offsets, pointer handling, focus, input selection and slider boundary callbacks using targeted scenarios. Capture representative software-renderer frames before deciding whether full baseline regeneration is justified.
5. Run the full visual review and performance budgets if representative frames or timing change. Update version-bearing manifests, screenshot metadata, compatibility docs, gallery copy, watchlist and consumer instructions only after the candidate passes.

## Initial migration status on 2026-09-16

- Exact Slint 1.18.0 runtime/build pins and registry lockfile are in place. `const-field-offset` and its macro resolve to 0.2.1; a stale 0.2.0 resolution caused generated-code compile errors until updated.
- Removed the experimental compiler flag. Replaced former flex item properties with preferred sizes and stretch/constraint properties, including dual-axis sizing for panes that stack at a breakpoint. Migrated actual `Flickable`, `ScrollView`, and `ListView` properties to `content-*` while preserving Atlas's own public `viewport-*` contract.
- The workspace compiles, tests pass, and the public quality gate passes locally. Source compatibility metadata, generated agent manifest, examples and documentation have been updated. The published Atlas 0.1.1 package remains a separate Slint 1.17.1 artifact.
- All 77 macOS arm64 software scenarios were captured against the 1.17.1 references; none passed the existing pixel threshold. Initial large geometry differences in sidebar/switcher/split recipes were corrected. The remaining comparison includes text-rendering changes, multiline measurement changes and deliberate copy changes. Reference PNGs have not been overwritten or approved. Linux, Windows, and Rust 1.92 CI evidence is pending.
- All four local render-performance budgets passed. Recorded medians were approximately 393 ms, 382 ms, 294 ms and 525 ms against a 2,000 ms limit. These figures are local debug-build measurements, not release-binary benchmarks.

## Reinforced audit: component and visual evidence

The component-source sweep checked every Slint `Flickable`, `ListView`, and
`ScrollView` property use in the workspace. Remaining `viewport-*` bindings
belong to Atlas's own public scroll/scrollbar components or their callers;
native scroll widgets use `content-*`. The workspace compile includes all
facades, gallery, and getting-started consumer. It does not exercise every
control at runtime.

The [runtime layout fixture](../crates/atlas-ui-testing/tests/layout_runtime.rs)
now renders real `AtlasSwitcher`, nested `AtlasAutoGrid`, and
`AtlasDocumentList` components with Slint's software window. It checks
wide/narrow/wide geometry, nested grid column counts, pane non-overlap and
min/max widths, plus document-row growth and recovery after text replacement,
insertion, and removal. This closes the first runtime measurement gap; repeated
table/grid cells, mixed font scales, other renderers, and native platforms still
need equivalent evidence.

The fixture exposed a composition boundary: when a flex-sized pane contains an
`AtlasAutoGrid` whose width follows that pane, binding each card's
`preferred-width` to the grid's `item-basis` creates a compiler-detected layout
cycle. Atlas now exposes `basis-width`; the passing fixture binds both
`reference-width` and `basis-width` to an independent parent width while cards
bind `preferred-width` to `item-basis`. This provides an explicit recipe for
the variable-basis composition without an intrinsic measurement cycle.

P0.1 now has a public integer-logical-pixel track allocator in Atlas Rust.
`AtlasDataTable` accepts one optional allocated-width model for its header and
rows. Both gallery tables now bind that model through a pure callback to the
Rust allocator, with viewport and column changes triggering recalculation; a
consumer without this binding retains native constraints. A nested
`AtlasContainer` software-window fixture measures local content size classes.
The gallery also binds document tables and key/value tracks; resized data
columns retain preferred widths by stable ID. `AtlasContainer` now forwards an
explicit environment value through nested regions, and a separate viewport
fixture checks two-axis offsets, reveal requests, and logical-direction keys.
A separate Slint 1.18 fixture confirms that `layout-order` repositions visual
children while Tab focus retains declaration order, so interactive reordering
still needs an explicit focus and accessibility policy. Another fixture checks
wrapped repeated grid rows, model insertion/removal, and a conditional spanning
row against live Slint geometry.

The [runtime interaction fixture](../crates/atlas-ui-testing/tests/interaction_runtime.rs)
dispatches actual Slint pointer and key events. It exposed two `ActionArea`
defects: explicit keyboard focus lost its visible indicator, and releasing a
different activation key could trigger the action. Both are fixed. The fixture
also checks disabling during a key press, tab traversal past a disabled
control, reenable without stale focus, pointer activation, and `AtlasModal`
confirmation, Tab traversal, Escape dismissal, and focus restoration. It runs
with the software renderer; native screen-reader behavior, IME, touch, nested
overlays, and other control families remain unverified.

The next interaction pass found that Slint 1.18 drops focus from hidden items:
the headless `OverlayFocusController` could not receive menu keys while its
root was invisible. It now remains visible at zero size, and `AtlasMenu` handles
arrows, Home/End, Enter/Space, Escape, Tab exit, disabled-item activation, and
focus restoration. Tab exit suppresses restoration to the invoking control so
the host's requested next or previous focus target can remain focused. The same
fixture covers `AtlasDrawer` Escape and focus
restoration. The disabled item remains in the arrow sequence so assistive
technology can discover it, following the [WAI-ARIA menu convention](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/).
Slint 1.18 does not expose menu/menu-item accessibility roles, so Atlas still
uses list/list-item roles; native announcement and active-item relationships
remain an open platform audit item.

The [workspace-tab runtime fixture](../crates/atlas-ui-testing/tests/tabs_runtime.rs)
dispatches Left/Right, Home/End, Enter/Space, and Delete to real stable tabs.
It verifies wraparound, focus on the surviving tab after close, and recovery
after the host disables the active tab. `AtlasWorkspaceTabList.focus-index()`
now lets the host name an eligible tab directly; it clamps the requested index
and emits the existing `focus-requested` intention. Eligibility remains
host-owned because the list receives a tab count rather than item availability.

The [nested-overlay runtime fixture](../crates/atlas-ui-testing/tests/nested_overlay_runtime.rs)
opens a modal and a menu in separate updates and in either order within one
update. In the atomic case, the modal's `focus-on-open: !menu-open` gives the
menu priority over the modal's initial focus callback. Escape closes the menu
and returns focus to a control inside the modal; another Escape closes the
modal and returns focus to the outside invoker. A separate software-window
fixture also unwinds a modal/popover/menu stack opened in one update. Arbitrary
stack depth and native assistive-technology behavior still need evidence.

The first 77-scenario 1.18.0 sweep failed all 77 comparisons against the
1.17.1 pixel threshold of 0.2%. Direct inspection of the largest differences
covered responsive layouts, intrinsic sizing, rich content, typography,
document navigation, Markdown presentation, footnotes, web proof, international
content, and media states. Text rasterization changes account for many pixels,
but the comparisons also expose layout and content changes. In particular:

All 77 scenarios were recaptured on 2026-09-17 after the final Slint and gallery changes.
The
[machine-readable delta report](slint-1.18-visual-deltas.json) records each
scenario's changed-pixel ratio and mean absolute difference against the old
reference. None meets the old 0.2% threshold (range: 0.90% to 20.31%). This
is migration evidence, not approval of the 1.18.0 rendering; a pixel mismatch
does not itself establish a functional defect.

| Surface | Finding | Action/status |
|---|---|---|
| `AtlasRichText` fragment flow | With 1.18.0, measured `Text.preferred-width` no longer reserved the trailing spaces used by the gallery's styled fragments. The rendered words joined, although the accessibility string remained correct. | Added scale-aware spacing for fragments ending in a space. Recaptured `markdown-presentation.light.compact.mobile`: visible word separation is restored. Its final 6.74% difference also includes the adaptive list-row geometry below; the 1.18 reference is approved. |
| `AtlasDocumentList` at large text scale | Fixed-height repeated rows overlapped when long list items wrapped to two lines. A 360 px, large-type Markdown capture exposed the defect. | Rows now take at least the measured text height, the list reports its layout's preferred height, and the gallery allocates more room when typography is enlarged. The large-type capture shows separated rows. |
| Native `Flickable` properties | A fresh Rust 1.92 build exposed remaining deprecated `viewport-*` bindings in `AtlasScrollViewport`. | Migrated its native properties to `content-*` and added a source contract check. Atlas's public `viewport-*` names remain intact. |
| Editorial text controls | `AtlasHeading` and `AtlasParagraph` already inherit the new Slint `Text` line-height and maximum-line properties; wrapped text controls did not forward them. | `AtlasStyledText` now forwards `line-height-factor` and `max-lines`; `AtlasSelectableText` forwards `line-height-factor`. Defaults preserve existing font normalization and visuals. |
| `AtlasTextField` input method | Slint 1.18.0 adds platform input-method hints to `TextInput`; the Atlas wrapper previously hid them. | The field now forwards an optional `input-method-hints` property. Actual soft-keyboard behavior still depends on the selected platform and is not established by the macOS software capture. |
| Rich-content figure and terminal in a stacked `AtlasSwitcher` | The new flex sizing divides the fixed height differently from the 1.17.1 reference; the figure is shorter and its SVG appears smaller. Gallery copy and terminal output also changed for the release. | The `rich-content.light.compact.narrow` reference was approved with the 1.18 set; its historical difference was 11.70%. |
| Media-state gallery capture | The 1.18.0 capture shows a visible scroll indicator near the right edge where the old reference does not. Figure and loading-state content otherwise remain positioned consistently in the inspected viewport. | The visible indicator was accepted in the approved 1.18 reference. Live scroll interaction remains outside the software capture. |
| `AtlasRangeControl`, `AtlasTextField`, selectable text, focus boundaries and overlays | Source bindings compile. Software-window fixtures cover shared action activation, modal/menu/drawer/popover/combobox/autocomplete/radio keyboard paths, menu outside dismissal, focus return, and a three-layer stack. Combobox and autocomplete fixtures also cover menu-model shortening, anchor invalidation, and host menu coordinates. The range-control fixture confirms that keyboard attempts to move past either bound do not emit another `value-changed` callback. IME composition, selectable text, touch, arbitrary stack depth, and platform accessibility remain outside these fixtures. | Run the remaining control and native assistive-technology matrices before claiming full interaction coverage. |

The quality gate tests source and Rust contracts, while the visual runner
captures rendered states. Neither supplies a complete interaction or
cross-platform accessibility audit. A clean-target macOS build and workspace
test run passed with Rust 1.92.0 installed in a temporary toolchain. The
historical 1.17.1 baselines were retained until the 1.18.0 set received
explicit release review approval.

The complete macOS quality gate also passed with Rust 1.92.0, including
formatting, Clippy with warnings denied, tests, and repository contracts. A
Rust 1.92.0 cross-target workspace check passed for `x86_64-pc-windows-gnu`.
The equivalent Linux cross-target check stopped in the upstream fontconfig
build script because this macOS host has no Linux `pkg-config` sysroot. Native
Linux and Windows CI subsequently passed on hosted Rust 1.92 runners.
All four local software-renderer budgets passed again after the final source
changes, with medians of approximately 395, 381, 294, and 525 ms against their
2,000 ms limits.

## Pre-tag verification on 2026-09-17

| Gate | Result |
|---|---|
| Workspace format, build, Clippy, tests, package contents and repository contracts | Passed with the locally installed Rust 1.97.1 toolchain. The 10,000-document search budget now passes in the full test suite after ranking before excerpt construction. |
| Slint source migration | Exact runtime/build pin `1.18.0`; no experimental compiler flag or deprecated native `Flickable.viewport-*` binding remains. Runtime fixtures cover flex layout order, responsive sizing, scroll offsets, keyboard focus, overlays, text controls and slider boundary callbacks. |
| External consumers | Nexus `cargo check --offline --locked --all-targets` passed. Command, Forge, Fleet and Ledger compiled against this Atlas checkout and captured 97 states in total. |
| Atlas visual comparison | Passed after review: the 77 macOS arm64 software-renderer references were approved for Slint 1.18.0. The historical 1.17.1-to-1.18.0 pixel-delta range was 0.90%–20.31%; all fresh comparisons with the approved 1.18.0 set pass. |
| Platform CI | Passed: [Atlas Rust 1.92 CI](https://github.com/0xrariux/Atlas-UI/actions/runs/35233008308) and [template CI](https://github.com/0xrariux/template-atlas/actions/runs/35234393099) completed on Linux, Windows, and macOS. |
| Native interaction limits | macOS/Linux/Windows assistive-technology reviews, IME, touch and live mouse-drag panning remain unverified. A synthetic software-window mouse-drag probe did not establish viewport movement, so that path is not counted as verified. |
| Release identity | Workspace and inter-crate dependencies say `0.2.0`; the source tag is separate from the still-published crates.io `0.1.1` packages. |

### Human visual review corrections

The reviewer identified layout defects in the initial 1.18.0 captures. The
shared callout frame now has square left corners along its colored accent and
retains rounded right corners. Compact metric cards reserve bottom space for
their final text line. The documentation footer divider now sits midway
between button rows; the content viewport has top inset and a gap before its
navigation row. The stacked layout pane gives its final button a bottom inset,
and data-table status badges are vertically centered in their rows. The
affected scenarios were inspected after recapture, then all 77 scenarios were
recaptured again. The 1.18.0 baseline set was approved during the release
review.

The source and consumer compilation gates, visual comparison, and platform CI
are green. The `0.2.0` source tag is supported by this evidence; registry
publication and broader native interaction claims remain separate decisions.
