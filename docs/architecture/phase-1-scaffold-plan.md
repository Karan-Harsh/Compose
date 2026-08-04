# Phase 1 Scaffold Plan

## Purpose

Phase 1 is where we turn the Phase 0 architecture into a runnable repository without jumping ahead into product behavior.

The goal is not to build features.

The goal is to produce a clean, disciplined scaffold that future features can safely grow inside.

## Scope

Phase 1 should include:

- npm workspace setup
- `apps/desktop` Tauri v2 application scaffold
- React, TypeScript, Vite, and TailwindCSS integration
- Shared TypeScript configuration
- Linting and formatting
- Base Rust command registration
- CI for install, lint, typecheck, and build validation

Phase 1 should not include:

- Real AI provider integrations
- Accessibility automation depth
- Clipboard-heavy workflows
- Text transformation commands
- Settings persistence beyond placeholders if not required for bootstrapping

## Deliverable Standard

At the end of Phase 1, we should have:

- A repository that installs cleanly
- A desktop app that boots locally
- A Rust command boundary that is real but minimal
- A frontend structure that prevents UI/business/native coupling
- Tooling that enforces consistency from the beginning

## Recommended Sequence

1. Create workspace root files and npm workspaces
2. Scaffold `apps/desktop`
3. Align TypeScript, lint, and formatting configs
4. Establish Rust module layout and command registration pattern
5. Add CI checks
6. Verify the scaffold and document any deviations

## Exit Criteria

Phase 1 is complete when:

- `npm install` succeeds
- The desktop app boots
- Lint and typecheck run successfully
- The Rust command layer is wired to the frontend
- The repo structure matches the documented architecture
- No feature code has been smuggled into the scaffold
