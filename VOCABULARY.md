# VOCABULARY.md

Canonical term registry for Bion / infinite-db namespace disjointness.
CI lint reads `DB_TERMS` and fails if any appear in `crates/bion-soma/src`.

## BION_TERMS

NeuronId, NeuronType, SignalType, Impulse, Polarity, RoutingLabel, FiberId,
ValidSynapse, BoolValue, IntValue, FloatValue, SignalText, ByteBlob, UnitValue,
PnsReader, PnsWriter, PnsWatcher, DefinitionChange, StateUpdate, ImpulseEnvelope,
Ganglion, GanglionId, Synapse, SynapseId, Fiber, Circuit, Executor,
Pacemaker, ActionWave, ActionPotential, Transmission, Threshold,
CliTransport, WsTransport

## DB_TERMS

InfiniteDb, HyperedgeId, Hyperedge, SpaceId, RevisionId, BranchId,
SnapshotId, DimensionVector, HilbertKey, WalEntry, Record, SignalSample,
DerivationBus, DerivationSubscriber, WriteSession, WriteJob
