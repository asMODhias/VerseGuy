#![allow(clippy::disallowed_methods, clippy::collapsible_if)]
use libp2p::multiaddr::Protocol;
use std::time::Duration;
use verseguy_p2p::network::{P2PNetwork, P2PEvent};

#[tokio::test]
async fn messages_between_two_peers() {
    let (_node_a, _rx_a, addr_a, id_a) = verseguy_test_utils::must(P2PNetwork::new().await);
    let (node_b, mut rx_b, _addr_b, _id_b) = verseguy_test_utils::must(P2PNetwork::new().await);

    // Dial b -> a
    let mut dial_addr = addr_a.clone();
    dial_addr.push(Protocol::P2p(id_a));
    verseguy_test_utils::must(node_b.dial(dial_addr));

    // small wait for connection
    tokio::time::sleep(Duration::from_millis(200)).await;

    // wait for PeerConnected event on b
    let got = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(ev) = rx_b.recv().await {
            if let P2PEvent::PeerConnected(pid) = ev {
                if pid == id_a {
                    return true;
                }
            }
        }
        false
    }).await.unwrap_or(false);

    assert!(got, "node_b did not observe connection to node_a");
}
