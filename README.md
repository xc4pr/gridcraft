# Publishing queue for storytold/gridcraft

One file per action under `queue/`. Each file starts with a header block:

- `kind`: `issue+pr` (open the issue, then the PR with `Fixes #ISSUE` replaced by the new number), `pr`, or `review`
- `repo`: target repository
- `base` / `head`: for PRs (head lives in the fork xc4pr/gridcraft)
- `pr` / `commit`: for reviews (the PR number and the head commit that was checked out and tested)

Then the title and body sections, ready to paste.
