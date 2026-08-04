# Architecture Overview

## Intent

This application is an AI layer over the operating system, not a browser tab replacement.

Its responsibility is to:

1. Capture or receive user text and surrounding context
2. Let the user invoke a command
3. Execute that command through a provider abstraction
4. Return the output to the user or active application

## High-Level Layers

```text
React UI
  |
Application State + Use Cases
  |
Tauri Command Client
  |
Rust Command Handlers
  |
Native Services
  |
Operating System APIs
```

## Boundary Rules

### UI Layer

The React application is responsible for:

- Rendering UI
- Collecting user input
- Displaying results
- Managing view state

The UI layer must not:

- Access platform APIs directly
- Contain provider-specific business rules
- Own native automation logic

### Application Layer

The application layer coordinates:

- Command selection
- Context assembly
- Request orchestration
- Result handling

This layer should remain mostly platform-agnostic and testable.

### Native Layer

Rust is the native boundary for:

- Global shortcuts
- Window management
- Text capture and insertion
- Accessibility integration
- Clipboard interaction
- Settings and storage integration

React should only interact with this layer through stable commands.

## Command-Driven Model

The core domain concept is a `command`, not a feature button.

Examples:

- `rewrite`
- `reply`
- `translate`
- `fix`
- `summarize`

Each command should eventually define:

- Identifier
- Description
- Input requirements
- Context requirements
- Prompt strategy
- Output handling expectations

This design allows the product surface to expand without coupling the UI to individual workflows.

## Service-Oriented Design

Native and application behavior should be organized around focused services.

Initial service candidates:

- `AIService`
- `ClipboardService`
- `AccessibilityService`
- `WindowService`
- `HotkeyService`
- `SettingsService`

Each service should:

- Own one area of responsibility
- Expose a small interface
- Avoid knowledge of unrelated services when possible

## Data and Persistence

SQLite is the default local persistence mechanism.

It should eventually store concerns such as:

- Settings
- Provider configuration metadata
- Command history, if we decide to keep it
- Future cached artifacts, if justified

Persistence details should not leak into UI components.

## Cross-Platform Posture

Cross-platform support is a first-class requirement, but identical implementation details across operating systems are not. We should design stable service contracts and allow platform-specific implementations behind them.

Recommended approach:

- Shared service interfaces
- OS-specific modules behind those interfaces
- Capability detection where needed
- Explicit documentation of platform gaps

## Architectural Priorities

When tradeoffs appear, prefer:

1. Clear boundaries
2. Testability
3. Readability
4. Extensibility
5. Performance optimization only where evidence justifies it
