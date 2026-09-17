//! Host-owned collection identity, paging, and tree projection.
//!
//! These adapters keep domain models in Rust. Slint receives only the visible
//! rows and emits intentions identified by stable IDs.

use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroUsize;

/// Selection, focus, and disclosure keyed by item identity rather than row index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionState<Id: Ord> {
    selected: BTreeSet<Id>,
    expanded: BTreeSet<Id>,
    focused: Option<Id>,
}

impl<Id: Ord> Default for CollectionState<Id> {
    fn default() -> Self {
        Self {
            selected: BTreeSet::new(),
            expanded: BTreeSet::new(),
            focused: None,
        }
    }
}

impl<Id: Ord + Clone> CollectionState<Id> {
    /// Creates an empty collection state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the selected IDs, including IDs hidden by the current filter.
    #[must_use]
    pub fn selected(&self) -> &BTreeSet<Id> {
        &self.selected
    }

    /// Returns the expanded tree-node IDs.
    #[must_use]
    pub fn expanded(&self) -> &BTreeSet<Id> {
        &self.expanded
    }

    /// Returns the focused ID, if any.
    #[must_use]
    pub fn focused(&self) -> Option<&Id> {
        self.focused.as_ref()
    }

    /// Resolves the focused ID against the current sorted, filtered, or paged rows.
    /// Hidden focus remains stored and becomes visible again when its row returns.
    #[must_use]
    pub fn focused_index(&self, visible_ids: &[Id]) -> Option<usize> {
        self.focused
            .as_ref()
            .and_then(|focused| visible_ids.iter().position(|id| id == focused))
    }

    /// Updates selection without depending on the current visual row index.
    pub fn select(&mut self, id: Id, selected: bool) {
        if selected {
            self.selected.insert(id);
        } else {
            self.selected.remove(&id);
        }
    }

    /// Updates disclosure state for a tree node.
    pub fn expand(&mut self, id: Id, expanded: bool) {
        if expanded {
            self.expanded.insert(id);
        } else {
            self.expanded.remove(&id);
        }
    }

    /// Sets the focused item by identity.
    pub fn focus(&mut self, id: Option<Id>) {
        self.focused = id;
    }

    /// Drops state only for IDs absent from the complete host model.
    ///
    /// Pass the full set before filtering or paging, so hidden items retain
    /// their selection and disclosure state.
    pub fn retain_existing(&mut self, ids: impl IntoIterator<Item = Id>) {
        let existing: BTreeSet<Id> = ids.into_iter().collect();
        self.selected.retain(|id| existing.contains(id));
        self.expanded.retain(|id| existing.contains(id));
        if self
            .focused
            .as_ref()
            .is_some_and(|id| !existing.contains(id))
        {
            self.focused = None;
        }
    }
}

/// A bounded page in a host-owned collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageWindow {
    /// Zero-based page after clamping to the available range.
    pub index: usize,
    /// Number of pages, zero for an empty collection.
    pub page_count: usize,
    /// Inclusive start index of the visible page.
    pub start: usize,
    /// Exclusive end index of the visible page.
    pub end: usize,
}

/// Computes a page without copying or mutating the host model.
#[must_use]
pub fn page_window(total: usize, requested_page: usize, page_size: NonZeroUsize) -> PageWindow {
    if total == 0 {
        return PageWindow {
            index: 0,
            page_count: 0,
            start: 0,
            end: 0,
        };
    }
    let size = page_size.get();
    let page_count = total / size + usize::from(!total.is_multiple_of(size));
    let index = requested_page.min(page_count - 1);
    let start = index * size;
    PageWindow {
        index,
        page_count,
        start,
        end: start.saturating_add(size).min(total),
    }
}

/// One contiguous group in a sorted or host-ordered collection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupRange<Key> {
    /// Host-defined group identity.
    pub key: Key,
    /// Inclusive first row.
    pub start: usize,
    /// Exclusive row boundary.
    pub end: usize,
}

/// Finds contiguous groups while retaining the host's row order.
#[must_use]
pub fn group_ranges<T, Key: Eq + Clone>(
    rows: &[T],
    mut key_of: impl FnMut(&T) -> Key,
) -> Vec<GroupRange<Key>> {
    let mut groups: Vec<GroupRange<Key>> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let key = key_of(row);
        if let Some(last) = groups.last_mut()
            && last.key == key
        {
            last.end = index + 1;
            continue;
        }
        groups.push(GroupRange {
            key,
            start: index,
            end: index + 1,
        });
    }
    groups
}

/// Projects a row's cells into the current column order by stable column ID.
///
/// Missing values remain `None`; insertion and reordering do not shift data
/// into the wrong column.
#[must_use]
pub fn project_cells<'a, Id: Ord, Cell>(
    columns: &[Id],
    cells: &'a BTreeMap<Id, Cell>,
) -> Vec<Option<&'a Cell>> {
    columns.iter().map(|id| cells.get(id)).collect()
}

/// A host-owned tree node. IDs must be unique within a projected tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeNode<Id, T> {
    /// Stable identity of the node.
    pub id: Id,
    /// Domain data retained by the host.
    pub value: T,
    /// Child nodes in host-defined order.
    pub children: Vec<Self>,
}

/// A borrowed row for a visible tree node.
#[derive(Clone, Copy, Debug)]
pub struct VisibleNode<'a, Id, T> {
    /// Stable identity of the node.
    pub id: &'a Id,
    /// Host-owned node data.
    pub value: &'a T,
    /// Zero-based depth for indentation.
    pub depth: usize,
    /// Whether the node can be expanded.
    pub has_children: bool,
}

/// Flattens only branches expanded by ID, borrowing values from the host tree.
///
/// Sorting or insertion ahead of a node does not change its expansion state.
#[must_use]
pub fn flatten_visible<'a, Id: Ord, T>(
    roots: &'a [TreeNode<Id, T>],
    expanded: &BTreeSet<Id>,
) -> Vec<VisibleNode<'a, Id, T>> {
    let mut visible = Vec::new();
    let mut stack: Vec<(usize, &'a TreeNode<Id, T>)> =
        roots.iter().rev().map(|node| (0, node)).collect();
    while let Some((depth, node)) = stack.pop() {
        visible.push(VisibleNode {
            id: &node.id,
            value: &node.value,
            depth,
            has_children: !node.children.is_empty(),
        });
        if expanded.contains(&node.id) {
            stack.extend(node.children.iter().rev().map(|child| (depth + 1, child)));
        }
    }
    visible
}

/// A visible tree row with stable identity and indentation depth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisibleTreeRow<Id> {
    /// Stable node ID.
    pub id: Id,
    /// Zero-based depth in the host tree.
    pub depth: usize,
    /// Whether disclosure is available.
    pub has_children: bool,
}

#[derive(Clone, Debug)]
struct StoredTreeNode<Id, T> {
    value: T,
    children: Vec<Id>,
}

/// Invalid host tree input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeProjectionError<Id> {
    /// Two nodes share one stable ID.
    DuplicateId(Id),
}

/// ID-indexed visible-tree projection with incremental disclosure updates.
///
/// Expanding inserts only the newly visible branch; collapsing removes its
/// contiguous descendants. Replacing the source tree preserves expansion for
/// IDs that still exist and rebuilds visible rows once.
#[derive(Clone, Debug)]
pub struct TreeProjection<Id: Ord, T> {
    nodes: BTreeMap<Id, StoredTreeNode<Id, T>>,
    roots: Vec<Id>,
    expanded: BTreeSet<Id>,
    visible: Vec<VisibleTreeRow<Id>>,
}

impl<Id: Ord + Clone, T> TreeProjection<Id, T> {
    fn insert_node(
        node: TreeNode<Id, T>,
        nodes: &mut BTreeMap<Id, StoredTreeNode<Id, T>>,
    ) -> Result<Id, TreeProjectionError<Id>> {
        let id = node.id;
        if nodes.contains_key(&id) {
            return Err(TreeProjectionError::DuplicateId(id));
        }
        let children = node
            .children
            .into_iter()
            .map(|child| Self::insert_node(child, nodes))
            .collect::<Result<Vec<_>, _>>()?;
        if nodes.contains_key(&id) {
            return Err(TreeProjectionError::DuplicateId(id));
        }
        nodes.insert(
            id.clone(),
            StoredTreeNode {
                value: node.value,
                children,
            },
        );
        Ok(id)
    }

    /// Builds a projection from a complete host-owned tree.
    ///
    /// # Errors
    /// Returns [`TreeProjectionError::DuplicateId`] if the input repeats an ID.
    pub fn new(roots: Vec<TreeNode<Id, T>>) -> Result<Self, TreeProjectionError<Id>> {
        let mut nodes = BTreeMap::new();
        let roots = roots
            .into_iter()
            .map(|node| Self::insert_node(node, &mut nodes))
            .collect::<Result<Vec<_>, _>>()?;
        let mut projection = Self {
            nodes,
            roots,
            expanded: BTreeSet::new(),
            visible: Vec::new(),
        };
        projection.rebuild_visible();
        Ok(projection)
    }

    fn collect_branch(&self, id: &Id, depth: usize, output: &mut Vec<VisibleTreeRow<Id>>) {
        let node = &self.nodes[id];
        output.push(VisibleTreeRow {
            id: id.clone(),
            depth,
            has_children: !node.children.is_empty(),
        });
        if self.expanded.contains(id) {
            for child in &node.children {
                self.collect_branch(child, depth + 1, output);
            }
        }
    }

    fn rebuild_visible(&mut self) {
        let mut rows = Vec::new();
        for id in &self.roots {
            self.collect_branch(id, 0, &mut rows);
        }
        self.visible = rows;
    }

    /// Returns the current rows in host-defined order.
    #[must_use]
    pub fn visible(&self) -> &[VisibleTreeRow<Id>] {
        &self.visible
    }

    /// Returns domain data for an ID without copying it into the view model.
    #[must_use]
    pub fn value(&self, id: &Id) -> Option<&T> {
        self.nodes.get(id).map(|node| &node.value)
    }

    /// Returns true if a node is currently expanded, even when hidden by an ancestor.
    #[must_use]
    pub fn is_expanded(&self, id: &Id) -> bool {
        self.expanded.contains(id)
    }

    /// Opens a branch, inserting its visible descendants without rebuilding siblings.
    pub fn expand(&mut self, id: &Id) -> bool {
        if self
            .nodes
            .get(id)
            .is_none_or(|node| node.children.is_empty())
            || !self.expanded.insert(id.clone())
        {
            return false;
        }
        if let Some(position) = self.visible.iter().position(|row| &row.id == id) {
            let depth = self.visible[position].depth + 1;
            let mut inserted = Vec::new();
            for child in &self.nodes[id].children {
                self.collect_branch(child, depth, &mut inserted);
            }
            let insertion = position + 1;
            self.visible.splice(insertion..insertion, inserted);
        }
        true
    }

    /// Closes a branch, removing only its contiguous visible descendants.
    pub fn collapse(&mut self, id: &Id) -> bool {
        if !self.expanded.remove(id) {
            return false;
        }
        if let Some(position) = self.visible.iter().position(|row| &row.id == id) {
            let depth = self.visible[position].depth;
            let end = self.visible[position + 1..]
                .iter()
                .position(|row| row.depth <= depth)
                .map_or(self.visible.len(), |offset| position + 1 + offset);
            self.visible.drain(position + 1..end);
        }
        true
    }

    /// Replaces the complete source tree and retains expansion for surviving IDs.
    ///
    /// # Errors
    /// Returns [`TreeProjectionError::DuplicateId`] without changing the
    /// current projection if the replacement repeats an ID.
    pub fn replace(&mut self, roots: Vec<TreeNode<Id, T>>) -> Result<(), TreeProjectionError<Id>> {
        let mut replacement = Self::new(roots)?;
        replacement.expanded = self
            .expanded
            .iter()
            .filter(|id| {
                replacement
                    .nodes
                    .get(*id)
                    .is_some_and(|node| !node.children.is_empty())
            })
            .cloned()
            .collect();
        replacement.rebuild_visible();
        *self = replacement;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CollectionState, TreeNode, TreeProjection, TreeProjectionError, flatten_visible,
        group_ranges, page_window, project_cells,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use std::num::NonZeroUsize;

    #[test]
    fn identity_survives_reorder_filter_and_paging() {
        let mut state = CollectionState::new();
        state.select("b", true);
        state.expand("b", true);
        state.focus(Some("b"));
        assert_eq!(page_window(5, 99, NonZeroUsize::new(2).unwrap()).start, 4);
        state.retain_existing(["c", "b", "a"]);
        assert!(state.selected().contains("b"));
        assert!(state.expanded().contains("b"));
        assert_eq!(state.focused(), Some(&"b"));
        assert_eq!(state.focused_index(&["c", "b", "a"]), Some(1));
        assert_eq!(state.focused_index(&["c", "a"]), None);
        state.retain_existing(["c", "a"]);
        assert!(state.selected().is_empty());
        assert!(state.expanded().is_empty());
        assert_eq!(state.focused(), None);
    }

    #[test]
    fn tree_projection_skips_collapsed_branches_without_copying_values() {
        let nodes = vec![TreeNode {
            id: "root",
            value: "Root",
            children: vec![TreeNode {
                id: "child",
                value: "Child",
                children: vec![TreeNode {
                    id: "leaf",
                    value: "Leaf",
                    children: vec![],
                }],
            }],
        }];
        let collapsed = flatten_visible(&nodes, &BTreeSet::default());
        assert_eq!(
            collapsed.iter().map(|node| *node.id).collect::<Vec<_>>(),
            ["root"]
        );
        let expanded = ["root", "child"].into_iter().collect();
        let visible = flatten_visible(&nodes, &expanded);
        assert_eq!(
            visible
                .iter()
                .map(|node| (*node.id, node.depth))
                .collect::<Vec<_>>(),
            [("root", 0), ("child", 1), ("leaf", 2)]
        );
    }

    #[test]
    fn grouping_and_cell_projection_preserve_host_identity() {
        let groups = group_ranges(&["a1", "a2", "b1", "a3"], |row| &row[..1]);
        assert_eq!(
            groups
                .iter()
                .map(|group| (group.key, group.start, group.end))
                .collect::<Vec<_>>(),
            [("a", 0, 2), ("b", 2, 3), ("a", 3, 4)]
        );
        let cells = BTreeMap::from([("name", "Atlas"), ("status", "Ready")]);
        assert_eq!(
            project_cells(&["status", "region", "name"], &cells),
            [Some(&"Ready"), None, Some(&"Atlas")]
        );
    }

    #[test]
    fn tree_projection_updates_visible_branches_and_preserves_ids() {
        let source = vec![
            TreeNode {
                id: "root",
                value: "Root",
                children: vec![TreeNode {
                    id: "child",
                    value: "Child",
                    children: vec![TreeNode {
                        id: "leaf",
                        value: "Leaf",
                        children: vec![],
                    }],
                }],
            },
            TreeNode {
                id: "sibling",
                value: "Sibling",
                children: vec![],
            },
        ];
        let mut tree = TreeProjection::new(source).unwrap();
        assert_eq!(
            tree.visible().iter().map(|row| row.id).collect::<Vec<_>>(),
            ["root", "sibling"]
        );
        assert!(tree.expand(&"root"));
        assert!(tree.expand(&"child"));
        assert_eq!(
            tree.visible().iter().map(|row| row.id).collect::<Vec<_>>(),
            ["root", "child", "leaf", "sibling"]
        );
        assert!(tree.collapse(&"root"));
        assert_eq!(
            tree.visible().iter().map(|row| row.id).collect::<Vec<_>>(),
            ["root", "sibling"]
        );
        assert!(tree.is_expanded(&"child"));
        assert!(tree.expand(&"root"));
        assert_eq!(
            tree.visible().iter().map(|row| row.id).collect::<Vec<_>>(),
            ["root", "child", "leaf", "sibling"]
        );
        assert_eq!(tree.value(&"leaf"), Some(&"Leaf"));

        tree.replace(vec![
            TreeNode {
                id: "sibling",
                value: "Sibling",
                children: vec![],
            },
            TreeNode {
                id: "root",
                value: "Root",
                children: vec![TreeNode {
                    id: "child",
                    value: "Child",
                    children: vec![],
                }],
            },
        ])
        .unwrap();
        assert_eq!(
            tree.visible().iter().map(|row| row.id).collect::<Vec<_>>(),
            ["sibling", "root", "child"]
        );
        assert!(tree.is_expanded(&"root"));
    }

    #[test]
    fn duplicate_tree_ids_leave_existing_projection_intact() {
        let mut tree = TreeProjection::new(vec![TreeNode {
            id: "root",
            value: 1,
            children: vec![],
        }])
        .unwrap();
        let duplicate = vec![TreeNode {
            id: "root",
            value: 2,
            children: vec![TreeNode {
                id: "root",
                value: 3,
                children: vec![],
            }],
        }];
        assert_eq!(
            tree.replace(duplicate),
            Err(TreeProjectionError::DuplicateId("root"))
        );
        assert_eq!(tree.value(&"root"), Some(&1));
    }
}
