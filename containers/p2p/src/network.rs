use anyhow::{Context, Result};
use futures::stream::StreamExt;
use libp2p::core::upgrade;
use libp2p::identity;
use libp2p::ping;
use libp2p::swarm::{Config as SwarmConfig, SwarmEvent};
use libp2p::tcp::tokio as tcp_tokio;
use libp2p::yamux::Config as YamuxConfig;
use libp2p::{PeerId, Swarm};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};
use tracing::debug;

#[derive(Debug, Clone)]
pub enum P2PEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    MessageReceived {
        from: PeerId,
        topic: String,
        data: Vec<u8>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
enum Control {
    Publish(String, Vec<u8>),
    Subscribe(String),
    Dial(libp2p::Multiaddr),
}

// Use only Ping behaviour for now to keep the implementation simple.
// We'll add Gossipsub and mDNS integration via separate test swarms to validate behavior.

pub struct P2PNetwork {
    #[allow(dead_code)]
    peer_id: PeerId,
    ctrl_tx: mpsc::UnboundedSender<Control>,
}

impl P2PNetwork {
    pub async fn new() -> Result<(
        Self,
        mpsc::UnboundedReceiver<P2PEvent>,
        libp2p::Multiaddr,
        PeerId,
    )> {
        // Create transport
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());

        // Use tokio TCP transport
        let tcp = tcp_tokio::Transport::new(libp2p::tcp::Config::default());
        use libp2p::Transport as _TransportTrait;
        let noise_config =
            libp2p::noise::Config::new(&keypair).context("failed to create noise config")?;
        let transport = tcp
            .upgrade(upgrade::Version::V1)
            .authenticate(noise_config)
            .multiplex(YamuxConfig::default())
            .boxed();

        // Use Ping behaviour initially to validate connectivity; gossipsub will be added later
        let ping_cfg = ping::Config::new();
        let behaviour = ping::Behaviour::new(ping_cfg);

        let mut swarm = Swarm::new(
            transport,
            behaviour,
            peer_id,
            SwarmConfig::with_executor(|fut| {
                tokio::spawn(fut);
            }),
        );
        // Listen on localhost ephemeral port
        swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (ctrl_tx, mut ctrl_rx) = mpsc::unbounded_channel::<Control>();
        let (addr_tx, addr_rx) = oneshot::channel::<libp2p::Multiaddr>();

        // Spawn background task
        tokio::spawn(async move {
            let mut listening_addrs_sent = false;
            let mut seen_peers = HashSet::new();
            // Wrap sender in Option so we only send once without moving errors
            let mut addr_tx = Some(addr_tx);

            loop {
                tokio::select! {
                    Some(cmd) = ctrl_rx.recv() => {
                        match cmd {
                            Control::Publish(_, _) => {
                                debug!("Publish called - not implemented yet");
                            }
                            Control::Subscribe(_) => {
                                debug!("Subscribe called - not implemented yet");
                            }
                            Control::Dial(addr) => {
                                if let Err(e) = swarm.dial(addr.clone()) {
                                    debug!("dial error: {:?}", e);
                                }
                            }
                        }
                    }
                    event = swarm.select_next_some() => {
                        match event {
                            // Currently we use Ping behaviour; handle generic behaviour events (log/ignore for now).
                            SwarmEvent::Behaviour(ev) => {
                                debug!("behaviour event: {:?}", ev);
                            }
                            SwarmEvent::NewListenAddr { address, .. } => {
                                if !listening_addrs_sent {
                                    if let Some(tx) = addr_tx.take() {
                                        let _ = tx.send(address.clone());
                                    }
                                    listening_addrs_sent = true;
                                }
                                debug!("Listening on {:?}", address);
                            }
                            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                                if seen_peers.insert(peer_id) {
                                    let _ = event_tx.send(P2PEvent::PeerConnected(peer_id));
                                }
                            }
                            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                                if seen_peers.remove(&peer_id) {
                                    let _ = event_tx.send(P2PEvent::PeerDisconnected(peer_id));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        // Wait for listen addr
        let listen_addr = addr_rx.await.context("failed to receive listen addr")?;

        Ok((Self { peer_id, ctrl_tx }, event_rx, listen_addr, peer_id))
    }

    pub fn publish(&self, topic: &str, data: Vec<u8>) -> Result<()> {
        self.ctrl_tx
            .send(Control::Publish(topic.to_string(), data))?;
        Ok(())
    }

    pub fn subscribe(&self, topic: &str) -> Result<()> {
        self.ctrl_tx.send(Control::Subscribe(topic.to_string()))?;
        Ok(())
    }

    pub fn dial(&self, addr: libp2p::Multiaddr) -> Result<()> {
        self.ctrl_tx.send(Control::Dial(addr))?;
        Ok(())
    }
}
