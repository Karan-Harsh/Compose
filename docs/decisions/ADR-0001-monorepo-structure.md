# ADR-0001: Monorepo Structure

## Status

Accepted

## Date

2026-08-04

## Context

The project will include a desktop application, native Rust code, frontend UI code, shared domain contracts, prompt logic, and architecture documentation. We need a repository layout that supports growth without prematurely fragmenting the codebase.

## Decision

We will use a monorepo with top-level `apps`, `packages`, and `docs` directories.

The initial target structure is:

```text
/
  apps/
    desktop/

  packages/
    ai-core/
    prompt-engine/
    shared/
    types/

  docs/
    architecture/
    decisions/
    api/
```

## Rationale

- The project already has multiple distinct responsibility areas
- Shared contracts will likely emerge early
- Documentation deserves a permanent top-level home
- A monorepo allows deliberate extraction without multi-repo overhead

## Consequences

Positive:

- Clear separation between application code and reusable packages
- Better support for shared tooling and standards
- Easier documentation discoverability

Negative:

- Requires discipline to avoid over-extraction
- Adds some structural complexity early

## Notes

We will keep code in `apps/desktop` until reuse is proven rather than immediately populating every planned package with implementation code.
