# Project Overview & Architecture

## Summary
Zttl is a TUI note-taking application designed around outliner workflows similar to LogSeq, native Markdown editing, embedded LaTeX rendering, and (in the future) RTC.

The system separates the three concerns that PARA and Zettelkasten traditionally conflate — physical storage, hierarchical containment, and actionability into three independent layers that do not have to agree with each other. The default mode is pure zettelkasten (atomic notes joined by links); an optional PARA actionability layer is layered on top via frontmatter metadata.

---

## Core

### 1. Storage (Flat)
Physical location carries zero semantic weight. Every note has a stable ULID and lives at `vault/<shard>/<slug>.md`, where `<shard>` is the last two characters of the ULID and `<slug>` is a human-readable filename. Sharding is on the ULID *suffix* (the random half) rather than the prefix, because ULID prefixes are time-ordered and would collapse every note into one bucket for decades.

```
vault/
├── JK/data-structures.md
├── JM/algorithms.md
├── KM/fibonacci.md
├── KN/distributed-systems.md
├── KP/raft.md
├── KQ/category-theory.md
└── KR/unsorted-inbox.md
```

* **Human-Readable Plaintext:** All files are plain `.md` with YAML frontmatter.

### 2. Graph & Containment
* **Containment Edges:** Part-whole relationships (e.g. `Fibonacci ⊂ Algorithms`) are typed edges in the graph index, not directory nesting.
  * **Hybrid source:** explicit `parents:` frontmatter on the child *and* structure-note membership (a `type: area`/`project` note listing its members as `[[links]]`), merged in the index with `parents` winning on conflict.
  * **Multi-Parent:** A note can belong to many parents (`flow-state ⊂ Psychology` and `⊂ Productivity`).
  * **Transitive Closure:** `A ⊂ B ⊂ C ⟹ A ⊂ C` is computed in the index, so "show everything under X" is a graph query.
* **Associative Links:** Peer-to-peer connections across branches via `[[Title]]` / `[[slug]]`.
* **Block Transclusions:** Pull atomic outliner bullets from any note into the current buffer via deterministic IDs (`((block-id))`).

### 3. Note Identity & Address Resolution
* **Dual-Key Index:** primary `id → node`, plus fuzzy `title/slug → id`.
* **Resolution Precedence** for `[[X]]`: exact `id` → exact `title` → exact `slug` → ranked fuzzy.
* **Edges Store the Resolved `id`** while rendered link text stays human; links are re-resolved to the current title/slug on display, so renames and moves never break them.
* **Lazy ID Assignment:** the first time a target resolves and lacks an `id`, a ULID is generated and written into its frontmatter. Un-linked notes stay clean.
* **Ambiguity:** duplicate titles resolve by containment context first, then most-recently-edited; the TUI surfaces unresolved/ambiguous links rather than silently breaking.

### 4. Taxonomy (PARA Hybrid)
Actionability is frontmatter metadata only; its presence or absence never affects where a note lives or how it links. Absent `type`/`status` means an atomic note.

```yaml
---
id: 01J5K80123456789ABCDEFGHJK   # ULID, lazy on first link
title: Two Sum
type: note                       # note | project | area | resource   (default: note)
status: active                   # active | archived                  (default: active)
parents: [algorithms]            # optional; id or slug refs, multi-parent
created: 2026-08-24
updated: 2026-08-24
deadline: 2026-09-01             # optional
tags: [algorithms, leetcode]
---
```

* `note` — atomic zettel (default)
* `project` / `area` — actionability structure notes
* `resource` — reference material
* A 5th `structure`/`moc` type is a trivial future extension for non-actionable topic hubs.

### 5. Block Identity
* **Lazy `^block-id` Tags:** an outliner line gets a compact-hash ID only when edited under CRDT, transcluded via `((id))`, or block-embedded. Unreferenced lines stay raw.
* **Collision Safety:** IDs are generated against the in-memory block registry; on collision they are re-rolled until unique.
* **Stable & Portable:** IDs are never regenerated and travel with the block across cut/paste between notes. Dangling `((id))` renders as "missing block".

### 6. Rendering Pipeline
* **Edit/View Dual Mode:** Active outliner blocks render as raw Markdown/LaTeX; unfocused blocks render parsed AST styled elements.
* **LaTeX Strategy:** Dual-path approach — lightweight ASCII/Unicode math conversion for universal terminals, with optional high-resolution Sixel/Kitty graphics protocol rendering for complex formulas.

### 7. RTC & Synchronizing
* **Block-Level CRDTs:** Text changes are tracked per outliner bullet using CRDT deltas (e.g., LWW-Element-Set / Yjs text deltas) to prevent file-wide merge conflicts.
* **Granular Sharing Scopes:** Edit and view permissions can be scoped to atomic blocks (`((block-id))`), single documents, or a containment subgraph (transitive closure).
* **Local-First Sync:** disk edits flush to markdown while network adapters broadcast P2P. The flat, ID-addressed store materially simplifies sync — there is no mutable file path for CRDT logic to reconcile.

---

## Roadmap
1. **Flat Store & Graph Index Engine**
	* Implement the flat directory scanner and YAML frontmatter parser (`id`, `title`, `type`, `status`, `parents`).
	* Build the in-memory graph index: containment edges, associative links, block transclusions, transitive closure, and the dual-key (`id`/`slug`) index.
2. **TUI Buffer Editor & Outliner**
	* Setup `ratatui` layout with a dual-mode block editing workflow.
	* Generate a folder-view *projection* from the graph (multi-parent DAG rendered as a tree), plus `type` filters and an archive toggle.
3. **LaTeX renderer**
	* Integrate fallback Unicode text-math converter for inline formulas.
	* Add Sixel/Kitty image protocols rendering for complex LaTeX block formulas.
4. **CRDT Sync & RTC**
	* Attach lazy compact-hash block IDs (`^block-id`) to outliner elements.
	* Integrate the CRDT network protocol for RTC block editing across the workspace.
