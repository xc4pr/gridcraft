kind: review
repo: storytold/gridcraft
pr: 2393
commit: 1ce4781d3e059e14696cd4a351f61e4242062767 (PR head)
event: COMMENT

# Body

I checked out the PR head (1ce4781) and built and tested it locally on Linux:

- `cargo test -p gridcraft-functions -p gridcraft-calc`: 139 + 80 passed, 1 ignored (the same one as on main), 0 failed.
- `git merge-tree` against current main (b8771d5): merges cleanly.
- To check the tests catch a revert, I put main's `pmt_calc` back and kept the new tests. `pmt_edge_cases_follow_excel` fails on `PMT(-2,3,100)` (100 instead of `#NUM!`), and `pmt_matches_excel_within_a_few_ulps` fails on its first probe.
- The examples from Microsoft's function pages still come out right through `gridcraft-cli eval`, so the functions built on `pmt_calc` are fine:
  - `PMT(0.08/12,10,10000)` = -1037.0320893591522
  - `IPMT(0.1/12,1,36,8000)` = -66.66666666666667
  - `PPMT(0.1/12,1,24,2000)` = -75.62318600836633
  - `CUMIPMT(0.09/12,360,125000,13,24,0)` = -11135.232130750841
- The algebra of the three branches checks out: `fv + pv*p` over `p - 1`, with `p = 1 + m`.

On the overflow CodeRabbit flagged and the description mentions: I confirmed `PMT(1e-300,10,1e12)` is `#NUM!` with this PR. Main gives `#NUM!` too, since `p - 1` is 0 there, so this is not a regression. The first branch can avoid it by dividing `rate` by `m` before multiplying:

```rust
-(pv * rate + (pv + fv) * (rate / m)) / k
```

`rate / m` is about `1 / nper` for a tiny rate, so nothing overflows. I tried it locally:

- `PMT(1e-300,10,1e12)` = -100000000000
- `PMT(0.05,10,100)` and `PMT(0.08/12,10,10000)` are unchanged.
- Both new tests still pass, including the 1e-15 tolerance on the ULP probes.

I have not compared that variant against your 400-probe set. If it costs bits there, keeping the current form is reasonable.
