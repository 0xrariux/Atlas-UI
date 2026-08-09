# Atlas UI visual evidence

## Local Codex pre-review

After `codex login`, run at most four analyses per invocation:

```bash
cargo run -p atlas-ui-tooling -- review-screenshots
```

The runner resumes at the first unprocessed scenario and stops after four
captures. `--dry-run` displays the next batch, `--batch-size 1..4` reduces its
size, `--scenario <id>` targets one capture, and `--reset` restarts the queue.
`--rerun-last-batch` analyzes the latest batch again after a fix without
advancing the queue. Reports in `screenshots/reviews/` remain local and never
constitute baseline approval.

The `review-contexts.json` registry describes page intentions, invariants, and
exclusions. A batch report authorizes a correction only for a `defect` with
confidence ≥ 0.90 and a violated contract. One local defect is sufficient; an
assumed shared defect requires consensus across at least two batch captures.

- `scenarios.json`: canonical identities and thresholds;
- `schema/`: machine-readable scenario contract;
- `baselines/`: reference PNGs tracked with the source;
- `metadata/`: identity and approval state for each baseline;
- `results/`: generated captures, ignored by Git;
- `diffs/`: generated red-channel diffs, ignored by Git.

The PNG in `baselines/` is the latest promoted reference after recapture and
therefore shows the current final rendering to review. `results/` contains the
most recent working capture, `diffs/` visualizes its difference from the
baseline, and `reviews/` stores local Codex assessments. A recaptured baseline
remains `pending-human` until explicitly approved.

```bash
cargo run -p atlas-ui-tooling -- capture-scenarios --validate-only
cargo run -p atlas-ui-tooling -- capture-scenarios --scenario foundations.dark.normal.desktop
cargo run -p atlas-ui-tooling -- capture-scenarios --update-baselines
cargo run -p atlas-ui-tooling -- capture-scenarios \
  --approve-baseline foundations.dark.normal.desktop \
  --reviewer "Name" \
  --note "Compared with the foundations-v1 fixture"
```

Updating a baseline resets its approval to `pending-human`. A comparison is
refused if identity or dimensions differ, even if the images look similar.
