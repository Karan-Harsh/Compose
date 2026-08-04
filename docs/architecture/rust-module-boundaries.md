# Rust Module Boundaries

## Purpose

The Rust layer is the native boundary of the application. It should be modular from the start so that platform complexity does not accumulate in command handlers.

## Recommended Layout

```text
apps/desktop/src-tauri/src/
  commands/
    mod.rs
    app.rs
    window.rs
  services/
    mod.rs
    window_service.rs
    app_info_service.rs
  state/
    mod.rs
    app_state.rs
  error/
    mod.rs
    app_error.rs
  lib.rs
  main.rs
```

The current scaffold keeps the native side intentionally small. `platform/`, `clipboard_service.rs`, and `settings_service.rs` should be introduced when their responsibilities become real rather than being added as empty ceremony.

## Module Responsibilities

### `commands`

Owns Tauri command handlers only.

Command handlers should:

- Validate transport inputs
- Call one or more services
- Map responses and errors

Command handlers should not:

- Implement platform details inline
- Own persistence logic
- Become the home for application state rules

### `services`

Owns core native-side business and orchestration logic.

Examples:

- `WindowService`
- `ClipboardService`
- `SettingsService`
- Later `HotkeyService` and `AccessibilityService`

This is the primary place where behavior should live.

### `platform`

Owns operating-system-specific implementations.

Recommended rule:

- Shared service interfaces above
- Platform-specific detail below

This lets us support macOS and Windows without leaking those concerns everywhere else.

### `state`

Owns application state shared across command handlers or native subsystems.

Examples:

- Window handles
- Runtime configuration
- Managed service instances

Keep this explicit rather than relying on scattered globals.

### `error`

Owns shared error definitions and mapping logic.

Establishing a small error module early will make command behavior more consistent and improve future debugging.

## `lib.rs` vs `main.rs`

Recommended approach:

- `lib.rs` owns module wiring and command registration
- `main.rs` stays very small and focuses on process startup

This improves testability and prevents `main.rs` from becoming the real application container.

## Early Design Caution

Do not over-abstract the Rust service layer in Phase 1.

We should create enough structure to support growth, but not invent traits and factories everywhere before multiple implementations exist.
