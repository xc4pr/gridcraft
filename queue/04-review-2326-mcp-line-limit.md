kind: review
repo: storytold/gridcraft
pr: 2326
commit: 394057247e8dee9f340cd9780dd18dd232db4c38 (PR head)
event: COMMENT

# Body

I checked out the PR head (3940572) and built and tested it locally on Linux:

- `cargo test -p gridcraft-mcp`: 23 passed, 0 failed.
- `cargo clippy -p gridcraft-mcp --all-targets -- -D warnings` and `cargo fmt -p gridcraft-mcp --check`: clean.
- `git merge-tree` against current main (b8771d5): merges cleanly.
- I ran the two new tests' binary under `getrusage`. Together they stream two 4 GiB lines, and peak RSS was about 40 MB. The bound holds, not just the capacity assertion.
- CodeRabbit asked whether `input.take(limit)` moves `input` before the later `skip_line(input)`. It doesn't: the crate compiles and the serve-loop test reads the message after the long line. The `&mut` receiver is reborrowed.

One behaviour change I could reproduce, at the exact limit. I ran a throwaway test (not pushed) that sends a valid `ping` padded to exactly 16 MiB (`MAX_LINE` bytes) and then a second `ping`:

| Line ending | main (b8771d5) | this PR |
|---|---|---|
| `\n` | `{"id":1,...,"result":{}}` | `{"id":1,...,"result":{}}` |
| `\r\n` | `{"id":1,...,"result":{}}` | `message too large (16 MB max)` |

`handle_line` measures the limit after `trim()`, but `read_message` now counts the raw bytes, `\r` included. That is an edge case, not a security problem. If you want to keep main's behaviour, reading with `take(MAX_LINE + 2)` and treating only `buf.len() > MAX_LINE + 1` without a final `\n` as over-long would leave the exact verdict to the existing check in `handle_line`. A CRLF line of exactly `MAX_LINE` would then still pass, and anything longer would still be rejected there or here.
