## Zttl

A terminal note-taking app with a graph index connecting them via containment, links and transclusions. 
Navigation is split into two modes over the same notes: 
- Ego: Surrounding context
- Spatial: Containment hierarchy

## Prerequisites
* **Rust Toolchain:** Stable Rust (1.80+ recommended) via [rustup](https://rustup.rs/)

## Run
```bash
git clone https://github.com/Pivius/zttl.git
cd zttl

cargo run
```

## Usage

## Data Format
Each note is Markdown + YAML frontmatter:

---
```yaml
id: "01J..."                     # ULID (26 chars)
title: "Data Structures"
status: active                   # active | archived
parents: []                      # slugs/ids of containing notes
tags: [ ... ]                    
created: / updated: / deadline:  # reserved
share_id: / visibility:          # reserved future RTC
```
---

Body syntax:
- [[Title] or [slug]]: Conceptual links
- ((block-id)): Transclusion
- - bullet text ^block-id: Addressable block