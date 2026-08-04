# Desktop Application Structure

## Purpose

`apps/desktop` should be the runnable application shell, not a dumping ground for all logic.

Its structure should make it obvious where UI code ends, application orchestration begins, and native integration starts.

## Recommended Layout

```text
apps/desktop/
  src/
    app/
    components/
    features/
    hooks/
    lib/
    state/
    styles/
    types/
    main.tsx
  src-tauri/
    src/
      commands/
      services/
      platform/
      state/
      error/
      lib.rs
      main.rs
    Cargo.toml
    tauri.conf.json
    icons/
  package.json
  tsconfig.json
  vite.config.ts
```

The current scaffold intentionally implements only the directories needed to support the Phase 1 shell. Additional directories like `components`, `features`, `hooks`, `platform`, and `public` should appear when they earn their keep rather than as empty placeholders.

## Frontend Boundaries

### `src/app`

Application shell concerns only.

Examples:

- App bootstrap
- Providers
- Routing, if ever introduced
- High-level layout composition

### `src/components`

Reusable presentational components.

This directory should stay mostly UI-focused. The current scaffold keeps the first presentational pieces close to `src/app` until reusable UI emerges naturally.

### `src/features`

Feature-facing UI modules.

This is where palette-specific UI flows can live later, but business logic should still be pushed downward into domain or orchestration modules.

### `src/hooks`

React-specific composition helpers.

Avoid turning hooks into hidden service containers.

### `src/lib`

Frontend application utilities with clear intent.

Good candidates:

- Tauri invoke helpers
- Frontend-side command client
- Validation adapters

### `src/state`

Zustand stores and state orchestration.

State should coordinate flows, not absorb all business logic.

### `src/types`

App-local types that do not yet justify extraction into `packages/types`.

## Important Constraint

Do not mirror backend or domain layers mechanically inside React just because it looks tidy. Directory structure should express real responsibility, not symmetry for its own sake.

## Rust Boundaries

The `src-tauri` tree is where native behavior is organized. It should be treated as a small backend with explicit modules, not a place for all logic to collapse into `main.rs`.

See [Rust Module Boundaries](/Users/karan/Documents/proj/docs/architecture/rust-module-boundaries.md).
