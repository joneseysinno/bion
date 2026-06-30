# bion-soma Vocabulary

Layer-neutral names for Soma (Level 1) public types.

## Current exports

| Type | Module | Role |
|------|--------|------|
| `NeuronId` | `id` | Opaque neuron identity |
| `FiberId` | `id` | Opaque fiber port identity |
| `GanglionId` | `id` | Opaque ganglion identity |
| `SynapseId` | `id` | Opaque synapse identity |
| `IdSeed` | `id` | Pure deterministic id derivation |
| `IdSource` | `id` | Trait for value-threaded minting |
| `NeuronType` | `neuron` | Structural neuron role |
| `Arity` | `neuron` | Fiber count contract |
| `NeuronCapabilities` | `neuron` | Role capability flags |
| `Polarity` | `polarity` | Fiber direction |
| `ValidSynapse` | `polarity` | Proof of legal wiring |
| `SignalType` | `signal` | Schema atom |
| `Impulse` | `signal` | Typed signal value |
| `*Value` newtypes | `signal` | Nominal payload wrappers |
| `RoutingLabel` | `tag` | Routing/filter label |
| `LabelError` | `tag` | Label construction errors |

## Retired names

| Old name | Replacement | Reason |
|----------|-------------|--------|
| `CortexTag` | `RoutingLabel` | No Level-1 type named after an upper layer |
| `TagError` | `LabelError` | Paired with `RoutingLabel` |
| `IdGen` | `IdSource` + `IdSeed` | Effectful `&mut` generators evicted |
| `SequentialIdGen` | `IdSeed` | Pure value-threaded counter |
| `UuidIdGen` | *(moved to bion-store)* | Entropy is an effect |
| `Compatibility` | *(moved to bion-cortex)* | Compatibility is policy, not ontology |

## Reserved namespace (not yet implemented)

- `PolaritySign` / `Charge` — excitatory/inhibitory dipole (inhibition deferred)
- Hierarchical `RoutingLabel` ordering via `.` separator — deferred
