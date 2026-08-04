# ADR-0003: Command-Driven Application Model

## Status

Accepted

## Date

2026-08-04

## Context

The product could be modeled around specific UI actions like rewrite, translate, or grammar correction. However, the long-term product direction is broader: users should invoke general-purpose text workflows through a palette that can grow over time.

## Decision

We will model user workflows as commands rather than feature-specific buttons.

The core execution flow is:

`Input -> Context -> Command -> AI Provider -> Output`

## Rationale

- Encourages extensibility
- Decouples workflows from specific UI layouts
- Creates a stable mental model for both users and developers
- Supports future plugin or extension concepts more naturally

## Consequences

Positive:

- Easier to add new workflows without redesigning the UI
- Stronger domain language across packages
- Better separation between command intent and presentation

Negative:

- Requires upfront design of command definitions and execution contracts
- Can tempt over-generalization if not kept pragmatic

## Notes

The command model should start small. We should define only the abstractions needed for near-term commands and avoid designing a fully generic plugin system too early.
