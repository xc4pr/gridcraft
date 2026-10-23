kind: pr
repo: storytold/gridcraft
base: main
head: xc4pr:fix/2335-table-one-value-array
commit: aae0086cf530249b96e6ea55c955ee72f5b8fa1a (on upstream main b8771d51)
issue: #2335 (exists)

# Title

Calc: a dynamic array of one value in a table is its value, not #SPILL! (#2335)

# Body

## Summary

A dynamic-array formula in a table column that returns one value (the usual `FILTER(T_Data[Owner],T_Data[Key]=[@Key])` lookup) showed `#SPILL!` on main. Excel shows the value. It gives the result the report shows for 0.10.0 and keeps #1491 (`C1#` on a one-value array is still the 1 x 1 range).

## Root cause

Since #1510 (#1491) a one-value array result is recorded as a 1 x 1 spill. Three places treat any spill anchored in a table as blocked: the in-pass check (`spill_blocked`), the post-pass layout, and the `spills_as_before` shortcut in `crates/calc/src/recalc.rs`. Before #1510 a 1 x 1 result never reached them.

## Fix

A small helper, `in_table`, applies the table rule only to arrays larger than one cell. All three places use it. A table formula that spills two or more cells is still `#SPILL!`.

## Tests

- New `tests::a_dynamic_array_of_one_value_in_a_table_is_not_spill` in `crates/calc/src/tests.rs`: a FILTER matching one row and `SEQUENCE(1)` in a table, a dependent formula, an edit, a full recalculation, and `SEQUENCE(2)` still `#SPILL!`.
- `cargo test -p gridcraft-calc --lib -- in_a_table` on main without the fix: fails, `left: Error(Spill)`, `right: Text("East")`.
- `cargo test -p gridcraft-calc --lib` with the fix: 80 passed, 1 ignored (the same one as on main).
- End to end with `gridcraft-cli run`: two tables (`T_Data`, `T_Main`) and the FILTER from the report in `T_Main[Owner]`. Before: `#SPILL!` three times. After: South, East, North. `gridcraft-cli check` on the saved file: 3 matched, 0 mismatched, round trip unchanged.
- `cargo xtask ci` on Linux: i18n-report, fmt, clippy `-D warnings` pass. The test step stops at `header_autofit::table_filter_autofit_works_through_command_and_header_double_click` (gridcraft-ui-egui), which fails the same way on unmodified main here (font-dependent, #2286, fix in #2269). The rest run separately: `cargo test --workspace --no-fail-fast -- --skip table_filter_autofit_works_through_command_and_header_double_click` passes, and `cargo xtask assets`, `layers`, `wasm`, `parity --check` and `keytips --check` pass.

Fixes #2335
