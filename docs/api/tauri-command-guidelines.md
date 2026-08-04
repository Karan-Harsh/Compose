# Tauri Command Guidelines

## Purpose

Tauri commands are the contract between the React UI and the native Rust layer.

They should be treated as public internal APIs with stable naming and predictable behavior.

## Core Rule

React must never call platform APIs directly.

Instead, it should call Tauri commands such as:

- `invoke("get_selected_text")`
- `invoke("paste_text")`
- `invoke("show_palette")`

## Design Principles

Good commands should be:

- Narrow in responsibility
- Named by capability
- Explicit in inputs and outputs
- Easy to mock from the frontend
- Independent from view-specific concerns

## Naming Guidance

Prefer verb-driven capability names:

- `get_selected_text`
- `paste_text`
- `show_palette`
- `hide_palette`
- `register_global_hotkey`
- `load_settings`
- `save_settings`

Avoid names tied to a specific button or UI flow.

Bad examples:

- `click_rewrite_button`
- `open_reply_modal`
- `run_grammar_feature`

## Input and Output Rules

Commands should:

- Accept structured arguments when complexity grows
- Return typed result payloads
- Use consistent error mapping

Recommended practice:

- Keep primitive-only signatures for very small commands
- Move to request/response structs as soon as a command has more than one or two meaningful parameters

## Separation of Concerns

Command handlers should orchestrate services, not implement all logic inline.

Preferred pattern:

1. Command handler validates input
2. Command handler calls a service
3. Service executes the domain or native behavior
4. Command handler maps the result to a transport-safe response

## Versioning Mindset

Even before formal versioning, commands should be designed as if they are stable contracts. Breaking changes should be deliberate and documented.

## Testing Guidance

We should eventually test commands at two levels:

- Unit tests for services and domain logic
- Integration tests for command-to-service wiring

The command layer should stay thin enough that most behavior is validated below it.
