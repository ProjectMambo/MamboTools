---
description: Long-term maintenance triggers, safety boundaries, and release gates for MamboTools.
title: Development plan
order: 30
---

::page{layout="docs" width="normal" sidebar=true}

# Development plan

MamboTools `0.1.0` is a released local repository manager. Feature development is deferred; this plan preserves its safe install, fast-forward update, and confirmed removal paths without inventing an active feature backlog.

The plan is trigger-based rather than calendar-based. Update it when a reproduced defect, security advisory, official catalog change, dependency incompatibility, or retirement decision changes the maintenance boundary.

## Current state

The application ships a compiled Project Mambo catalog, resolves an XDG-aware managed root, clones public HTTPS remotes, updates only with `git pull --ff-only`, and removes a local clone only after explicit confirmation. Deletion rejects symbolic links, unexpected origins, unsafe names, and directories Git does not identify as the selected repository root.

`catalog`, `paths`, and `repo` form the tested library boundary. The Ratatui entry point owns selection, status, and confirmation through MamboUI `0.1.0`; Git remains an external command and owns transport and credentials. MamboTools stores no database, account, daemon state, or remote catalog.

Current evidence consists of the source and lockfile, 13 focused unit and local-Git tests, synchronized user and architecture guides, and CI gates for formatting, tests, Clippy, and the locked build.

## Target maintenance state

Keep the released install, update, refresh, and removal journeys reliable on supported Rust, Git, terminal, and MamboUI versions. Maintenance work should correct demonstrated safety, compatibility, catalog, or documentation problems with the smallest coherent change.

There is no active expansion milestone. A future feature becomes planned only after repeated user or maintainer evidence establishes the problem, MamboDocs defines any shared contract, and the plan is revised before implementation begins.

## Scope, ownership, and deferrals

In scope are correctness and security fixes, official catalog additions or retirements, supported dependency and toolchain compatibility, accessibility corrections, documentation accuracy, packaging, and maintenance releases.

| Boundary | Owner and rule |
|---|---|
| Catalog, managed paths, local-state labels, confirmations, and bounded operations | MamboTools; filesystem and Git mutations fail closed. |
| Transport, authentication, remotes, branches, and upstreams | Git and the user's existing configuration; MamboTools stores no credentials. |
| Shared terminal primitives | MamboUI; a provider change is validated and delivered before its MamboTools pin changes. |
| Installed clone contents | The user; rejected or failed operations preserve them, while confirmed removal clearly warns that the complete local clone is deleted. |

Deferred work includes a remote registry, accounts, cloud state, a daemon, background updates, third-party catalogs, plugins, dependency graphs, coordinated multi-repository updates, arbitrary repository scripts, semantic manifest edits, merge or rebase automation, force operations, pushes, releases, and remote deletion. Reconsider any item only after repeated observed failures or requests show that the released single-repository workflow is insufficient.

## Stage 0: Stable hold

Preserve the current key bindings, environment-variable precedence, one-child managed-path rule, storage-free architecture, exact-origin checks, fast-forward-only updates, destructive confirmation, and terminal restoration. Do not churn code or dependencies without a maintenance trigger.

Exit gate: CI and the documented local gate pass, user documentation matches the executable behavior, and there is no known actionable security or correctness defect in the supported journey.

## Stage 1: Triggered maintenance

Begin only for a reproduced incident, security advisory, official catalog change, or confirmed Rust, Git, terminal, or MamboUI incompatibility. Reproduce the problem at the lowest owning boundary, make the smallest fix there, and add one focused regression covering success, failure, and recovery where the path can affect user-owned data.

Never weaken path, origin, fast-forward, confirmation, or terminal-restoration safeguards to make a failing case pass. Keep Git as a subprocess unless measured evidence proves that this boundary itself is the problem. When MamboUI owns the fix, deliver and validate that provider first, then update MamboTools deliberately.

Exit gate: the reproduction fails before the change and passes after it, representative rejection or failure leaves the managed clone intact, affected documentation is current, and the complete repository gate passes.

## Stage 2: Maintenance release

Synchronize canonical documentation into MamboTools and MamboWiki, then exercise install, update, rejection, and confirmed removal in a disposable managed root. Version only shipped behavior according to compatibility impact, record the trigger and recovery path in release notes, and publish any provider change before the MamboTools consumer update.

MamboTools has no hidden stored state to migrate. The rollback path is the previous binary or tag, and managed clones remain ordinary Git repositories that users can operate without MamboTools.

Exit gate: a clean environment completes every documented journey, failure cases preserve user-owned data, CI and strict documentation checks pass, repository and Wiki plans match, and the previous release remains available for rollback.

## Retirement trigger

If MamboTools is superseded or can no longer be maintained safely, stop claiming active support, document the replacement or ordinary Git fallback, preserve the final source and tag, and archive only after users can continue managing their clones without proprietary state or a destructive migration.

Exit gate: the README and Wiki state the final support boundary, replacement or fallback instructions are verified, and no user repository depends on hidden MamboTools state.

## Validation, milestones, and risks

Run the complete gate for every maintenance change:

```bash
cargo fmt --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
../MamboDocs/script/check-repository.sh --strict .
git diff --check
```

The meaningful milestones are the released baseline, a triggered maintenance fix, a maintenance release, or retirement. There are no percentage-complete or calendar milestones while feature work is deferred.

The primary risks are destructive deletion, managing a wrong-origin repository, upstream Rust, Git, terminal, or MamboUI incompatibility, stale catalog entries, and documentation that overstates support. Preserve the current fail-closed boundaries, test the exact incident path, and add complexity only when evidence changes this plan.
