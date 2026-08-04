# Repository Structure

## Recommendation

Use a monorepo from day one.

The project already has multiple natural seams:

- Desktop application
- Shared domain packages
- Prompt logic
- Provider abstractions
- Documentation

A monorepo keeps those seams explicit without forcing premature package publishing.

## Proposed Layout

```text
/
  apps/
    desktop/
      src/
      src-tauri/
      public/

  packages/
    ai-core/
    prompt-engine/
    shared/
    types/

  docs/
    architecture/
    decisions/
    api/

  .github/
  README.md
  ROADMAP.md
  CONTRIBUTING.md
```

## Directory Responsibilities

### `apps/desktop`

Owns the runnable Tauri application.

Suggested responsibilities:

- React UI
- Tauri integration
- Window bootstrap
- Frontend state wiring
- Rust command registration
- Native platform modules

Keep product-specific execution here unless there is a clear reason to extract it.

### `packages/ai-core`

Owns provider-facing abstractions and AI orchestration contracts.

Good candidates:

- Provider interfaces
- Model capability definitions
- Request and response contracts
- Retry and error policies

This package should not depend on React or Tauri.

### `packages/prompt-engine`

Owns reusable prompt construction and command prompt policies.

Good candidates:

- Prompt templates
- Command prompt assembly
- Context serialization helpers
- Prompt versioning rules

Keep it deterministic and testable.

### `packages/shared`

Owns shared utilities that are truly generic.

Examples:

- Result helpers
- Validation utilities
- Shared constants

This package should stay small. If it becomes a dumping ground, split it.

### `packages/types`

Owns shared TypeScript domain types used across the frontend workspace.

Examples:

- Command definitions
- Settings schemas
- Provider metadata
- DTOs used by UI packages

Do not put behavior here unless there is a strong reason.

### `docs`

Owns architecture, decisions, APIs, and contributor-facing material. The docs directory should mature alongside the codebase rather than lag behind it.

## Important Constraint

Do not extract packages too early just because code is reusable in theory.

A good heuristic:

- Keep code in `apps/desktop` until reuse is proven
- Extract to `packages/*` when a boundary becomes stable and valuable

This avoids a monorepo full of thin packages with unclear ownership.

## Recommended Future Additions

These should be added only when justified:

- `packages/test-utils`
- `packages/eslint-config`
- `packages/tsconfig`
- `docs/templates`

## What To Avoid

- Mixing native Rust modules into generic TypeScript packages
- Putting command business logic directly into React components
- Treating `shared` as a miscellaneous bucket
- Creating separate packages for speculative future reuse
