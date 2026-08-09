# Publication policy

Atlas keeps its public source tree reproducible, reviewable, and usable without
private project context. English is the official and only language for
project-authored prose in every published source file, document, example, test,
fixture, and contribution form. Deliberate internationalization and Unicode test
data may use other scripts when the language itself is part of the test.

## Published material

The repository is intended to publish the Rust and Slint workspace, examples,
gallery, documentation, canonical visual scenarios and baselines, public
automation scripts, issue forms, license material, and shared Cargo
configuration. `Cargo.lock` is intentionally versioned because the workspace
contains applications and executable examples.

Public documentation and checks must not depend on an ignored file. Coding
agents can start from `AGENTS.md`, the component index, and the machine-readable
manifest without access to private planning history.

## Local-only material

The following categories remain local and must not be committed:

- private planning and agent workspaces (`ai/`, `.agents/`, and `.codex/`);
- machine-specific editor, IDE, Cargo, and environment configuration;
- credentials, secrets, signing material, and local environment files;
- build output, caches, logs, coverage output, and temporary files;
- generated screenshot results, diffs, performance runs, and review reports;
- locally assembled archives, packages, and release artifacts.

The canonical screenshot scenarios, schemas, metadata, and approved baselines
remain public because they define reproducible visual contracts. Generated
review output remains local because it is transient evidence, may contain
machine-generated prose, and is not an approved product contract.

## Release check

Run the complete gate before publishing:

```bash
sh scripts/quality-gate.sh
```

The gate checks the public language and publication boundary in addition to the
Rust, Slint, agent-manifest, and visual-scenario contracts. Before tagging a
release, also inspect the Git index itself and confirm that no ignored file was
previously force-added:

```bash
git status --short
git ls-files -ci --exclude-standard
```

The second command must produce no output.
