# Roadmap

## Guiding Principle

This roadmap is intentionally staged so that architecture, tooling, and operational discipline come before product features.

## Phase 0: Foundation

Status: `Completed`

Goals:

- Finalize repository structure
- Define architectural boundaries
- Document conventions and decisions
- Align on delivery phases
- Reduce ambiguity before scaffolding

Deliverables:

- Architecture overview
- ADR set for initial decisions
- Contribution guidelines
- Phase-based roadmap

Exit criteria:

- We agree on repository shape
- We agree on the Rust/React boundary
- We agree on the command-driven domain model
- We agree on initial package responsibilities

Outcome:

- Repository foundation documented
- Initial ADRs accepted
- Phase 1 scaffold blueprint defined

## Phase 1: Project Scaffolding

Status: `In progress`

Goals:

- Scaffold `apps/desktop` with Tauri v2, React, TypeScript, Vite, and TailwindCSS
- Configure npm workspaces
- Establish linting and formatting
- Add test strategy placeholders and quality gates
- Add CI for install, lint, typecheck, and build validation

Expected outputs:

- Working desktop shell
- Shared TypeScript configuration strategy
- Base Rust command registration
- CI workflow and repository automation

Current progress:

- npm workspaces configured
- `apps/desktop` scaffolded
- Shared TypeScript, ESLint, Prettier, and EditorConfig added
- Minimal Rust command and service boundary implemented
- CI workflow added
- Validation passing for lint, typecheck, web build, and native `cargo check`

Supporting design docs:

- [Phase 1 Scaffold Plan](/Users/karan/Documents/proj/docs/architecture/phase-1-scaffold-plan.md)
- [Workspace Strategy](/Users/karan/Documents/proj/docs/architecture/workspace-strategy.md)
- [Desktop Application Structure](/Users/karan/Documents/proj/docs/architecture/desktop-application-structure.md)
- [Rust Module Boundaries](/Users/karan/Documents/proj/docs/architecture/rust-module-boundaries.md)
- [Command And Service Design](/Users/karan/Documents/proj/docs/architecture/command-and-service-design.md)

## Phase 2: Core Platform Services

Goals:

- Introduce native service boundaries
- Implement settings persistence
- Define SQLite access strategy
- Establish command registry and execution pipeline
- Create provider abstraction contracts

Candidate services:

- `WindowService`
- `HotkeyService`
- `ClipboardService`
- `AccessibilityService`
- `SettingsService`
- `AIService`

## Phase 3: Command Palette Shell

Goals:

- Build the lightweight palette UI
- Support command discovery and execution
- Add basic command input and response rendering
- Establish application state flows with Zustand

Focus:

- User flow correctness
- Command architecture
- UI composability

## Phase 4: Text Workflows

Goals:

- Add first-party commands like rewrite, tone, summarize, and grammar fix
- Add input/context assembly rules
- Improve output insertion and user review flow

Focus:

- Reliability over breadth
- Clear command behavior contracts
- Prompt versioning discipline

## Phase 5: Provider Ecosystem

Goals:

- Add provider adapters for OpenAI, Anthropic, Gemini, and Ollama
- Support provider-specific capabilities without leaking them into UI code
- Add credential and model configuration flows

Focus:

- Stable provider abstraction
- Clear error handling
- Testable adapter contracts

## Phase 6: Platform Depth

Goals:

- Improve cross-platform text capture and insertion
- Expand accessibility and clipboard integrations
- Harden window behavior and shortcut handling

Focus:

- macOS and Windows parity
- Reliable native automation behavior
- Clear platform-specific isolation

## Phase 7: Open-Source Maturity

Goals:

- Improve contributor onboarding
- Add issue templates and discussion conventions
- Expand automated testing
- Publish releases and changelog process

Focus:

- Repository quality
- Community trust
- Sustainable maintenance
