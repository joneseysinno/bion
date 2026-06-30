//! In-memory PNS gateway for integration tests.

use std::sync::Arc;

use async_trait::async_trait;
use bion_pns::{
    FiberDescriptor, ImpulseEnvelope, NeuronDescriptor, PnsError, PnsReader, PnsWatcher,
    PnsWriter, RevisionAck, SubgraphSnapshot,
};
use bion_soma::{IdSeed, Impulse, NeuronId, NeuronType};
use tokio::sync::mpsc;

/// Mock reader returning a minimal subgraph for any root.
pub struct MockReader;

#[async_trait]
impl PnsReader for MockReader {
    async fn fetch_neuron(&self, id: NeuronId) -> Result<NeuronDescriptor, PnsError> {
        Ok(NeuronDescriptor {
            id,
            neuron_type: NeuronType::Interneuron,
            revision: 1,
        })
    }

    async fn fetch_subgraph(&self, root: NeuronId) -> Result<SubgraphSnapshot, PnsError> {
        Ok(SubgraphSnapshot {
            root,
            revision: 1,
            neurons: vec![NeuronDescriptor {
                id: root,
                neuron_type: NeuronType::Interneuron,
                revision: 1,
            }],
            fibers: Vec::new(),
        })
    }

    async fn fetch_fibers(&self, _neuron: NeuronId) -> Result<Vec<FiberDescriptor>, PnsError> {
        Ok(Vec::new())
    }
}

/// Mock writer that always returns revision 1.
pub struct MockWriter;

#[async_trait]
impl PnsWriter for MockWriter {
    async fn write_state(
        &self,
        _neuron: NeuronId,
        _impulse: Impulse,
    ) -> Result<RevisionAck, PnsError> {
        Ok(RevisionAck { revision: 1 })
    }
}

/// Mock watcher with no events.
pub struct MockWatcher {
    rx: mpsc::Receiver<bion_pns::DefinitionChange>,
}

impl MockWatcher {
    /// Create a watcher whose channel is immediately closed.
    pub fn closed() -> Box<dyn PnsWatcher> {
        let (_tx, rx) = mpsc::channel(1);
        Box::new(Self { rx })
    }
}

#[async_trait]
impl PnsWatcher for MockWatcher {
    async fn recv(&mut self) -> Option<bion_pns::DefinitionChange> {
        self.rx.recv().await
    }
}

/// Root neuron id for tests.
pub fn test_root() -> NeuronId {
    IdSeed::first().mint_neuron().unwrap().0
}

/// Sample impulse envelope.
pub fn test_envelope(root: NeuronId) -> ImpulseEnvelope {
    use bion_soma::{BoolValue, Impulse};
    ImpulseEnvelope {
        target: root,
        impulse: Impulse::Bool(BoolValue::new(true)),
        client_revision: None,
    }
}

/// Build mock gateway triple.
pub fn mock_gateway() -> (
    Arc<dyn PnsReader>,
    Arc<dyn PnsWriter>,
    Box<dyn PnsWatcher>,
) {
    (
        Arc::new(MockReader),
        Arc::new(MockWriter),
        MockWatcher::closed(),
    )
}
