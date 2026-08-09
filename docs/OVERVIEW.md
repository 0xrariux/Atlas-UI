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

Version `0.2.1` retains 48 stable foundational symbols. The rest of the catalog is
available as preview so it can evolve without creating a permanent compatibility
commitment too early.

The currently proven profile is the project's macOS arm64 machine with Slint's
software renderer and a deterministic scale factor of 1. Linux, Windows,
WebAssembly, and other renderers are not currently claimed as supported.

## Intended users

- Rust applications that need a coherent native UI;
- teams sharing a design system across multiple products;
- Slint component authors looking for a validation methodology;
- documentation products, dashboards, and complex desktop tools.

Atlas is a library, not a complete application framework: domain logic, routes,
services, and infrastructure remain in the host application.
