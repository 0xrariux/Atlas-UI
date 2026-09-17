# Atlas UI documentation

Start with the [repository README](../README.md) for the project and its
current release status. The published v0.1.1 crate uses Slint 1.17.1; the
0.2.0 release candidate uses Slint 1.18.0.

## Build an application

1. [Getting started](../GETTING_STARTED.md) — choose the matching Atlas and
   Slint versions, then compile a first view.
2. [Component catalog](COMPONENTS.md) — find components and their stable or
   preview contracts.
3. [Template applications](https://github.com/0xrariux/template-atlas) — browse
   complete native application examples.

Use the [compatibility matrix](COMPATIBILITY.md) before claiming support for a
platform or renderer. The [Slint integration guide](SLINT_INTEGRATION.md)
explains which behavior belongs to Atlas and which belongs to Slint.

## Work with a coding agent

- [Agent quickstart](AGENT_QUICKSTART.md) — dependency setup and implementation
  procedure.
- [Component index](AGENT_COMPONENT_INDEX.md) — select an API by task.
- [Visual workflow](AGENT_VISUAL_WORKFLOW.md) — provide a product reference and
  inspect rendered consumer output.
- [Integration guide](AI_INTEGRATION_GUIDE.md) — host-side callbacks and
  examples.
- [API manifest guide](AGENT_MANIFEST.md) and
  [machine-readable manifest](atlas-ui-agent-manifest.json) — exact symbols and
  signatures.
- [Agent evaluation kit](../evals/agent-discovery/README.md) — discovery tests.

## Understand and validate Atlas

- [Architecture](ARCHITECTURE.md) — workspace layers and ownership.
- [Engineering](ENGINEERING.md) and [tooling](TOOLING.md) — component contracts
  and quality gates.
- [Component evidence](COMPONENT_EVIDENCE.md) — runtime and input coverage,
  with platform-validation limits.
- [External consumer scenarios](EXTERNAL_CONSUMER_SCENARIOS.md) — the four
  template suites and their 97 captured states.
- [Distribution](DISTRIBUTION.md) and [publication policy](PUBLICATION_POLICY.md)
  — packaging and public-source boundaries.

## Roadmap and focused references

The [roadmap](../ROADMAP.md) opens with the current priorities. The
[technology watchlist](../TECHNOLOGY_WATCHLIST.md) and
[Slint 1.18 audit](SLINT_1_18_AUDIT.md) record upstream and migration evidence.

For focused implementation work, see [P0 foundations](P0_FOUNDATIONS.md),
[layout review](P0_1_MANUAL_REVIEW.md), [interaction review](P0_2_MANUAL_REVIEW.md),
[track allocation](TRACK_ALLOCATION.md), [overlay placement](OVERLAY_PLACEMENT.md),
[collection adapters](COLLECTION_ADAPTERS.md), and
[binary efficiency](BINARY_EFFICIENCY.md). The
[Talos consumer audit](TALOS_CONSUMER_GAP_AUDIT.md) records an earlier external
integration review.
