# Component runtime evidence

Atlas tracks evidence for existing components independently from the stable API
count. A component counts toward **direct Slint runtime fixture coverage** only
when a public component is instantiated in a compiled Slint fixture and a Rust
test creates that fixture. The tests use Slint's `MinimalSoftwareWindow` and
exercise real layout, model updates, focus, or input paths. This metric records
test reachability; it does not imply that every property or interaction of the
component is covered.

**Slint event-dispatch fixture coverage** is the subset whose associated Rust
test sends `WindowEvent` input through Slint. It measures exercised native Slint
input paths, including keyboard and pointer routing. It does not measure native
operating-system window or accessibility behavior.

| Evidence metric | Published v0.1.1 baseline | Atlas v0.2.2 · Slint 1.18 |
|---|---:|---:|
| Previously published components with direct runtime fixtures | 0/97 | 13/97 |
| Previously published components in event-dispatch fixtures | 0/97 | 9/97 |
| Newly added preview components with direct runtime fixtures | — | 6/6 |
| Newly added preview components in event-dispatch fixtures | — | 5/6 |
| All public components with direct runtime fixtures | 0/97 | 19/103 |
| All public components in event-dispatch fixtures | 0/97 | 14/103 |
| Public stable components | 26 | 26 |

The v0.1.1 tag contains no Slint runtime fixture or corresponding Rust runtime
test. The current source fixture is
[`layout-runtime.slint`](../crates/atlas-ui-testing/ui/layout-runtime.slint),
with its executable tests in `crates/atlas-ui-testing/tests/*_runtime.rs`. The
six new preview components are `AtlasContainer`, `AtlasPopover`,
`AtlasCombobox`, `AtlasAutocomplete`, `AtlasRadioGroup`, and `AtlasTreeView`.

Run the metric directly with:

```bash
cargo run -p atlas-ui-tooling -- component-evidence
```

The quality gate runs `component-evidence --check`, which recomputes coverage
from the public API manifest, Slint fixture instantiations, and Rust test
references. It rejects a stale table here or an unreviewed change to the
v0.1.1 component baseline. A passing quality gate also runs those Rust tests.

**Native platform evidence is a separate gate.** The runtime fixtures use a
software window and do not inspect a macOS, Linux, or Windows accessibility
tree, input method, touch path, or platform renderer. The Slint 1.18 migration
has **0/3 completed platform-specific interaction and assistive-technology
reviews**. The [compatibility matrix](COMPATIBILITY.md) and
[P0.2 review checklist](P0_2_MANUAL_REVIEW.md) record the remaining work.
Likewise, the 77 Atlas visual scenarios and 97 template states measure rendered
coverage rather than complete behavior or native accessibility.
