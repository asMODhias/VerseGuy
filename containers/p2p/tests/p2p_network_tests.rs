use libp2p::multiaddr::Protocol;
use std::time::Duration;
use verseguy_p2p::network::{P2PNetwork, P2PEvent};

#[tokio::test]
async fn messages_between_two_peers() {
    let (node_a, mut rx_a, addr_a, id_a) = P2PNetwork::new().await.expect("create a");
    let (node_b, mut rx_b, addr_b, id_b) = P2PNetwork::new().await.expect("create b");

    // Dial b -> a
    let mut dial_addr = addr_a.clone();
    dial_addr.push(Protocol::P2p(id_a));
    node_b.dial(dial_addr).expect("dial");

    // small wait for connection
    tokio::time::sleep(Duration::from_millis(200)).await;

    // wait for PeerConnected event on b
    let got = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(ev) = rx_b.recv().await {
            match ev {
                P2PEvent::PeerConnected(pid) => {
                    if pid == id_a {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }).await.unwrap_or(false);

    assert!(got, "node_b did not observe connection to node_a");
}
