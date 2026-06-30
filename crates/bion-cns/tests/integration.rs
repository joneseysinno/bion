mod mock_gateway;

use bion_cns::start;
use bion_soma::BoolValue;
use mock_gateway::{mock_gateway, test_envelope, test_root};
use tokio::sync::mpsc;

#[tokio::test]
async fn cns_starts_with_mock_gateway() {
    let root = test_root();
    let (reader, writer, watcher) = mock_gateway();
    let (_tx, impulse_rx) = mpsc::channel(4);
    let (runtime, circuit, _subs) = start(reader, writer, watcher, root, impulse_rx)
        .await
        .expect("start");
    let guard = circuit.read().await;
    assert_eq!(guard.root, Some(root));
    bion_cns::shutdown(runtime).await;
}

#[tokio::test]
async fn execution_loop_persists_impulse() {
    let root = test_root();
    let (reader, writer, watcher) = mock_gateway();
    let (impulse_tx, impulse_rx) = mpsc::channel(4);
    let (runtime, circuit, subs) = start(reader, writer, watcher, root, impulse_rx)
        .await
        .expect("start");

    let (out_tx, mut out_rx) = mpsc::channel(4);
    subs.write().await.push(out_tx);

    impulse_tx
        .send(test_envelope(root))
        .await
        .expect("send impulse");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let update = out_rx.try_recv().expect("state update");
    assert_eq!(update.source, root);
    assert_eq!(
        update.impulse,
        bion_soma::Impulse::Bool(BoolValue::new(true))
    );

    bion_cns::shutdown(runtime).await;
    let _ = circuit;
}
