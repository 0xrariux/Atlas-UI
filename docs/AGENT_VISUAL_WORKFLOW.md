# Visual workflow for coding agents

Use this guide when an agent builds an Atlas interface, especially when the
consumer application lives outside the Atlas repository.

## A dependency path is not a design brief

Giving an agent the path to Atlas only tells it where the library and its API
documentation live. It does not communicate the intended product identity,
page hierarchy, target viewport, content density, or reference composition.
Atlas supplies a visual grammar and reusable contracts; it does not infer a
finished application design from a component import.

High-quality generation needs three inputs:

1. **Atlas context** — the supported API, components, tokens, templates, and
   ownership boundaries;
2. **product direction** — a screenshot, mockup, existing web implementation,
   or a written brief with hierarchy, density, and responsive expectations;
3. **rendered evidence** — screenshots captured at explicit viewport sizes and
   reviewed before the work is considered complete.

Compilation proves API correctness. It does not prove visual quality.

## Connect an external consumer to a local checkout

Use a relative path when the repositories have a stable local relationship, or
an absolute path for temporary local work. The path must point to the
`crates/atlas-ui` facade, not only to the Atlas repository root.

```toml
# Cargo.toml
[dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui" }
slint = "=1.17.1"

[build-dependencies]
atlas-ui = { path = "../Atlas/crates/atlas-ui" }
slint-build = "=1.17.1"
```

```rust
// build.rs
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(atlas_ui::slint_library_paths());
    slint_build::compile_with_config("ui/app.slint", config)
        .expect("compile the Atlas UI application");
}
```

Keep machine-specific absolute paths out of committed manifests. Use the
published crate or a pinned Git dependency when the consumer must build without
the neighboring checkout.

## Give the agent Atlas context explicitly

Repository instructions do not automatically cross dependency boundaries. An
agent working in another repository will not necessarily discover or read
Atlas documentation merely because Cargo references it.

Add a short section like this to the consumer repository's `AGENTS.md`, using
the real Atlas checkout path:

```markdown
## Atlas UI

This application uses Atlas from `/absolute/path/to/Atlas`.

Before editing Slint, read these files in order:

1. `/absolute/path/to/Atlas/docs/AGENT_VISUAL_WORKFLOW.md`
2. `/absolute/path/to/Atlas/docs/AGENT_QUICKSTART.md`
3. `/absolute/path/to/Atlas/docs/AGENT_COMPONENT_INDEX.md`
4. `/absolute/path/to/Atlas/docs/atlas-ui-agent-manifest.json`

Use the manifest and public `.slint` declarations as API authority. Prefer
stable exports, compose existing components, and keep external effects in
Rust. Treat the supplied product reference as the visual authority. Compile,
capture the requested viewport, inspect the image, and refine the composition;
do not stop after `cargo check` alone.
```

If the consumer repository must not commit a local path, put the same context
in the task prompt or in a local, ignored agent-instruction file.

## Provide a visual contract

Before implementation, specify:

- the screen's purpose and primary user action;
- a visual reference, when one exists;
- the exact initial viewport and at least one narrower viewport;
- theme, density, required states, and realistic content;
- which aspects must match exactly and which may adapt to Atlas conventions.

When translating an existing web design, preserve its information hierarchy,
alignment, rhythm, density, and responsive behavior. Translate controls and
semantic values to Atlas components and tokens. Application-specific
composition may remain in the consumer; do not force every product surface
into a generic Atlas template.

Use `docs/AGENT_COMPONENT_INDEX.md` and the agent manifest to select components.
Use the gallery and relevant files in `screenshots/baselines/` as evidence of
Atlas component behavior, not as a substitute for the product's own visual
brief.

## Work in a render-and-review loop

For each target screen:

1. Establish the product hierarchy and select Atlas components before writing
   detailed markup.
2. Implement one representative state with realistic text and data.
3. Compile the external consumer.
4. Capture the rendered window at the agreed viewport and scale factor.
5. Inspect the screenshot for clipping, overlap, unintended stretching,
   alignment, typography, whitespace, contrast, and content density.
6. Compare it with the product reference and correct the largest structural
   differences first.
7. Repeat at a narrower viewport and verify focus, hover, empty, loading, and
   error states where relevant.

Do not accept a screen solely because it compiles or uses Atlas components.
Common generation failures include controls stretched to the parent width,
cards inheriting the full available height, fixed coordinates that only work at
one size, headings hidden behind chrome, placeholder content, invented APIs,
and arbitrary visual values that bypass Atlas tokens. Status badges are a
frequent special case: use `AtlasBadge` at its intrinsic width instead of
drawing a wide capsule with `Rectangle` and `AtlasShape.radius-round`.

## Reusable task prompt

Replace the bracketed values before giving this prompt to an agent:

```text
Build [screen or flow] with Rust, Slint, and Atlas UI.

Atlas checkout: [absolute path to Atlas]
Product reference: [screenshot, mockup, web source, or written brief]
Target viewports: [width x height and narrow width x height]
Theme and density: [values]

Before writing code, read docs/AGENT_VISUAL_WORKFLOW.md,
docs/AGENT_QUICKSTART.md, docs/AGENT_COMPONENT_INDEX.md, and query
docs/atlas-ui-agent-manifest.json inside the Atlas checkout. Inspect the
relevant Atlas source declarations when a signature is uncertain.

Preserve the reference's hierarchy, proportions, alignment, and density. Use
Atlas components and tokens for shared UI contracts, while keeping
application-specific composition in the consumer. Do not copy Atlas component
implementations or invent properties.

The task is complete only after the consumer compiles and you have captured,
visually inspected, and refined screenshots at both target viewports. Report
the Atlas facade used, any preview APIs, the capture settings, and remaining
visual differences.
```
