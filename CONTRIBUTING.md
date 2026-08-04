# Contributing

## Philosophy

This project values clarity over speed.

Contributors should prefer:

- Small focused changes
- Explicit architectural boundaries
- Clean abstractions over convenience shortcuts
- Documentation for major decisions
- Changes that improve long-term maintainability

Please avoid introducing code that "works for now" but undermines the design direction of the repository.

## Before You Build

Before adding code:

- Read the architecture docs in [`docs/architecture`](/Users/karan/Documents/proj/docs/architecture/overview.md)
- Read relevant ADRs in [`docs/decisions`](/Users/karan/Documents/proj/docs/decisions/ADR-0001-monorepo-structure.md)
- Confirm whether the proposed work belongs in Phase 1 or later

If a change alters the architecture, add or update an ADR.

## Design Rules

- React must not call operating system APIs directly
- Native behavior must flow through Rust commands
- Business logic should live outside React components
- Services should have one responsibility
- New modules should have clear ownership and naming
- Prefer composition over inheritance
- Avoid unnecessary dependencies

## Documentation Expectations

Documentation is part of the implementation.

Please document:

- Architectural decisions
- New service boundaries
- Public contracts between packages
- Non-obvious tradeoffs

## Pull Request Guidance

Good pull requests are:

- Narrow in scope
- Easy to review
- Backed by clear rationale
- Consistent with the current phase of the roadmap

If a pull request introduces technical debt, call it out explicitly rather than hiding it.

## Commit Guidance

Prefer commits that describe intent rather than mechanics.

Examples:

- `docs: add initial architecture decision records`
- `build: scaffold desktop workspace`
- `refactor: extract provider contract from UI flow`

## Early Testing Expectations

As the project matures, every layer should gain appropriate tests. During early phases, we still expect contributors to validate:

- Build integrity
- Type safety
- Lint cleanliness
- Basic command contract stability

## Collaboration Style

This is an opinionated learning-focused open-source project. Thoughtful disagreement is welcome. If something seems like technical debt or architectural drift, raise it early.
