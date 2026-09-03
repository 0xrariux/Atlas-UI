# Distribution and registry preparation

Atlas has a local facade crate named `atlas-ui`. It exposes Rust packages and a
registry-safe `slint_library_paths()` helper, allowing consumer markup to use
named imports rather than paths tied to the Atlas monorepo.

## Current state

- Atlas `v0.1.0` is distributed as a tagged GitHub release. The seven library
  crates remain configured for a later crates.io publication; the gallery and
  getting-started application remain `publish = false`.
- Internal path dependencies also declare the exact `0.1.0` version required by
  registry packaging.
- The facade has a description, README, license, keywords, and category.
- Repository metadata points to `https://github.com/0xrariux/Atlas-UI`, and every
  public crate declares its intended versioned docs.rs URL.
- The release contains 77 deterministic visual scenarios across 32 pages.

The canonical user dependency is the `v0.1.0` Git tag:

```toml
atlas-ui = { git = "https://github.com/0xrariux/Atlas-UI", tag = "v0.1.0" }
```

Versioned docs.rs links remain pending until the crates are published there.

## Planned publication order

Publish in dependency order:

1. `atlas-ui-tokens`;
2. `atlas-ui-core`;
3. `atlas-ui-icons`;
4. `atlas-ui-testing`;
5. `atlas-ui-documents`;
6. `atlas-ui-components`;
7. `atlas-ui`.

Applications and examples should remain unpublished.

## Publication checklist

Before publishing:

1. Confirm ownership and availability of every crate name.
2. Review `cargo package --list` for every public crate.
3. Run `cargo package` and test the packaged sources, not only the workspace.
4. Authenticate to crates.io and publish in dependency order.
5. Verify every crates.io page and docs.rs build before announcing the release.

Registry publication is intentionally not automated by the repository's local
quality gate.
