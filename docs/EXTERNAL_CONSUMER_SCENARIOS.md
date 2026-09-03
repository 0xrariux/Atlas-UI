# External consumer scenarios

Atlas keeps its canonical component and composition scenarios inside this
repository. Complete applications remain in the companion
[`template-atlas`](https://github.com/0xrariux/template-atlas) repository so
that dependency direction and product ownership stay realistic.

The four template applications are release-level external consumer suites:

| Suite | Representative product surface | Captured states |
|---|---|---:|
| Command | Operations, analytics, services, alerts, and administration | 16 |
| Forge | Explorer, editor, source control, tasks, extensions, and settings | 26 |
| Fleet | Infrastructure, telemetry, deployments, incidents, and automation | 30 |
| Ledger | Portfolio, markets, transfers, assets, security, and settings | 25 |

Together they provide 97 native Rust + Slint integration states across four
distinct application structures. The state lists and deterministic capture
commands are owned by `template-atlas/scripts/capture-*-native.sh`.

## Validate a local Atlas upgrade

Place the repositories next to each other, then run from the Atlas root:

```text
workspace/
├── Atlas/
└── template-atlas/
```

```bash
sh scripts/template-consumer-gate.sh
```

The command compiles every template and all of its Rust targets against the
current Atlas checkout. Use an explicit path when the repositories are not
siblings:

```bash
sh scripts/template-consumer-gate.sh --template-root ../template-atlas
```

For a release or a change to tokens, layout, typography, icons, or rendered
components, capture all external states as well:

```bash
sh scripts/template-consumer-gate.sh --capture
```

Generated images are written under `target/template-consumer-captures/` and
remain local. Review all changed states. After approval, regenerate the four
tracked README previews from the companion repository and commit those changes
there; Atlas never writes product code or approved assets across the repository
boundary automatically.

## Relationship to local scenarios

The local gallery remains self-contained and is the canonical pixel-regression
contract for Atlas components, themes, densities, responsive behavior, and
accessibility fixtures. External consumer suites add upgrade evidence for
realistic application composition; they do not replace local scenarios or
inflate the local scenario count.

Consumer findings can justify an Atlas primitive only when they generalize
beyond one product. The [Talos consumer gap audit](TALOS_CONSUMER_GAP_AUDIT.md)
records that process. Its no-overflow scrollbar finding is represented in the
local interaction specimen by the `FITS · HIDDEN` state.
