# Bion Three-Crate Architecture — Cursor Implementation Plan
*v1.0 · June 2026 · Greenfield Build*

---

## 0. Architectural Laws (Read Before Every Phase)

These are inviolable. Treat violations as compiler errors.

| Law | Statement |
|-----|-----------|
| **Soma Purity** | `bion-soma` has zero I/O. `#![no_std]` enforced. No `bion-*` imports. No `infinite-db` vocabulary. |
| **PNS Gateway** | `bion-cns` never opens a db connection. It holds `&dyn PnsReader` and `&dyn PnsWriter`. Nothing else. |
| **No Direct DB from CNS** | If `bion-cns` imports `infinite-db` at any level, that is a retrovirus-class violation. Fail the build. |
| **Retrovirus Prohibition** | Run mode never writes graph definitions (DNA). App state writes are permitted. Definition writes are not. |
| **Channel Boundary** | Sync→Async boundary always crossed via `try_send` into `tokio::sync::mpsc`. Never block the executor. |
| **One Function Per File** | Every public function in `bion-pns` and `bion-cns` lives in its own file. No exceptions. |
| **Vocabulary Disjointness** | Bion terms and infinite-db terms never share a namespace. `VOCABULARY.md` at workspace root. CI lint enforced. |

---

## 1. Workspace Structure

```
bion/                          ← workspace root
├── Cargo.toml                 ← workspace manifest
├── VOCABULARY.md              ← canonical term registry; CI lint source
├── ARCHITECTURE.md            ← this document summarized as reference
│
├── crates/
│   ├── bion-soma/             ← shared alphabet (no_std, no I/O)
│   ├── bion-pns/              ← peripheral nervous system (connector)
│   ├── bion-cns/              ← central nervous system (execution engine)
│   └── bion/                  ← facade (re-exports, feature flags)
```

### Workspace `Cargo.toml`

```toml
[workspace]
members = [
    "crates/bion-soma",
    "crates/bion-pns",
    "crates/bion-cns",
    "crates/bion",
]
resolver = "2"

[workspace.dependencies]
bion-soma = { path = "crates/bion-soma" }
bion-pns  = { path = "crates/bion-pns" }
bion-cns  = { path = "crates/bion-cns" }
tokio     = { version = "1", features = ["full"] }
serde     = { version = "1", features = ["derive"] }
uuid      = { version = "1", features = ["v4"] }
```

---

## 2. Dependency Graph

```
infinite-db  (external — not in this workspace)
      ↑
bion-pns     (owns the db connection; exposes PnsReader + PnsWriter traits)
      ↑
bion-cns     (holds &dyn PnsReader + &dyn PnsWriter; owns Circuit execution)
      ↑
bion         (facade; re-exports; feature flags)
      ↑
[ any consumer: WebSocket frontend, HTTP client, CLI, Genesis, test harness ]

bion-soma    (imported by bion-pns AND bion-cns; no arrows between soma and db)
```

**Dependency rules encoded in `Cargo.toml`:**
- `bion-soma` depends on: nothing (no_std + alloc only)
- `bion-pns` depends on: `bion-soma`, `infinite-db`, `tokio`, `serde`
- `bion-cns` depends on: `bion-soma`, `bion-pns` (trait only — via feature flag)
- `bion` depends on: `bion-soma`, `bion-pns`, `bion-cns`

---

## 3. Phase P0 — Workspace Scaffold

**Blockers:** None. Start here.

### Tasks

- [ ] Create workspace `Cargo.toml` as above
- [ ] Create `VOCABULARY.md` with two sections: `BION_TERMS` and `DB_TERMS`
- [ ] Create `ARCHITECTURE.md` summarizing the three-crate model and the seven laws
- [ ] Create all four crate stubs (empty `lib.rs` + `Cargo.toml` for each)
- [ ] Add CI lint: grep `crates/bion-soma/src` for any token in `DB_TERMS` → fail
- [ ] Add CI lint: grep `crates/bion-cns/src` for `infinite_db` or `infinite-db` → fail
- [ ] Verify `cargo check --workspace` passes on stubs

### `VOCABULARY.md` seed

```markdown
# VOCABULARY.md

## BION_TERMS
NeuronId, NeuronType, SignalType, Impulse, Polarity, CortexTag, FiberId,
ValidSynapse, BoolValue, IntValue, FloatValue, SignalText, ByteBlob, UnitValue,
PnsReader, PnsWriter, PnsWatcher, DefinitionChange, StateUpdate, ImpulseEnvelope,
Ganglion, GanglionId, Synapse, SynapseId, Fiber, Circuit, Executor,
Pacemaker, ActionWave, ActionPotential, Transmission, Threshold,
CliTransport, WsTransport

## DB_TERMS
InfiniteDb, HyperedgeId, Hyperedge, SpaceId, RevisionId, BranchId,
SnapshotId, DimensionVector, HilbertKey, WalEntry, Record, SignalSample,
DerivationBus, DerivationSubscriber, WriteSession, WriteJob
```

---

## 4. Phase P1 — `bion-soma` (Existing; Validate and Lock)

**Blockers:** P0 complete.

`bion-soma` is already scaffolded. This phase validates it is complete and clean before the other crates depend on it.

### Crate manifest

```toml
[package]
name    = "bion-soma"
version = "0.1.0"
edition = "2021"

[features]
default = []
serde   = ["dep:serde"]
bridge  = []          # unlocks into_inner() accessors for PNS serialization

[dependencies]
serde = { workspace = true, optional = true }

# NO infinite-db. NO tokio. NO other bion-* crates.
```

### File layout (existing — verify all present)

```
crates/bion-soma/src/
├── lib.rs          ← pub mod declarations + re-exports only
├── id.rs           ← NeuronId, FiberId, IdSeed, IdSource
├── neuron.rs       ← NeuronType, NeuronCapabilities
├── polarity.rs     ← Polarity, ValidSynapse
├── signal.rs       ← SignalType, Impulse, BoolValue, IntValue, FloatValue,
│                     SignalText, ByteBlob, UnitValue, Compatibility
└── tag.rs          ← CortexTag, TagError
```

### Validation checklist

- [ ] `#![no_std]` present in `lib.rs`
- [ ] `extern crate alloc;` present
- [ ] No `use infinite_db` anywhere in `src/`
- [ ] No `use bion_` anywhere in `src/`
- [ ] `FloatValue` uses `to_bits()` equality (NaN-lawful)
- [ ] `Impulse` does not implement `Hash` (float makes this unlawful)
- [ ] `IdSeed::mint(self) -> Option<(NeuronId, Self)>` pattern present (referentially transparent)
- [ ] `cargo test -p bion-soma` passes

---

## 5. Phase P2 — `bion-pns` (Peripheral Nervous System)

**Blockers:** P1 locked.

PNS owns all contact with infinite-db. It exposes traits that CNS and frontends consume. It owns transport adapters (WebSocket, CLI, HTTP). Every public function is its own file.

### Crate manifest

```toml
[package]
name    = "bion-pns"
version = "0.1.0"
edition = "2021"

[features]
default = ["ws"]
ws      = ["dep:tokio-tungstenite"]
cli     = []
http    = ["dep:axum"]

[dependencies]
bion-soma   = { workspace = true, features = ["serde", "bridge"] }
infinite-db = { path = "../../infinite-db" }   # adjust path
tokio       = { workspace = true }
serde       = { workspace = true }
tokio-tungstenite = { version = "0.21", optional = true }
axum        = { version = "0.7", optional = true }
```

### Module tree

```
crates/bion-pns/src/
├── lib.rs                     ← pub mod declarations + re-exports

├── gateway/                   ← trait definitions (what CNS sees)
│   ├── mod.rs
│   ├── pns_reader.rs          ← trait PnsReader
│   ├── pns_writer.rs          ← trait PnsWriter
│   └── pns_watcher.rs         ← trait PnsWatcher + DefinitionChange type

├── reader/                    ← PnsReader implementations
│   ├── mod.rs
│   ├── fetch_neuron.rs        ← fn fetch_neuron(...)
│   ├── fetch_subgraph.rs      ← fn fetch_subgraph(...)
│   └── fetch_fibers.rs        ← fn fetch_fibers(...)

├── writer/                    ← PnsWriter implementations
│   ├── mod.rs
│   ├── write_state.rs         ← fn write_state(...)
│   ├── write_definition.rs    ← fn write_definition(...) [edit mode only]
│   └── write_ack.rs           ← RevisionAck type

├── watcher/                   ← DerivationBus → mpsc bridge
│   ├── mod.rs
│   ├── spawn_watcher.rs       ← fn spawn_watcher(...) → PnsWatcherHandle
│   ├── translate_event.rs     ← fn translate_event(db_event) → DefinitionChange
│   └── reconcile.rs           ← fn reconcile(ui_rev, db_rev) → ReconcileAction

├── transport/                 ← frontend-facing surfaces
│   ├── mod.rs
│   ├── ws/
│   │   ├── mod.rs
│   │   ├── accept_connection.rs    ← fn accept_connection(...)
│   │   ├── read_impulse.rs         ← fn read_impulse(stream) → ImpulseEnvelope
│   │   └── write_state_update.rs   ← fn write_state_update(stream, update)
│   └── cli/
│       ├── mod.rs
│       ├── read_line.rs            ← fn read_line(stdin) → ImpulseEnvelope
│       └── write_output.rs         ← fn write_output(stdout, update)

└── types/
    ├── mod.rs
    ├── impulse_envelope.rs    ← ImpulseEnvelope { neuron_id, impulse, revision }
    ├── state_update.rs        ← StateUpdate { neuron_id, impulse, revision }
    ├── definition_change.rs   ← DefinitionChange { subgraph_id, revision }
    └── reconcile_action.rs    ← ReconcileAction enum
```

### Key type definitions

#### `gateway/pns_reader.rs`
```rust
/// The only surface CNS uses to read graph definitions.
/// Implemented over the real db client and over an in-memory mock for tests.
#[async_trait]
pub trait PnsReader: Send + Sync {
    async fn fetch_neuron(&self, id: NeuronId) -> Result<NeuronDescriptor, PnsError>;
    async fn fetch_subgraph(&self, root: NeuronId) -> Result<SubgraphSnapshot, PnsError>;
    async fn fetch_fibers(&self, neuron: NeuronId) -> Result<Vec<FiberDescriptor>, PnsError>;
}
```

#### `gateway/pns_writer.rs`
```rust
/// The only surface CNS uses to persist app state.
/// CNS calls this after Circuit calculation produces a result.
#[async_trait]
pub trait PnsWriter: Send + Sync {
    async fn write_state(
        &self,
        neuron: NeuronId,
        impulse: Impulse,
    ) -> Result<RevisionAck, PnsError>;
}
```

#### `gateway/pns_watcher.rs`
```rust
/// Delivers DefinitionChange notifications to CNS.
/// Backed by tokio::sync::mpsc::Receiver.
pub trait PnsWatcher: Send {
    async fn recv(&mut self) -> Option<DefinitionChange>;
}

pub struct DefinitionChange {
    pub subgraph_id: NeuronId,
    pub revision: u64,            // RevisionId from infinite-db, wrapped
}
```

#### `types/impulse_envelope.rs`
```rust
/// A typed signal arriving from any transport.
/// Same shape whether it came from WebSocket, CLI, or HTTP.
pub struct ImpulseEnvelope {
    pub target: NeuronId,
    pub impulse: Impulse,
    pub client_revision: Option<u64>,  // for optimistic UI reconciliation
}
```

#### `types/state_update.rs`
```rust
/// A typed state change pushed to any subscribed frontend.
pub struct StateUpdate {
    pub source: NeuronId,
    pub impulse: Impulse,
    pub revision: u64,
}
```

### spawn_blocking boundary (enforced here, not in CNS)

All infinite-db calls inside `reader/` and `writer/` functions must use `tokio::task::spawn_blocking`. The pattern for every function in those modules:

```rust
// Example: reader/fetch_neuron.rs
pub async fn fetch_neuron(
    db: Arc<InfiniteDb>,
    id: NeuronId,
) -> Result<NeuronDescriptor, PnsError> {
    let raw_id = id.into_raw();   // soma bridge accessor
    tokio::task::spawn_blocking(move || {
        // synchronous db call here
        db.query(...)
    })
    .await
    .map_err(PnsError::from)?
    .map(translate_to_descriptor)
}
```

### CLI transport note

`transport/cli/read_line.rs` reads a line from stdin, parses it as `target_id:signal_type:value` (or JSON), and returns an `ImpulseEnvelope`. `write_output.rs` formats a `StateUpdate` as human-readable text and writes to stdout. The db stack below the transport boundary is identical to WebSocket. This is the cheapest full-stack integration test surface — script it in CI.

---

## 6. Phase P3 — `bion-cns` (Central Nervous System)

**Blockers:** P2 stable (gateway traits locked).

CNS owns graph execution. It holds two injected PNS traits and nothing else from the outside world. It runs three concurrent loops. Every public function is its own file.

### Crate manifest

```toml
[package]
name    = "bion-cns"
version = "0.1.0"
edition = "2021"

[dependencies]
bion-soma = { workspace = true }
bion-pns  = { workspace = true }   # trait definitions only — no db types leak through
tokio     = { workspace = true }
serde     = { workspace = true }

# CRITICAL: infinite-db must NOT appear here. Ever.
```

### Module tree

```
crates/bion-cns/src/
├── lib.rs                         ← pub mod declarations + re-exports

├── circuit/                       ← graph execution model
│   ├── mod.rs
│   ├── ganglion.rs                ← Ganglion, GanglionId, GanglionInstance
│   ├── synapse.rs                 ← Synapse, SynapseId, SynapticMode
│   ├── fiber.rs                   ← Fiber, FiberId (wraps bion-soma FiberId)
│   ├── threshold.rs               ← Threshold (was FirePolicy)
│   ├── transmission.rs            ← Transmission (signal propagation rules)
│   └── circuit.rs                 ← Circuit (the live graph of GanglionInstances)

├── execution/                     ← the execution engine
│   ├── mod.rs
│   ├── executor.rs                ← Executor — drives a Circuit
│   ├── pacemaker.rs               ← Pacemaker — sequences execution order
│   ├── action_wave.rs             ← ActionWave — parallel wavefront
│   ├── action_potential.rs        ← ActionPotential — live signal in flight
│   └── graded_potential.rs        ← GradedPotential — derived/accumulated signal

├── hydration/                     ← startup + hot-reload
│   ├── mod.rs
│   ├── hydrate_circuit.rs         ← fn hydrate_circuit(reader) → Circuit
│   ├── hydrate_subgraph.rs        ← fn hydrate_subgraph(reader, id) → CircuitFragment
│   ├── rebuild_fragment.rs        ← fn rebuild_fragment(circuit, fragment)
│   └── snapshot.rs                ← SubgraphSnapshot → CircuitFragment translation

├── loops/                         ← the three concurrent runtime loops
│   ├── mod.rs
│   ├── execution_loop.rs          ← fn run_execution_loop(circuit, rx, pns_writer)
│   ├── watch_loop.rs              ← fn run_watch_loop(circuit, watcher, pns_reader)
│   └── output_loop.rs             ← fn run_output_loop(result_rx, state_tx)

├── runtime/                       ← top-level assembly
│   ├── mod.rs
│   ├── cns_runtime.rs             ← CnsRuntime struct — assembles the three loops
│   ├── start.rs                   ← fn start(reader, writer, watcher) → CnsRuntime
│   └── shutdown.rs                ← fn shutdown(runtime)

└── types/
    ├── mod.rs
    ├── circuit_fragment.rs        ← CircuitFragment { root, ganglia, synapses }
    ├── cns_error.rs               ← CnsError enum
    └── revision_cursor.rs         ← RevisionCursor { last_written, last_seen }
```

### Three concurrent loops

#### `loops/execution_loop.rs`
```rust
/// Receives ImpulseEnvelope from any transport (via channel from PNS).
/// Runs the impulse through the Circuit.
/// Calls PnsWriter to persist the result.
/// Sends StateUpdate to output_loop for frontend delivery.
pub async fn run_execution_loop(
    circuit: Arc<RwLock<Circuit>>,
    impulse_rx: mpsc::Receiver<ImpulseEnvelope>,
    pns_writer: Arc<dyn PnsWriter>,
    result_tx: mpsc::Sender<StateUpdate>,
) { ... }
```

#### `loops/watch_loop.rs`
```rust
/// Receives DefinitionChange from PNS watcher channel.
/// Calls PnsReader to fetch the new subgraph definition.
/// Triggers rebuild_fragment to hot-reload that Circuit fragment.
/// Does NOT interrupt in-flight ActionWaves — drains first.
pub async fn run_watch_loop(
    circuit: Arc<RwLock<Circuit>>,
    mut watcher: Box<dyn PnsWatcher>,
    pns_reader: Arc<dyn PnsReader>,
) { ... }
```

#### `loops/output_loop.rs`
```rust
/// Receives StateUpdate from execution_loop.
/// Broadcasts to all subscribed frontend channels.
/// Handles backpressure: slow frontends get dropped updates, not blocked execution.
pub async fn run_output_loop(
    result_rx: mpsc::Receiver<StateUpdate>,
    subscribers: Arc<RwLock<Vec<mpsc::Sender<StateUpdate>>>>,
) { ... }
```

### Hydration sequence

```
CnsRuntime::start()
  │
  ├─ call hydrate_circuit(pns_reader)          ← PnsReader.fetch_subgraph(root)
  │   └─ returns Circuit (immutable snapshot)
  │
  ├─ spawn run_execution_loop(circuit, ...)
  ├─ spawn run_watch_loop(circuit, ...)
  └─ spawn run_output_loop(...)
       │
       └─ System is live. Loops run independently.

On DefinitionChange arriving in watch_loop:
  ├─ call hydrate_subgraph(pns_reader, changed_id)
  ├─ drain in-flight signals for affected fragment
  ├─ call rebuild_fragment(circuit, new_fragment)
  └─ resume — hot reload complete, no restart required
```

### Revision cursor (UI/DB sync validation)

`types/revision_cursor.rs` tracks two revisions per Neuron:
- `last_written` — the `RevisionAck.revision` returned by the last `PnsWriter.write_state()` call
- `last_seen` — the revision on the last `DefinitionChange` or incoming `StateUpdate`

When `last_seen > last_written + 1`, another writer has modified the state. CNS fires a reconciliation signal to PNS (`reconcile.rs`). PNS fetches the current db state and pushes a corrective `StateUpdate` to all subscribers. This is the optimistic UI + automatic reconciliation guarantee — the UI shows instant results from CNS calculation, and drift is corrected in the background.

---

## 7. Phase P4 — `bion` Facade

**Blockers:** P2 + P3 stable.

```
crates/bion/src/
├── lib.rs    ← feature-gated re-exports only. Zero logic.
```

### `lib.rs`
```rust
//! Bion — typed reactive graph data backend.
//! Add one dependency. Choose your features.

#[cfg(feature = "soma")]
pub use bion_soma as soma;

#[cfg(feature = "pns")]
pub use bion_pns as pns;

#[cfg(feature = "cns")]
pub use bion_cns as cns;
```

### `Cargo.toml`
```toml
[package]
name    = "bion"
version = "0.1.0"
edition = "2021"

[features]
default = ["soma", "pns"]
soma    = ["dep:bion-soma"]
pns     = ["dep:bion-pns"]
cns     = ["dep:bion-cns"]
full    = ["soma", "pns", "cns"]

[dependencies]
bion-soma = { workspace = true, optional = true }
bion-pns  = { workspace = true, optional = true }
bion-cns  = { workspace = true, optional = true }
```

**Consumer recipes:**
- Frontend connector only: `bion = { features = ["pns"] }`
- Full backend with execution: `bion = { features = ["full"] }`
- Type vocabulary only: `bion = { features = ["soma"] }`

---

## 8. Data Flow Summary (Canonical Reference)

```
[Template customization by user]
UX writes custom genome → ImpulseEnvelope(definition_write)
  → PNS transport adapter (ws or cli)
  → PNS writer/write_definition.rs
  → spawn_blocking → infinite-db write
  → DerivationBus fires (sync)
  → PNS watcher/translate_event.rs → DefinitionChange
  → mpsc channel → CNS watch_loop
  → hydrate_subgraph → rebuild_fragment
  → StateUpdate pushed to UX (new app snapshot)

[User interacts with the app]
UX field update → ImpulseEnvelope(state_write)
  → PNS transport adapter
  → mpsc channel → CNS execution_loop
  → Circuit.execute(impulse) → ActionWave
  → result: StateUpdate → output_loop → UX (instant, optimistic)
  → side effect: PnsWriter.write_state() → spawn_blocking → infinite-db
  → RevisionAck → RevisionCursor.last_written updated

[Background: drift detection]
PNS watcher sees incoming revision > RevisionCursor.last_written + 1
  → reconcile.rs → PnsReader.fetch_subgraph → corrective StateUpdate → UX
```

---

## 9. CI Invariants

Add these checks to every PR. Fail the build on any violation.

```bash
# Law 1: Soma has no I/O vocabulary
grep -r "std::io\|tokio\|async fn\|infinite_db" crates/bion-soma/src && exit 1

# Law 2: CNS never touches the db directly  
grep -r "infinite_db\|InfiniteDb\|SpaceId\|RevisionId\|HilbertKey" crates/bion-cns/src && exit 1

# Law 3: Vocabulary disjointness (DB_TERMS must not appear in bion-soma)
# (script reads VOCABULARY.md DB_TERMS section and greps bion-soma/src)
scripts/lint_vocabulary.sh

# Law 4: One function per file in pns and cns (no fn in mod.rs except re-exports)
scripts/lint_single_function.sh crates/bion-pns/src
scripts/lint_single_function.sh crates/bion-cns/src

# Law 5: cargo test --workspace passes
cargo test --workspace
```

---

## 10. Build Sequence

| Phase | Target | Key Deliverable | Blocker |
|-------|--------|-----------------|---------|
| P0 | Workspace | `Cargo.toml`, `VOCABULARY.md`, CI lint, crate stubs | None |
| P1 | `bion-soma` | Validate existing soma; lock API | P0 |
| P2a | `bion-pns/gateway` | `PnsReader`, `PnsWriter`, `PnsWatcher` traits | P1 |
| P2b | `bion-pns/types` | `ImpulseEnvelope`, `StateUpdate`, `DefinitionChange` | P2a |
| P2c | `bion-pns/reader` | `fetch_neuron`, `fetch_subgraph`, `fetch_fibers` | P2b + infinite-db |
| P2d | `bion-pns/writer` | `write_state`, `write_definition` | P2b + infinite-db |
| P2e | `bion-pns/watcher` | `spawn_watcher`, `translate_event`, `reconcile` | P2c + P2d |
| P2f | `bion-pns/transport` | CLI adapter first, then WebSocket | P2b |
| P3a | `bion-cns/circuit` | `Ganglion`, `Synapse`, `Fiber`, `Circuit` | P2a (traits only) |
| P3b | `bion-cns/execution` | `Executor`, `Pacemaker`, `ActionWave` | P3a |
| P3c | `bion-cns/hydration` | `hydrate_circuit`, `hydrate_subgraph`, `rebuild_fragment` | P3a + P2a |
| P3d | `bion-cns/loops` | `execution_loop`, `watch_loop`, `output_loop` | P3b + P3c |
| P3e | `bion-cns/runtime` | `CnsRuntime`, `start`, `shutdown` | P3d |
| P4 | `bion` facade | Feature-gated re-exports | P2 + P3 |

**Start at P2a.** The gateway traits are the contract that everything else depends on. Lock them before writing any implementation.

---

*Bion Three-Crate Architecture Plan · v1.0 · June 2026*
