# Project Overview & Architecture

## Summary
This project is a TUI note-taking application designed around outliner workflows similar to that of LogSeq, native 
Markdown editing, emedded LaTeX rendering, and in the future RTC.

The system will rely on a mereological storage layout combined with bidirectional links and block references.

---

## Core

### 1. Storage and Directory Hierarchy
Notes strictly follow a 4 tier part-whole hierarchy: **Workspace $\rightarrow$ Directory $\rightarrow$ Document $\rightarrow$ Block**.

* **Dual-Path Resolution:** Folders and standalone documents share names to allow nodes to act as both detailed content containers and structural parents (e.g., `Algorithms.md` sits alongside `Algorithms/`).
* **Transitive Parts:** Containment in directory sub-paths automatically implies mereological child status ($A \subset B \subset C \implies A \subset C$).
* **Human-Readable Plaintext:** All files are stored as pure `.md` files with YAML frontmatter.
```
vault/
├── Data_Structures.md              <-- Parent node content
├── Data_Structures/                <-- Sub-parts
│   ├── Algorithms.md
│   └── Algorithms/
│       └── Two_Sum.md
```

### 2. Graph Linkage
* **Path-Based Links:** Express hierarchical containment (`[[Data_Structures/Algorithms]]`).
* **Associative Links:** Express peer-to-peer graph connections across disparate branches (`[[LinearAlgebra]]`)
* **Block Transclusions:** Pull atomic outliner bullets from any note into the current buffer via deterministic block IDs (`((block_id))`)

### 3. Rendering Pipeline
* **Edit/View Dual Mode:** Active outliner blocks render as raw Markdown/Latex; unfocused blocks render parsed AST styled elements.
* **Latex Strategy:** Dual-path approach supporting lightweight ASCII/Unicode math conversion for universal terminals, with optional high-resolution Sixel/Kitty graphics protocol rendering for complex formulas.

### 4. RTC & Synchronizing
* **Block-Level CRDTs:** Text changes are tracked per outliner bullet using CRDT delta operations (e.g., LWW-Element-Set / Yjs text deltas) to prevent file-wide merge conflicts.
* **Granular Sharing Scopes:** Edit and view permissions can be scoped to individual atomic blocks (`((block_id))`), single documents, or full directory trees.
* **Local-First Sync:** Disk edits continuously flush to standard markdown while network adapters broadcast state changes P2P.

## Roadmap
1. **Mereological File & Index Engine**
	* Implement directory/file scanner and YAML frontmatter parser.
	* Build the in-memory graph index for reference links and path hierarchies.
2. **TUI Buffer Editor & Outliner**
	* Setup `ratatui` layout with a dual-mode block editing workflow
	* Implement block-level focus/unfocus events and Markdown styling.
3. **LaTeX renderer**
	* Integrate fallback Unicode text-math converter for inline inline formulas.
	* Add Sixel/Kitty image protocols rendering for complex LaTeX block formulas.
4. **CRDT Sync & RTC**
	* Attach block UUID tags (`^block-id`) to outliner elements.
	* Integrate the CRDT network protocol for RTC block editing across workspace.