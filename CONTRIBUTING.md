# Contributing feedback

Atlas UI is experimental and actively maintained. Reports from real Rust and
Slint applications help identify rendering differences, unclear APIs, missing
states, accessibility problems, and gaps in the component system.

## Choose the appropriate issue form

- Use **Rendering or component defect** for incorrect rendering, interaction,
  layout, accessibility, regression, or runtime behavior.
- Use **Feature or design-system feedback** for a missing capability, API
  improvement, component proposal, or broader workflow problem.

Do not use public issues for security vulnerabilities or content containing
credentials, private source code, personal data, or proprietary screenshots.

## What makes a report actionable

Provide enough information for another person to reproduce and evaluate the
behavior without guessing:

- affected Atlas component, template, or subsystem;
- Atlas and Slint versions or commit revisions;
- stable or preview facade;
- operating system, architecture, renderer, scale factor, theme, and density;
- viewport or window dimensions for visual issues;
- minimal reproduction steps and a reduced code sample or repository;
- expected and actual results;
- screenshots or short recordings when they materially clarify the issue;
- regression status and last known working version, when available.

For visual feedback, describe the observable consequence—for example clipping,
overlap, unreadable contrast, unstable geometry, or lost hierarchy—rather than
only stating a stylistic preference.

## Triage expectations

Maintainers may ask for a smaller reproduction, additional environment details,
or confirmation against a supported profile. A report may be closed when it
cannot be reproduced, falls outside Atlas's documented scope, duplicates an
existing issue, or lacks the information required to investigate it.

Submitting an issue does not guarantee a particular implementation or release
schedule. Stable API compatibility and shared fixes take priority over local
workarounds; preview APIs may evolve as feedback is incorporated.
