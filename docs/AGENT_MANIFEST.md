# Machine-readable API manifest

`docs/atlas-ui-agent-manifest.json` is the compact machine-readable contract for
tools and coding agents that consume Atlas UI. It is generated from the public
Slint source declarations and must not be edited by hand.

## Purpose

The manifest lets an agent answer these questions without scanning every Slint
file:

- Which facade exports a component?
- Is the component stable or preview?
- Which properties are available, including inherited Atlas properties?
- What are their types, directions, and explicit defaults?
- Which callbacks must the Rust host handle?
- Which enums, structs, and globals are exported?
- Where is the authoritative source declaration?
- What is the smallest valid import and component expression?

The `.slint` declaration referenced by `source` and `source_line` remains the
final authority.

## Schema overview

The root metadata records Atlas and Slint versions, public facades, ownership
boundaries, documentation entry points, symbol counts, and the recommended
agent workflow.

The `api` object contains three generated collections:

- `components`: public stable and preview components;
- `types`: public enums and structs;
- `globals`: public Atlas token and settings globals.

Each component entry contains:

| Field | Meaning |
|---|---|
| `name` | Exact public Slint symbol |
| `maturity` | `stable` or `preview` |
| `source`, `source_line` | Authoritative declaration location |
| `inherits` | Direct Slint base component |
| `properties` | Effective public properties, including Atlas inheritance |
| `callbacks` | Effective public callback contract |
| `inputs_without_explicit_default` | Inputs whose declarations do not specify a default expression |
| `minimal_example` | Minimal facade import and component expression |

Every property records `name`, `type`, `direction`, `default`, `declared_in`,
and whether it is inherited. A `null` default means the declaration has no
explicit default expression; Slint may still provide the type's implicit
default value.

Every callback records its parameter types, return type, declaration owner, and
inheritance status. Callbacks express intentions and do not transfer ownership
of external effects to Slint.

## Example queries

Find the complete `AtlasButton` signature:

```bash
jq '.api.components[] | select(.name == "AtlasButton")' \
  docs/atlas-ui-agent-manifest.json
```

List stable components:

```bash
jq -r '.api.components[] | select(.maturity == "stable") | .name' \
  docs/atlas-ui-agent-manifest.json
```

List callbacks for a component:

```bash
jq '.api.components[] | select(.name == "AtlasDataTable") | .callbacks' \
  docs/atlas-ui-agent-manifest.json
```

Find the fields of a public struct:

```bash
jq '.api.types[] | select(.name == "DataRow")' \
  docs/atlas-ui-agent-manifest.json
```

## Generation and freshness

Regenerate the manifest after changing a public Slint declaration or facade:

```bash
cargo run -p atlas-ui-tooling -- generate-agent-manifest
```

Check that the committed file is current without modifying it:

```bash
cargo run -p atlas-ui-tooling -- generate-agent-manifest --check
```

`scripts/quality-gate.sh` runs check mode automatically. It also verifies symbol
classification, source paths, signature completeness, duplicate members,
same-line declarations, and inherited Atlas properties.

## Consumer rules

An agent consuming the manifest should:

1. Select by user need, not only by component name similarity.
2. Prefer stable exports and disclose preview dependencies.
3. Treat `out` properties as observable state rather than host inputs.
4. Keep data, navigation, persistence, network, filesystem, and clipboard
   effects in Rust.
5. Use callbacks as controlled intentions.
6. Confirm uncertain behavior in the referenced source declaration.
7. Compile generated code and run the public quality gate.
