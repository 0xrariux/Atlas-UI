# Atlas UI overview

Atlas UI is a native design system and component library for applications
written in Rust with a Slint interface. It aims to narrow the experience gap
between native development and mature web frontend ecosystems, without
reproducing CSS or introducing a web runtime.

## What Atlas provides

Atlas standardizes decisions that applications would otherwise repeat:

- colors, typography, spacing, density, elevation, and motion;
- a geometric grid and responsive rules;
- controls, surfaces, navigation, and data presentation;
- editorial and documentation components;
- reusable application templates;
- vector icons and bundled fonts;
- an executable gallery, fixtures, scenarios, and reproducible captures;
- keyboard, accessibility, performance, and compatibility contracts.

Components express intentions. The application retains ownership of its data,
navigation, network access, filesystem, clipboard, and persistence. Atlas does
not perform silent remote actions or domain mutations.

## Current scope

The `v0.1.0` release exposes 97 components and 180 public symbols: 58 stable
and 122 preview. It includes stable
`AtlasStatusIndicator`, a compact semantic signal that shares `BadgeTone` with
`AtlasBadge`, plus preview `AtlasScrollbar` for controlled vertical overflow.
Preview contracts can evolve without creating a permanent
compatibility commitment too early.

Linux, Windows, and macOS are continuously validated for workspace compilation,
Clippy, tests, and public contracts with Rust `1.92`. The deterministic visual
profile remains the project's macOS arm64 machine with Slint's software
renderer and a scale factor of 1. CI portability must not be interpreted as a
pixel-identical rendering claim for every backend, display scale, font stack,
or input configuration. WebAssembly and embedded targets remain unverified.

## Intended users

- Rust applications that need a coherent native UI;
- teams sharing a design system across multiple products;
- Slint component authors looking for a validation methodology;
- documentation products, dashboards, and complex desktop tools.

Atlas is a library, not a complete application framework: domain logic, routes,
services, and infrastructure remain in the host application.

## Companion application templates

The [`template-atlas`](https://github.com/0xrariux/template-atlas) repository
contains four complete native consumers—Command, Forge, Fleet, and Ledger—that
demonstrate Atlas across distinct themes and application structures. Its README
provides rendered previews, clone instructions, and commands for running each
template.

The companion repository is the recommended visual starting point when a
consumer needs a complete screen or shell. Atlas remains the dependency and
component authority; template-atlas owns product composition, demonstration
data, navigation, and art direction. Its four applications also form external
consumer suites for Atlas upgrades, covering 97 rendered states without moving
product code into the library. See
[External consumer scenarios](EXTERNAL_CONSUMER_SCENARIOS.md).
