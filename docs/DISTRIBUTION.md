# Distribution and registry preparation

Atlas has a local facade crate named `atlas-ui`. It exposes Rust packages and a
registry-safe `slint_library_paths()` helper, allowing consumer markup to use
named imports rather than paths tied to the Atlas monorepo.

## Current state

- Every Atlas crate remains `publish = false`.
- Internal path dependencies also declare the exact `0.2.0` version required by
  registry packaging.
- The facade has a description, README, license, keywords, and category.
- Repository and documentation URLs are intentionally absent until the final
  public GitHub owner and canonical URLs are known.
- The release readiness check remains blocked by unapproved visual baselines.

No agent should claim that Atlas is available from crates.io or docs.rs while
this state remains in effect.

## Planned publication order

After the repository URL is known and release evidence is approved, publish in
dependency order:

1. `atlas-ui-tokens`;
2. `atlas-ui-core` and `atlas-ui-icons`;
3. `atlas-ui-components`, `atlas-ui-documents`, and `atlas-ui-testing` as
   applicable;
4. `atlas-ui`.

Applications and examples should remain unpublished.

## Required human decisions

Before enabling publication:

1. Confirm ownership and availability of every crate name, especially
   `atlas-ui`.
2. Choose the canonical public repository URL.
3. Add `repository` and, when applicable, `documentation` or `homepage`
   metadata.
4. Decide which implementation and testing crates are supported as public
   packages rather than packaging details.
5. Approve all required visual baselines.
6. Review `cargo package --list` for every public crate.
7. Run `cargo package` and test the packaged sources, not only the workspace.
8. Authenticate to crates.io and publish manually in dependency order.

Registry publication is intentionally not automated by the repository's local
quality gate.
