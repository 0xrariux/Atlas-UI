//! Workspace dependency and public-contract checks.

use std::{fs, path::Path};

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("testing crate remains under crates/")
}

#[test]
fn public_slint_facades_exist() {
    let root = workspace_root();
    for relative in [
        "crates/atlas-ui-tokens/ui/tokens.slint",
        "crates/atlas-ui-core/ui/core.slint",
        "crates/atlas-ui-icons/ui/icons.slint",
        "crates/atlas-ui-components/ui/components.slint",
    ] {
        assert!(root.join(relative).is_file(), "missing facade: {relative}");
    }
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
        "JetBrainsMono-Variable.ttf",
        "font-sans: \"Inter\"",
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
        "restore-focus-requested",
    ] {
        assert!(
            core.contains(contract),
            "missing focus boundary contract: {contract}"
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
        "x: AtlasGrid.zero; y: AtlasGrid.zero; width: parent.width * root.percent; height: parent.height",
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
        "AtlasMetricCard",
        "AtlasSelectField",
        "AtlasSegmentedControl",
        "AtlasProgressBar",
        "AtlasRangeControl",
        "AtlasPagination",
        "AtlasKeyValueList",
        "accessible-value",
    ] {
        assert!(
            source.contains(contract),
            "missing migration contract: {contract}"
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
        "add", "back", "lock", "logs", "profile", "refresh", "shield", "warning",
    ] {
        assert!(icons.contains(icon), "missing icon: {icon}");
    }
}
