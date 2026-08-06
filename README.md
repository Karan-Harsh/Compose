# Desktop AI Writing Assistant

An open-source cross-platform desktop AI writing assistant built with a native shell and a web UI.

The product goal is simple: remove the friction of switching between an application and a separate AI chat window just to rewrite or respond to text. A global shortcut should open a lightweight command palette anywhere the user can type, capture context, run a command, and return the output back into the active application.

This repository is being built deliberately as an engineering project first.

We are optimizing for:

- Clean architecture
- Maintainability
- Extensibility
- Production-quality code
- Thoughtful documentation
- A repository other developers enjoy reading and contributing to

We are not optimizing for rapid feature delivery or short-term hacks.

## Current Status

The project has completed `Phase 0`, `Phase 1`, and the core of `Phase 2`. It is now in `Phase 3`.

Phase 0 established:

- Repository structure
- Architectural boundaries
- Documentation standards
- Development conventions
- Roadmap and delivery phases

Phase 1 delivered:

- npm workspace setup
- `apps/desktop` scaffolded with Tauri v2, React, TypeScript, Vite, and TailwindCSS
- Shared linting, formatting, and TypeScript configuration
- Initial Rust command and service wiring
- CI validation for install, lint, typecheck, web build, and native check

Phase 2 delivered native platform services:

- SQLite-backed `SettingsService`
- `ClipboardService` text read/write
- `HotkeyService` OS registration
- `AccessibilityService` selection capture before window focus
- `AiService` provider contracts with stub completion and command registry metadata

Phase 3 has begun with:

- Lightweight command palette shell (`features/palette`)
- Command discovery, keyboard navigation, and stub execution
- Palette-focused Zustand state flows
- Compact always-on-top palette window with hide-on-blur / Esc

Still deferred: deeper AX/UIA replace paths and richer Phase 4 prompt workflows. OpenRouter is configured via the root `.env`.

## Product Direction

The application should eventually be available anywhere through a global keyboard shortcut. Instead of switching to a browser or chat app, the user opens a command palette and invokes commands such as:

- `/rewrite`
- `/reply`
- `/translate`
- `/fix`
- `/summarize`

The long-term processing model is:

`Input -> Context -> Command -> AI Provider -> Output`

## Architecture Principles

- React knows nothing about macOS or Windows APIs
- Native capabilities are exposed through Rust commands
- Platform behavior is isolated behind services
- The system is command-driven rather than feature-driven
- Provider integrations are abstracted behind a stable AI boundary
- Documentation is treated as a first-class artifact

## Proposed Repository Shape

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

  .github/
  README.md
  ROADMAP.md
  CONTRIBUTING.md
```

See the architecture docs for the rationale behind this layout:

- [Architecture Overview](/Users/karan/Documents/proj/docs/architecture/overview.md)
- [Repository Structure](/Users/karan/Documents/proj/docs/architecture/repository-structure.md)
- [Phase 1 Scaffold Plan](/Users/karan/Documents/proj/docs/architecture/phase-1-scaffold-plan.md)
- [Workspace Strategy](/Users/karan/Documents/proj/docs/architecture/workspace-strategy.md)
- [Desktop Application Structure](/Users/karan/Documents/proj/docs/architecture/desktop-application-structure.md)
- [Rust Module Boundaries](/Users/karan/Documents/proj/docs/architecture/rust-module-boundaries.md)
- [Command And Service Design](/Users/karan/Documents/proj/docs/architecture/command-and-service-design.md)
- [Tauri Command Guidelines](/Users/karan/Documents/proj/docs/api/tauri-command-guidelines.md)

## Decision Records

Major architectural decisions are captured as ADRs:

- [ADR-0001: Monorepo Structure](/Users/karan/Documents/proj/docs/decisions/ADR-0001-monorepo-structure.md)
- [ADR-0002: Native Boundary Through Rust Commands](/Users/karan/Documents/proj/docs/decisions/ADR-0002-native-boundary.md)
- [ADR-0003: Command-Driven Application Model](/Users/karan/Documents/proj/docs/decisions/ADR-0003-command-system.md)

## Near-Term Focus

Configure OpenRouter in the root `.env` (`OPENROUTER_API_KEY`, `OPENROUTER_MODEL`), then polish insertion reliability and Phase 4 prompt/review flows.
