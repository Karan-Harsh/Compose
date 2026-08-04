# ADR-0002: Native Boundary Through Rust Commands

## Status

Accepted

## Date

2026-08-04

## Context

The application depends on native operating system capabilities such as global shortcuts, clipboard access, accessibility APIs, active window awareness, and text insertion. If UI code accesses platform APIs directly, the codebase will become tightly coupled to platform concerns and harder to test.

## Decision

React will not call operating system APIs directly.

All native functionality will be exposed through Rust-based Tauri commands and implemented behind service boundaries in the native layer.

## Rationale

- Preserves a clean separation between presentation and platform integration
- Improves testability of frontend logic
- Makes platform-specific behavior explicit
- Aligns with the intended service-oriented architecture

## Consequences

Positive:

- Cleaner frontend code
- Stronger platform abstraction
- Easier future support for multiple operating systems

Negative:

- Requires careful command design
- Adds an integration boundary that must be documented and tested

## Notes

Command handlers should remain thin and delegate to services such as `ClipboardService`, `AccessibilityService`, and `WindowService`.
