kind: pr
repo: storytold/gridcraft
base: main
head: xc4pr:fix/2334-editor-text-valign
commit: 6a8adfd9bda955ff4669358d820d8ac0ae652e85 (on upstream main b8771d51)
issue: #2334 (exists)

# Title

In-cell editor: the text stays where the cell shows it (#2334)

# Body

## Summary

Starting to edit a cell no longer moves its text up. The in-cell editor now draws the text at the same height as the cell. Bottom-aligned (the default), centred and top-aligned values stay where they were, also in tall rows, with Wrap Text, with Justify and for two lines from a line break.

## Root cause

`in_cell_editor` in `crates/ui-egui/src/grid.rs` always laid the field out from the top of the cell (`erect.shrink2(vec2(2.0, 1.0))`). The grid draws the value with `cell_text_y` / `wrapped_cell_text_y`, at the bottom for the default alignment. Measured with a probe at the default zoom: B3 is shown at y 248.6 and edited at y 248.0. That is 0.6 pt, about one device pixel on a 2x display, which matches the report from macOS. In a tall row the text jumped to the top of the cell (392.6 shown, 248.0 edited). Two lines from a line break, in their auto-fitted row, moved up by 2.6 pt.

## Fix

While the cell shows the whole text, the field starts where the cell's text starts. It uses the same `cell_text_y` / `wrapped_cell_text_y` call as the grid, with the editor's own galley height. The wrapped one is used for wrapped and justified text, under the same condition as the grid. Text taller than the cell still starts at the top, as before. The backdrop and the editor's size are unchanged.

Not in this PR: the editor's 2 pt left inset differs from the cell's text padding (2.5 pt times the zoom, plus any indent) by about 1 pt horizontally. That could be a follow-up.

Possible textual conflicts on rebase: open PRs #1821 (editor fill) and #2143 (phonetic text offset) touch neighbouring lines.

## Tests

- New `crates/ui-egui/tests/it/in_cell_editor_text_position.rs`. It compares the top of B3's text shape before and while editing, in seven cases: default row, tall row bottom-aligned, tall row middle- and top-aligned, tall row with Wrap Text, tall row justified, two lines from a line break in their auto-fitted row, and 200% zoom.
- With main's `grid.rs`, all 7 fail (for example `tall row, bottom: shown at y 328.6, edited at y 248`, `default row: shown at y 248.6, edited at y 248` and `line break, fitted row: shown at y 250.59999, edited at y 248`). With the change, all 7 pass. The tolerance is 0.1 pt because egui snaps the field's text to 1/16 pt.
- `cargo test -p gridcraft-ui-egui --test it -- in_cell_editor`: 13 passed (the size and wrap tests included).
- Offscreen render at 2x with a tall row 3 (B3 bottom, C3 middle, D3 top): before, the edited "Bottom" moves to the top of the cell. After, it stays at the bottom, and "Middle" stays centred.
- `cargo xtask ci` on Linux: i18n-report, fmt, clippy `-D warnings` pass. The test step stops at `header_autofit::table_filter_autofit_works_through_command_and_header_double_click`, which fails the same way on unmodified main here (font-dependent, #2286, fix in #2269). The rest run separately: `cargo test --workspace --no-fail-fast -- --skip table_filter_autofit_works_through_command_and_header_double_click` gives 2193 passed, 0 failed, and `cargo xtask assets`, `layers`, `wasm`, `parity --check` and `keytips --check` pass.

Fixes #2334
