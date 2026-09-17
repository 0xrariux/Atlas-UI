# P0.1 template review

Use this checklist after the automated gate, on real consumer templates at
compact, standard, and enlarged typography scales. Record the template,
viewport, platform, renderer, scale factor, and screenshot for each difference.

1. Compare the same layouts under Slint 1.17.1 and 1.18.0. Inspect wrapped
   editorial text, validation messages, notification rows, repeated table
   cells, and long unbroken words. Classify intended measurement corrections
   before approving a new visual baseline.
2. Resize responsive screens across every breakpoint in both directions.
   Check nested `AtlasSwitcher`, `AtlasAutoGrid`, `AtlasColumnGrid`, and split
   panes for overlap, clipping, empty tracks, and stale widths.
3. Exercise `AtlasContainer` with a narrow nested region inside a wide window.
   Bind nested `reference-width` explicitly to `content-width`; check the local
   `ContainerSize` and optional clipping. The child slot does not propagate a
   scoped context automatically.
4. For a table using `allocate_tracks`, bind one width model to
   `allocated-column-widths`. Resize, change density, reorder/add/remove
   columns, and inspect header/body alignment and horizontal overflow. Check
   finite maximums that leave unused viewport space. The gallery `data` and
   `data-rich` pages now provide the host-wired examples; software captures at
   960 and 1360 logical pixels cover their initial desktop states.
5. Test any use of Slint `layout-order` on interactive content. Confirm visual,
   keyboard, and accessibility-tree order all express the same intended
   sequence; the software fixture shows Tab still follows declaration order.
6. Exercise repeated and conditional grids after text, visibility, and model
   mutations, including a spanning cell. Compare geometry on each supported
   renderer and operating system.

The remaining P0.1 dependencies are scoped context and dynamic grid placement
in Slint. Atlas keeps explicit width bindings and width-based grid recipes
until those upstream contracts are available. A flex-sized pane containing a
grid whose child preferred widths depend on that grid's own measured width can
form a compiler-detected cycle. Bind `reference-width` and `basis-width` to an
independent parent-owned width before using `item-basis` for child preferred
widths; the runtime fixture now compiles and measures that recipe.
