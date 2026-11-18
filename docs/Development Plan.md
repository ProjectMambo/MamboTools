---
description: Dependency-ordered safety, relationship, coordination, and release stages for MamboTools.
title: Development plan
order: 30
---

::page{layout="docs" width="normal" sidebar=true}

# Development plan

MamboTools is a released local repository manager, not a greenfield package manager. This plan preserves its small, guarded single-repository lifecycle while adding cross-repository awareness only where real Project Mambo provider and consumer relationships require it.

The plan is the dependency and acceptance map, not a calendar promise or issue list. Update it when implementation evidence changes a safety boundary, relationship contract, stage order, public behavior, or release gate.

## Current state

Version `0.1.0` ships a compiled catalog, XDG-aware install-root selection, public HTTPS clone, origin-verified fast-forward update, and confirmed local deletion. `catalog`, `paths`, and `repo` form the tested library boundary; the Ratatui entry point owns selection, status, and confirmation through MamboUI. Git remains an external command and owns transport and credentials.

The application stores no database, account, background state, or remote catalog. It derives each repository's state from one direct child of the configured root and treats unexpected paths as conflicts. Updates never merge or rewrite history. Deletion rejects symbolic links, unexpected origins, and directories that Git does not identify as the selected repository root.

The current evidence is the `0.1.0` source and lockfile, focused unit and local-Git tests, synchronized user and architecture guides, and CI formatting, test, Clippy, and locked-build gates. The remaining work begins with observed safety and integration gaps rather than speculative features.

## Target outcome

MamboTools remains the safe local home for official Project Mambo repositories and can explain their direct relationships, diagnose incompatible or missing local providers, and—only after the read-only model is proven—preview and perform a guarded provider-before-consumer fast-forward sequence.

~~~text
official catalog + declared direct relationships
                    |
                    v
       state and compatibility diagnosis
                    |
             explicit preview
                    |
                    v
 guarded repository operations -> installed Git worktrees
~~~

Providers continue to own their public contracts and releases. Consumers continue to own their exact pins, semantic adapters, migrations, and validation. MamboTools may order and guard repository operations; it does not infer application mappings or silently rewrite consumer files.

## Scope and deferrals

The target includes incident-driven hardening of install, update, and deletion; a minimal relationship contract agreed with MamboDocs; deterministic read-only dependency diagnosis; and, after that diagnosis proves useful, explicitly confirmed coordinated local updates with clear stop and recovery behavior.

The target excludes arbitrary third-party catalogs, a remote registry, accounts, cloud state, a daemon, background updates, plugins, operating-system package installation, remote repository deletion, automatic merges or rebases, force operations, semantic configuration edits, automatic dependency-pin changes, release publication, and arbitrary repository scripts. Add remote catalog loading only when manual catalog releases become a measured maintenance problem. Add background work only when measured Git latency makes the synchronous interface unusable.

## Ownership and trust boundaries

| Boundary | Owner and rule |
|---|---|
| Catalog, managed root, local state, planning, and confirmations | MamboTools; paths stay below one configured root and mutations fail closed. |
| Dependency declaration standard | MamboDocs; machine-readable relationships must complement, not scrape, the human README declaration. |
| Public API and compatibility promise | The provider repository; MamboTools does not reinterpret it. |
| Version pin, adapter, migration, and consumer tests | The consumer repository; coordination never replaces its validation. |
| Transport, authentication, remotes, branches, and upstreams | Git and the user's existing configuration; MamboTools does not store credentials. |
| Installed clone contents | The user; deletion remains explicit and warns that uncommitted and untracked files are removed. |

Package names, relationship declarations, filesystem entries, configured remotes, Git output, and repository contents are untrusted input. Diagnostics must not expose credentials or complete sensitive command output. MamboTools runs only its bounded Git operations and never executes commands supplied by a repository.

## Stage 0: Preserve the released baseline

The `0.1.0` install, update, refresh, and confirmed-delete journeys are the compatibility baseline. Keep the compiled official catalog and current storage-free model until a later stage demonstrates why either must change. Public key bindings, environment precedence, state labels, origin rules, and destructive confirmation remain stable unless a versioned change includes migration guidance.

Exit gate: the locked build and existing tests pass, the documented user journeys match the executable behavior, and no later stage weakens the path, origin, fast-forward, or confirmation checks.

## Stage 1: Harden the single-repository lifecycle

Start from real failures in the current workflow. Extend focused tests around install success and partial failure, dirty or divergent updates, missing upstreams, wrong origins, symbolic links, nested Git roots, deletion failure, Git output bounds, and terminal restoration. A failed operation must leave user-owned content untouched and report the next safe recovery step. Any cleanup of an incomplete clone is limited to a destination that MamboTools proved absent before starting and created for that operation.

Keep Git as a subprocess and reuse the existing `repo` boundary. Do not add an embedded Git implementation, generic command runner, job framework, or persistent operation store merely to make testing convenient. Record supported platforms from CI evidence rather than promising untested portability.

Exit gate: representative local-Git fixtures prove successful and failed install, update, and deletion paths; destructive rejection tests preserve the target; the terminal restores after normal exit and injected operation errors; and the complete locked gate passes.

## Stage 2: Define direct repository relationships

Coordinate with MamboDocs on the smallest versioned, machine-readable declaration needed for real sibling dependencies. Prove it against at least two existing shapes, such as a released provider consumed by an exact version and a provider consumed at an exact commit during coordinated development. The declaration identifies repositories and compatibility expectations; it does not duplicate package-manager lockfiles, semantic mappings, generated artifacts, or maintainer prose.

MamboColour is one provider example: it owns its stable roles, ordered colour access, compatibility, and issue-driven adaptability, while MamboSite or MamboDot owns its pin and adapter. MamboTools may report that relationship but does not add colour roles or choose consumer mappings.

Reject unknown schema versions, unsafe repository names, duplicate edges, self-dependencies, cycles, and references outside the official catalog. Do not scrape README tables or accept executable commands in the relationship data.

Exit gate: MamboDocs documents the ownership and compatibility rules, provider and consumer fixtures round-trip deterministically, invalid graphs fail with stable diagnostics, and existing repositories without a declaration retain their `0.1.0` behavior.

## Stage 3: Deliver read-only dependency diagnosis

Load declared direct relationships into a deterministic graph and show, for an installed repository, missing providers, installed revisions or versions that do not satisfy the declaration, unsupported schema versions, cycles, and the provider-before-consumer order. Read installed revisions and declared pins without changing a worktree, fetching a remote, or running consumer code.

Add only the minimum TUI detail needed to answer what is wrong and what should be handled first. Keep independent repositories usable when another declaration is invalid. Bound all displayed paths and Git diagnostics, and keep the existing install state visible.

Exit gate: fixtures cover an independent repository, a valid chain, a shared provider, a missing provider, a pin mismatch, a cycle, and an unsupported declaration; repeated scans produce the same order and messages; and diagnosis performs no filesystem or Git mutation.

## Stage 4: Add guarded coordinated updates

Begin this stage only after real use of Stage 3 shows that manual provider-to-consumer ordering causes repeated errors. Before mutation, present the exact repositories, current revisions, intended fast-forward operations, dependency order, and blocking conditions. Require explicit confirmation for the complete plan.

Every selected worktree must have the expected origin and a state compatible with fast-forward update. Stop before the first mutation when preflight fails. During execution, stop on the first failure, leave completed fast-forwards intact, and report each repository's previous and current revision plus the safe retry point. Do not reset, stash, discard changes, edit manifests, resolve conflicts, push, merge, publish, or claim atomic rollback across independent Git repositories.

Exit gate: local-remote fixtures prove successful chains, shared providers updated once, preflight rejection without mutation, mid-sequence failure with an accurate recovery report, and a safe retry. The same plan order is shown before and after execution, and single-repository actions remain available.

## Stage 5: Validate and release each milestone

Apply this gate to every milestone; a release is not blocked merely because evidence has correctly deferred Stage 4. Review keyboard-only operation, visible focus, confirmation wording, narrow-terminal behavior, bounded error output, and terminal restoration. Run formatting, all locked tests, strict Clippy, the locked build, strict MamboDocs validation, and the complete documentation synchronization check. Exercise the journeys implemented by the milestone in a disposable root: the baseline covers clean install, independent update, and confirmed deletion; lifecycle confidence adds failed-operation recovery; dependency visibility adds diagnosis; guarded coordination adds rejected and successful sequences.

Version each public behavior change according to its compatibility impact. Publish provider contracts before consumers, synchronize canonical documentation before generated repository and Wiki copies, and retain the previous release as the rollback path. Because MamboTools stores no hidden database, downgrading the binary must leave managed clones usable; any future persistent state requires its own version, migration, backup, and recovery contract before adoption.

Exit gate: a clean environment completes the documented journeys, failure cases preserve user data, CI and documentation gates pass, release notes identify compatibility changes, and the repository and Wiki publish the same plan.

## Milestones and risks

| Milestone | Included stages | Outcome |
|---|---|---|
| Released baseline | Stage 0 and its Stage 5 gate | Safe official-repository install, fast-forward update, and confirmed local deletion. |
| Lifecycle confidence | Stage 1 and its Stage 5 gate | Existing operations have representative failure and recovery evidence. |
| Dependency visibility | Stages 2 and 3 and their Stage 5 gate | Users can understand provider and consumer state without mutation. |
| Guarded coordination | Stage 4 and its Stage 5 gate | Proven local update sequences are previewed, confirmed, stopped safely, and released with complete validation. |

The main risks are treating prose as a machine contract, allowing a relationship graph to become a generic package manager, promising atomicity that independent Git worktrees cannot provide, and automating consumer changes that require project-specific judgment. Keep the contract narrow, make diagnosis useful before mutation, and require evidence before Stage 4.

Future complexity also requires evidence. A remote catalog needs repeated catalog-release pain; persistent state needs information that cannot be derived safely; background jobs need measured blocking latency; and a plugin or arbitrary-command system needs a trusted use case that bounded built-in operations cannot satisfy. Until then, leave them out.
