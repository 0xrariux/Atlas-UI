# Native Rust tooling

Atlas UI maintenance automation is implemented by the `atlas-ui-tooling` Rust
crate. The repository does not require Node.js for validation, manifest
generation, screenshot capture, performance measurement, or visual review.

Run commands from the workspace root:

```bash
cargo run -p atlas-ui-tooling -- generate-agent-manifest --check
cargo run -p atlas-ui-tooling -- capture-scenarios --validate-only
cargo run -p atlas-ui-tooling -- review-screenshots --dry-run
cargo run -p atlas-ui-tooling -- measure-render-performance
cargo run -p atlas-ui-tooling -- validate all
```

The screenshot commands preserve the existing options, including `--scenario`,
`--update-baselines`, `--approve-baseline`, `--reviewer`, `--note`, `--reset`,
`--rerun-last-batch`, and `--batch-size`.

The public quality gate remains the canonical entry point:

```bash
sh scripts/quality-gate.sh
```

It runs Cargo formatting, compilation, Clippy, tests, public validation, package
content checks, local Markdown-link validation, the Rust-only tooling invariant,
optional local `ai/` data validation, source-derived agent-manifest validation,
and capture-manifest validation. It deliberately does not launch 77 graphical
captures.

Before publishing a release, run the exhaustive gate on the reference computer:

```bash
cargo run -p atlas-ui-tooling -- release-gate
```

`release-gate` runs the complete native quality gate, recaptures every declared
visual scenario, and compares each result with its approved baseline. It never
updates baselines automatically.

When `ATLAS_UI_GALLERY_CAPTURE` is set by the capture tooling, the gallery
installs Slint's `MinimalSoftwareWindow` platform before constructing the UI.
This headless path renders the requested logical viewport and writes its
snapshot without registering a native operating-system window. Running the
gallery normally still uses Slint's native event loop and window backend. The
release tooling waits one second before each snapshot so bundled fonts and the
expanded gallery finish their first software-rendered frame consistently.

The companion applications are validated separately so the Atlas workspace
does not depend on product code:

```bash
sh scripts/template-consumer-gate.sh
sh scripts/template-consumer-gate.sh --capture
```

The first command compiles Command, Forge, Fleet, and Ledger against the local
Atlas checkout. The second also renders their 97 declared states to
`target/template-consumer-captures/` for release review. See
[External consumer scenarios](EXTERNAL_CONSUMER_SCENARIOS.md).

When the ignored local `ai/` directory is available, native validation covers:

- exact Rust and Slint compatibility pins and capability policies;
- token registries, facade exports, and forbidden visual literals;
- deterministic visual scenario identities;
- stable and preview API partitioning;
- WCAG contrast evidence and accessibility source contracts;
- SHA-256 integrity and licensing files for icons and fonts;
- Rust and render-performance sampling contracts;
- legacy migration and retirement consistency;
- release-readiness evidence and baseline approval counts;
- component compatibility groups and their visual scenarios.
