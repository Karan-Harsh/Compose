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

The project is in `Phase 0`, where we are defining:

- Repository structure
- Architectural boundaries
- Documentation standards
- Development conventions
- Roadmap and delivery phases

Application features will not be implemented until the foundation is solid.

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
- [Tauri Command Guidelines](/Users/karan/Documents/proj/docs/api/tauri-command-guidelines.md)

## Decision Records

Major architectural decisions are captured as ADRs:

- [ADR-0001: Monorepo Structure](/Users/karan/Documents/proj/docs/decisions/ADR-0001-monorepo-structure.md)
- [ADR-0002: Native Boundary Through Rust Commands](/Users/karan/Documents/proj/docs/decisions/ADR-0002-native-boundary.md)
- [ADR-0003: Command-Driven Application Model](/Users/karan/Documents/proj/docs/decisions/ADR-0003-command-system.md)

## Near-Term Focus

Phase 1 should begin only after we agree on the Phase 0 foundation. Once approved, the next step is to scaffold the desktop app and project tooling inside the structure defined here.
