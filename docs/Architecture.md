---
description: Understand MamboTools modules, Git operations, MamboUI integration, and destructive-action boundary.
title: Architecture
order: 20
---

::page{layout="docs" width="normal" sidebar=true}

# Architecture

MamboTools is a small terminal application with three core modules and one UI entry point:

| Area | Responsibility |
|---|---|
| `catalog` | Official ordered repository list and public clone URLs |
| `paths` | XDG install-root discovery and safe single-component paths |
| `repo` | Install, fast-forward update, identity validation, state detection, and verified deletion |
| `main` | Keyboard handling and Ratatui presentation |

The core modules form a library so deterministic behaviour can be tested without a terminal. MamboUI owns shared terminal styling and shell layout; MamboTools owns its repository table, status text, operations, and confirmation dialog.

## Operation flow

1. Resolve the managed root from `MAMBO_REPOS_DIR`, XDG, or `HOME`.
2. Turn the selected catalog name into exactly one child path.
3. Check the path state before invoking Git or the filesystem.
4. For an existing clone, verify its configured origin against the selected catalog package.
5. Run one bounded Git or filesystem operation and show its result.
6. Re-read local state on the next frame.

Git runs as a subprocess. Users keep the behaviour and configuration of the Git installation they already trust, while MamboTools avoids a large native dependency and separate authentication model.

## Delete boundary

Deletion is the only destructive operation. The UI requires explicit confirmation, and the core rejects unsafe package names, conflicting paths, symbolic links, an unexpected GitHub origin, and a directory whose canonical path differs from Git's reported top-level directory. Only then does it remove that exact managed child.

## Current constraints

- The catalog ships with the binary; there is no network catalog service.
- Git work is synchronous, so the UI waits while a clone or pull runs.
- Authentication prompts are disabled inside the alternate screen.
- Repository releases must update the compiled catalog when Project Mambo adds or retires a project.

These constraints keep the first release predictable. Add remote catalog loading or background jobs only when catalog maintenance or measured latency makes them necessary.
