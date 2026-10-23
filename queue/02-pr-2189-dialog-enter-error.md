kind: pr
repo: storytold/gridcraft
base: main
head: xc4pr:fix/2189-dialog-enter-error
commit: 2ad976a1b0eea065a1a198c89d9418ae3e2eddf5 (on upstream main b8771d51)
issue: #2189 (exists)

# Title

Dialogs: Enter on OK shows the error a failed command gives, as a click does (#2189)

# Body

## Summary

Pressing Enter in a dialog whose command fails now shows the error, as clicking OK does. Example from the issue: Rename Sheet with a name that is taken. Before, Enter left the dialog open and showed nothing. Now "That name is already taken. Try a different one." appears, and the next Enter closes the message and leaves the dialog open so the name can be fixed. Find Next with nothing to find behaves the same way.

## Root cause

As the issue describes: `ok_cancel` in `crates/ui-egui/src/dialogs.rs` treats Enter as OK, and a failing command sets `app.message`. `widgets::message_box` runs later in the same frame and closes on Enter. The Enter that ran OK was still in the frame's input, so the box closed before it was ever drawn. A click has no Enter event, so it worked.

## Fix

- New `take_enter(ui)` in `dialogs.rs`: reports whether Enter was pressed and removes the Enter key events from the frame's input (option A in the issue). It replaces the four places where Enter acts as OK: the form `ok_cancel`, Paste Special's OK, Find Next, and the command list.
- While the message box (the only `egui::Modal` in the crate) is above the dialog, `take_enter` leaves Enter alone (`Memory::allows_interaction` for the dialog's layer). Otherwise the second Enter would run OK again and bring the same message straight back, so the box could only be closed by mouse or Escape.

## Tests

- New `crates/ui-egui/tests/it/dialog_enter_error.rs` (egui_kittest, like `dialog_focus.rs`):
  - `enter_on_rename_sheet_with_a_taken_name_shows_why`
  - `enter_on_rename_sheet_with_a_free_name_still_renames`
  - `enter_on_find_next_with_nothing_to_find_shows_why` (also checks that a second Enter closes the message)
  - `enter_again_closes_the_message_and_keeps_the_dialog`
- With main's `dialogs.rs` and the new tests, `cargo test -p gridcraft-ui-egui --test it -- dialog_enter_error`: 3 of 4 fail (the message is `None`, or still shown after the second Enter). The free-name test passes, as it should.
- With the fix: 4 of 4 pass, and all 40 `dialog*` tests in that binary pass.
- `cargo test -p gridcraft-ui-egui -- --skip table_filter_autofit_works_through_command_and_header_double_click`: 761 + 151 + 25 + 2 passed, 0 failed.
- `cargo xtask ci` on Linux: i18n-report, fmt, clippy `-D warnings` pass. The test step stops at `header_autofit::table_filter_autofit_works_through_command_and_header_double_click`, which fails the same way on unmodified main here (font-dependent, #2286, fix in #2269). The rest run separately: `cargo test --workspace --no-fail-fast -- --skip table_filter_autofit_works_through_command_and_header_double_click` passes, and `cargo xtask assets`, `layers`, `wasm`, `parity --check` and `keytips --check` pass.

Note: open PR #870 also consumes Enter before setting `app.message` in two places. It is not a duplicate, but one of the two may need a small rebase.

Fixes #2189
