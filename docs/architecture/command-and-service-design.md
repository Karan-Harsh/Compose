# Command And Service Design

## Goal

Before we scaffold code, we should be explicit about the first command and service shapes so that the initial implementation does not bake in accidental architecture.

## Command Categories

The first scaffold should introduce only a few command categories:

- App lifecycle commands
- Window commands
- System utility commands

Examples:

- `show_palette`
- `hide_palette`
- `get_app_info`
- `health_check`

These are scaffold commands, not product commands.

## Why Avoid Product Commands In Phase 1

Commands like `/rewrite` or `/translate` imply provider integration, prompt strategy, context assembly, and output flows. That is Phase 2 and later work.

Phase 1 commands should prove the boundary, not the product.

## Frontend Command Client

The React app should not call `invoke` directly all over the codebase.

Recommended pattern:

- Centralize Tauri invocation in a thin command client
- Expose typed frontend functions
- Keep the raw transport API hidden behind that client

Example direction:

```ts
export async function showPalette(): Promise<void>
export async function hidePalette(): Promise<void>
export async function getAppInfo(): Promise<AppInfo>
```

This gives us one place to evolve payloads, typing, and error handling.

## Initial Native Services

Phase 1 should keep the service list intentionally small.

Recommended initial services:

- `WindowService`
- `AppInfoService`

These are enough to demonstrate:

- Native command wiring
- Service delegation
- Typed responses
- Rust/React separation

## Deferred Services

These should wait until Phase 2 unless scaffolding truly requires them:

- `AIService`
- `HotkeyService`
- `AccessibilityService`
- `ClipboardService`
- `SettingsService`

They are important, but introducing empty service abstractions too early will create ceremony without clarity.

## State Management Recommendation

On the frontend:

- Use Zustand for app-level state only where shared coordination exists
- Avoid creating stores for trivial component-local state

On the Rust side:

- Use managed state only for cross-command runtime concerns
- Avoid stuffing all dependencies into a single oversized app state object

## Testing Guidance

Even the initial scaffold should make future testing easy.

Recommended early posture:

- Frontend command client is unit-testable
- Rust services are testable without UI concerns
- Command handlers stay thin enough to need minimal direct testing
