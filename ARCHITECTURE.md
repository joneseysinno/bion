# Bion Three-Crate Architecture

## Crates

| Crate | Role |
|-------|------|
| `bion-soma` | Shared alphabet — `#![no_std]`, zero I/O, no `bion-*` imports |
| `bion-pns` | Peripheral nervous system — owns infinite-db contact, exposes `PnsReader` / `PnsWriter` |
| `bion-cns` | Central nervous system — graph execution via injected PNS traits only |
| `bion` | Facade with feature-gated re-exports |

## Dependency graph

```
infinite-db (external)
      ↑
bion-pns
      ↑
bion-cns
      ↑
bion (facade)

bion-soma ← imported by bion-pns and bion-cns (no edge to db)
```

## Architectural laws

1. **Soma Purity** — `bion-soma` has zero I/O, `#![no_std]`, no `bion-*` imports, no infinite-db vocabulary.
2. **PNS Gateway** — `bion-cns` never opens a db connection; it holds `&dyn PnsReader` and `&dyn PnsWriter`.
3. **No Direct DB from CNS** — `infinite-db` must never appear in `bion-cns`.
4. **Retrovirus Prohibition** — run mode never writes graph definitions (DNA); app state writes are permitted.
5. **Channel Boundary** — sync→async via `try_send` into `tokio::sync::mpsc`; never block the executor.
6. **One Function Per File** — every public function in `bion-pns` and `bion-cns` lives in its own file.
7. **Vocabulary Disjointness** — Bion terms and infinite-db terms never share a namespace (`VOCABULARY.md`).

## Runtime loops (CNS)

- **execution_loop** — impulses in → circuit → `PnsWriter` + `StateUpdate` out
- **watch_loop** — `DefinitionChange` → hydrate → hot-reload fragment
- **output_loop** — broadcast `StateUpdate` to frontends (drop slow subscribers)

See `bion-three-crate-plan.md` for the full implementation plan.
