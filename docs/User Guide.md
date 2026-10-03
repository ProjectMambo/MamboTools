---
description: Install, update, and safely remove Project Mambo repositories with MamboTools.
title: User guide
order: 10
---

::page{layout="docs" width="normal" sidebar=true}

# User guide

MamboTools manages local clones from the official Project Mambo catalog. It does not install operating-system packages, publish releases, or delete remote repositories.

## Choose the repository root

The default root is the XDG data directory, normally `~/.local/share/mambo/repos`. Set `MAMBO_REPOS_DIR` before launch to keep Mambo repositories elsewhere:

```bash
MAMBO_REPOS_DIR="$HOME/ProjectMambo" mambo-tools
```

MamboTools creates the root when the first repository is installed. It will not overwrite an existing child path that is not the expected clone.

## Install

Select a repository and press `i` or `Enter`. MamboTools clones its public HTTPS URL into one direct child of the managed root. A successful status message names the installed repository; a failure leaves Git's concise diagnostic in the status line.

## Update

Select an installed repository and press `u`. The update uses `git pull --ff-only` against the clone's configured upstream. Local commits, divergent history, uncommitted conflicts, missing upstream configuration, and authentication errors remain visible for the user to resolve with Git; MamboTools never invents a merge or rewrites history.

## Remove

Select an installed repository and press `d`. Read the warning, then press `y` to remove or `n`/`Esc` to cancel.

Removal is permanent for the local clone and includes uncommitted and untracked files. Before deleting, MamboTools checks all of these conditions:

- the catalog name is one safe path component;
- the target is not a symbolic link;
- Git reports the target itself as the worktree root;
- `origin` identifies the matching repository in the ProjectMambo GitHub organization.

If any check fails, MamboTools leaves the path untouched.

## Recover from common states

| Status | Meaning | Next step |
|---|---|---|
| `not installed` | No path exists for this catalog entry. | Press `i` to clone it. |
| `installed` | A Git worktree exists at the managed path. | Update or remove it as needed. |
| `path conflict` | A path exists but is not a directly managed Git worktree. | Inspect and move it manually; MamboTools will not overwrite it. |
| Git cannot fast-forward | Local and upstream histories differ or local work blocks the pull. | Resolve the repository with Git, then retry. |
| Origin mismatch | The path is a Git repository but not the selected official repository. | Move it out of the managed catalog path or correct the remote manually. |
