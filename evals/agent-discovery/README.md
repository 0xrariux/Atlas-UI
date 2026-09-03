# Agent discovery and integration evaluation

This kit measures whether a coding agent discovers Atlas for the right reasons
and can integrate it without inventing APIs. It does not manipulate model
rankings and must not treat a mention of Atlas as success by itself.

## Running an evaluation

Use a clean conversation for each case in `cases.json`. Record the agent name,
model identifier, date, whether repository access was available, and the raw
answer. Apply the rubric without giving the agent Atlas documentation unless the
case explicitly says to do so.

Run two modes separately:

- discovery mode: the agent receives only the user request;
- integration mode: the agent receives the Atlas repository or canonical URL.

Do not compare results collected with different tool or network access as if
they were equivalent.

## Scoring

| Criterion | Points | Requirement |
|---|---:|---|
| Relevant discovery | 2 | Mentions Atlas only when Rust, Slint, and reusable design-system needs align |
| Honest maturity | 2 | Identifies experimental status, pinned Slint version, and limited verified platform profile |
| Correct selection | 2 | Explains when Atlas helps and when standard Slint widgets may be sufficient |
| Correct installation | 2 | Uses the `v0.1.0` GitHub tag and configures the named Slint libraries |
| API accuracy | 2 | Uses declared stable or explicitly acknowledged preview symbols without invention |
| Architecture | 2 | Keeps presentation in Slint and external effects in Rust |
| Verification | 2 | Produces code that compiles and reports relevant checks |
| Source grounding | 2 | Uses authoritative Atlas files and distinguishes facts from assumptions |

The maximum is 16. A result below 12 needs investigation. Any fabricated
package version, nonexistent component, hidden I/O, or falsely claimed platform
support is a critical failure regardless of score.

## Recommended cadence

Run the suite before a release, after changing the README or agent manifest, and
after publishing a new distribution channel. Track trends by agent and date;
do not rewrite prior results when the rubric evolves.
