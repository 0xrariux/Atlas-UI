# Atlas and Slint

## Two complementary projects

Slint is the technical foundation: the `.slint` language, compiler, runtime,
windows, native rendering, properties, Rust bindings, layouts, and events.
Atlas organizes these capabilities into a reusable design system.

| Slint provides | Atlas adds |
|---|---|
| declarative UI language | shared tokens and conventions |
| runtime and renderers | visually coherent components |
| properties and callbacks | controlled contracts without silent I/O |
| layout primitives | standardized grid, recipes, and breakpoints |
| native focus and accessibility | verified keyboard flows and registries |
| Rust/Slint compilation | gallery, fixtures, baselines, and quality gate |

Atlas does not hide Slint behind an abstraction: consumers continue to write
normal Slint and can compose Atlas components with native language elements.

## Dynamic version tracking

The Slint version is pinned in `Cargo.toml` and tracked in an internal
compatibility registry. For every upstream capability, the registry describes:

- its stable, experimental, or limited status;
- affected Atlas components;
- the Atlas strategy: use, wrap, implement, or monitor;
- the expected impact of a future release.

For every candidate Slint release, Atlas follows this process:

1. Read release notes and breaking changes.
2. Update the capability registry and watchlist.
3. Identify affected components and templates.
4. Adapt or simplify wrappers that have become unnecessary.
5. Compile all facades and the consumer example.
6. Run tests, scenarios, budgets, and visual comparisons.
7. Promote a preview API only after an explicit audit.

An experimental Slint feature never enters Atlas's stable facade directly. It
remains behind a preview wrapper or a replaceable Atlas implementation.
`FlexboxLayout` in Slint 1.17.1 illustrates this rule: recipes that depend on it
remain in preview.

## Licenses

Atlas is licensed under MIT. Slint retains its own licensing options—GPLv3,
Royalty-Free, or commercial depending on use—and the final application must
select the appropriate Slint license. Atlas's MIT License does not replace
those terms.
