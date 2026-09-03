# Distribution and registry releases

Atlas has a local facade crate named `atlas-ui`. It exposes Rust packages and a
registry-safe `slint_library_paths()` helper, allowing consumer markup to use
named imports rather than paths tied to the Atlas monorepo.

## Current state

- Atlas `0.1.0` is distributed through seven crates.io library packages and a
  matching tagged GitHub source release. The gallery, tooling, and
  getting-started application remain `publish = false`.
- Internal path dependencies also declare the exact `0.1.0` version required by
  registry packaging.
- The facade has a description, README, license, keywords, and category.
- Repository metadata points to `https://github.com/0xrariux/Atlas-UI`, and every
  public crate declares its intended versioned docs.rs URL.
- The release contains 77 deterministic visual scenarios across 32 pages.

The canonical user dependency is the exact crates.io release:

```toml
atlas-ui = "=0.1.0"
```

Cargo downloads the packaged Slint facades and assets automatically; consumers
do not need a manual Atlas checkout. The `v0.1.0` Git tag remains the
corresponding auditable source snapshot.

## Publication order

Publish every release in dependency order:

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
