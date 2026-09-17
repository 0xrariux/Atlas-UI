# P0.2 template review

Use this checklist with real consumer templates after the automated gate. Test
each supported renderer and platform at normal and enlarged text scales.

1. Open a menu, popover, combobox, modal, and drawer with pointer and keyboard.
   Check visible focus, labels, enabled state, Escape, Tab, and Shift+Tab.
2. Open modal → popover → menu both sequentially and in one update. Close one
   layer at a time and check that focus returns to its live invoking control.
3. Place a popover near every viewport edge, then resize the window and change
   its content size. Check flip, shift, clipping, and any overlap with the
   anchor. Use the host's `place_overlay` result to drive its panel position.
   Repeat with combobox and autocomplete `menu-x`/`menu-y` bindings.
4. Remove, disable, or hide an anchor while its menu, popover, tooltip, or
   combobox is open. Check closure and the host's fallback focus destination.
5. Select combobox entries with pointer, arrow keys, Enter, and Space; check
   Escape and Tab exit. Remove and reorder options while open, including the
   active option, and check the host's active-ID settlement. Check the field's
   expanded accessibility state.
6. Type continuously into autocomplete while suggestions are open. Use arrows,
   Enter, Escape, Tab, and Shift+Tab; check caret movement with Home/End,
   the selected suggestion, focus order, and expanded accessibility state.
   Repeat with an input method editor and results changing during composition.
7. Select radio choices with pointer, Enter, and Space. Use the host's arrow
   navigation callback to skip disabled choices and verify the focused radio
   follows `focus-index(index)` after model changes.
8. Inspect native accessibility trees and a screen reader: names, descriptions,
   checked/expanded states, item counts, focus order, and announcement timing.
   The software-window fixtures do not establish backend parity.

Record the template, viewport, platform, renderer, input method, accessibility
tool, and screenshot or recording for each discrepancy. The remaining P0.2
work is tracked in [ROADMAP.md](../ROADMAP.md).
