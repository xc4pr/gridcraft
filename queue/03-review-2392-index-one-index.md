kind: review
repo: storytold/gridcraft
pr: 2392
commit: 74b2bda75c4770148be7b9ba5605dd6b2c579753 (PR head, merge of main b8771d51 into the fix commit 6db94d15)
event: COMMENT

# Body

I checked out the head of this PR (74b2bda) and built and tested it locally on Linux. Results:

- `cargo test -p gridcraft-calc --lib`: 80 passed, 1 ignored (same ignored test as on main).
- I put `crates/calc/src/eval.rs` back to main's version and kept the new test. `index_with_one_index` then fails at its first 2-D case, `=TYPE(INDEX(A5:C7,2))` (left 64, right 16). So the test catches a revert.
- `cargo test -p gridcraft-engine -p gridcraft-xlsx -p gridcraft-functions -p gridcraft-cli` on the PR head: all pass (405 + 250 + 137 + 80 + 25 + 11, nothing failed). No existing workbook test depended on the old row result.
- `git merge-tree` against current main (b8771d5): merges cleanly.
- Spot checks with `gridcraft-cli eval` on the PR build:
  - `INDEX(A1:C3,2)` and `INDEX(A1:C3,1)`: `#REF!`
  - `INDEX(A1:C3,2,)` and `INDEX(A1:C3,2,0)`: the row; `COLUMNS(INDEX(A1:C3,2,0))` is 3
  - `INDEX(A1:A3,2)`, `INDEX(A1:C1,2)`, `INDEX(A1:C3,,2)`: single cell or column, unchanged
  - `INDEX({1,2;3,4},2)`: still the row `3 4`
  - `INDEX((A1:C3,E1:E3),2,,2)`: picks along the 1-D second area

The new check sits before the existing single-row swap, and `two_args` is only true when the column argument is omitted, not when it is empty. That matches the description.

One question, not blocking: the `#REF!` list in the test includes `=INDEX(A5:C7,0)`, and `ROWS(INDEX(A1:C3,0))` is now `#REF!` too, where main returned the whole range. The description and code comment talk about `INDEX(A1:C3,2)`. Was the row 0 case also observed in Excel? If so, a short note in the code comment that 0 counts as a single index too would help whoever touches this next.
