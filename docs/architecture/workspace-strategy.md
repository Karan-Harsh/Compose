# Workspace Strategy

## Recommendation

Use npm workspaces with a small number of explicit top-level packages.

This fits the current goals well:

- Low operational overhead
- Good support for a monorepo
- Familiar workflow
- No extra package manager complexity unless we later prove a need

## Root Layout

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
```

## Root Responsibilities

The repository root should own:

- Workspace registration
- Shared scripts
- Shared TypeScript base configuration
- Shared lint and formatting configuration
- CI entry points

The repository root should not own product logic.

## Root Files To Add In Phase 1

Recommended initial root files:

- `package.json`
- `package-lock.json`
- `tsconfig.base.json`
- `eslint.config.js` or `eslint.config.mjs`
- `prettier.config.cjs`
- `.editorconfig`
- `.github/workflows/ci.yml`

## Workspace Registration

Recommended workspace declaration:

```json
{
  "workspaces": ["apps/*", "packages/*"]
}
```

This keeps the workspace simple and predictable.

## Package Extraction Strategy

Not every planned package needs implementation immediately.

Phase 1 should scaffold directory placeholders only when that helps the repository structure. It is acceptable to leave some packages absent until real ownership emerges.

Recommended posture:

- `apps/desktop` should be real in Phase 1
- `packages/types` may be justified early if frontend contracts appear quickly
- `packages/shared`, `packages/ai-core`, and `packages/prompt-engine` should stay unimplemented until there is real code to hold

## Shared Script Strategy

Prefer root scripts that delegate to workspace scripts. This keeps contributor onboarding simple.

Examples:

- `npm run lint`
- `npm run typecheck`
- `npm run build`
- `npm run dev --workspace @typeflow/desktop`

## Naming Recommendation

Use a stable project scope early for packages.

Recommended convention:

- `@typeflow/desktop`
- `@typeflow/types`
- `@typeflow/shared`
- `@typeflow/ai-core`
- `@typeflow/prompt-engine`

If the project name changes later, the package scope can change once, but drifting names across the repo will create avoidable cleanup work.
