# Host-owned collection adapters

Atlas keeps collection identity and domain models in Rust. Use
`atlas_ui::core::collections::CollectionState<Id>` to retain selected,
expanded, and focused IDs across row insertion, reordering, filtering, and
paging. Call `retain_existing` with the **complete** source ID set only when
items are removed from the underlying model; passing a filtered page would
discard hidden state. `focused_index` resolves an ID against the current
visible row order.

`page_window` clamps a requested zero-based page to the available range.
`group_ranges` returns contiguous boundaries without sorting the host model.
`project_cells` aligns values to the current column IDs, leaving absent cells
empty instead of shifting values into the wrong column. `flatten_visible`
borrows tree values and walks only expanded branches. The host converts its
visible projection to the preview `TreeViewRow` Slint model and handles
`AtlasTreeView` callbacks by stable ID.

For a mutable tree, `TreeProjection` indexes nodes by ID. `expand` inserts
only the newly visible branch and `collapse` removes only its contiguous
descendants. `replace` validates unique IDs, retains expansion for surviving
nodes, and rebuilds visible rows once after a complete source mutation. Use
`visible()` to build the Slint model and `value(id)` to access host data without
copying it into presentation state.

```rust
use atlas_ui::core::collections::{CollectionState, page_window};
use std::num::NonZeroUsize;

let mut state = CollectionState::new();
state.select("node-42", true);
state.focus(Some("node-42"));
let page = page_window(125, 3, NonZeroUsize::new(25).unwrap());
assert_eq!((page.start, page.end), (75, 100));
```

`AtlasTreeView` renders host-projected rows with fixed row height and emits
`selection-requested`, `expansion-requested`, and `focus-requested`. The host
must update the row model after disclosure and settle `focused-index` after
model mutations. Disabled rows cannot be selected or expanded. The component
uses list/list-item accessibility roles because Slint 1.18 does not expose a
complete native tree relationship contract.
