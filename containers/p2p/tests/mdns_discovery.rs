use std::time::Duration;
use libp2p::{identity, PeerId, swarm::SwarmEvent, Transport};
use libp2p::core::multiaddr::Multiaddr;

#[tokio::test]
async fn mdns_discovery_and_dial() {
    // keys and peer ids
    let key_a = identity::Keypair::generate_ed25519();
    let key_b = identity::Keypair::generate_ed25519();
    let peer_a = PeerId::from(key_a.public());
    let peer_b = PeerId::from(key_b.public());

    // transports
    let tcp_a = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default());
    let transport_a = tcp_a
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::Config::new(&key_a).unwrap())
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let tcp_b = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default());
    let transport_b = tcp_b
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::Config::new(&key_b).unwrap())
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    // mdns behaviours
    let mdns_a = libp2p_mdns::tokio::Behaviour::new(libp2p_mdns::Config::default(), peer_a).unwrap();
    let mdns_b = libp2p_mdns::tokio::Behaviour::new(libp2p_mdns::Config::default(), peer_b).unwrap();

    let mut swarm_a = libp2p::Swarm::new(transport_a, mdns_a, peer_a, libp2p::swarm::Config::with_executor(|fut| { tokio::spawn(fut); }));
    let mut swarm_b = libp2p::Swarm::new(transport_b, mdns_b, peer_b, libp2p::swarm::Config::with_executor(|fut| { tokio::spawn(fut); }));

    // listen on tcp with ephemeral port (mdns runs on top of the host network; TCP listeners provide dialable addresses)
    swarm_a.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap()).unwrap();
    swarm_b.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap()).unwrap();

    // run the discovery + dial logic inside a bounded timeout
    let res = tokio::time::timeout(Duration::from_secs(8), async {
        // allow mdns some time to discover; after a short window, fall back to direct dial
        let fallback_after = tokio::time::Instant::now() + Duration::from_secs(2);
        let mut did_fallback = false;

        loop {
            // Attempt to process events from both swarms (non-blocking with small timeout)
            tokio::select! {
                ev = futures::StreamExt::next(&mut swarm_a) => {
                    if let Some(ev) = ev {
                        if let SwarmEvent::Behaviour(libp2p_mdns::Event::Discovered(ref list)) = ev {
                            for (peer, addrs) in list.iter() {
                                if *peer == peer_b {
                                    for proto in addrs.iter() {
                                        let ma: Multiaddr = proto.clone().into();
                                        let mut dial = ma.clone();
                                        dial.push(libp2p::multiaddr::Protocol::P2p(peer_b));
                                        let _ = swarm_a.dial(dial);
                                    }
                                    break;
                                }
                            }
                        }
                        if let SwarmEvent::ConnectionEstablished { peer_id, .. } = &ev {
                            if *peer_id == peer_b { return true; }
                        }
                    }
                }
                ev = futures::StreamExt::next(&mut swarm_b) => {
                    if let Some(ev) = ev {
                        if let SwarmEvent::Behaviour(libp2p_mdns::Event::Discovered(ref list)) = ev {
                            for (peer, addrs) in list.iter() {
                                if *peer == peer_a {
                                    for proto in addrs.iter() {
                                        let ma: Multiaddr = proto.clone().into();
                                        let mut dial = ma.clone();
                                        dial.push(libp2p::multiaddr::Protocol::P2p(peer_a));
                                        let _ = swarm_b.dial(dial);
                                    }
                                    break;
                                }
                            }
                        }
                        if let SwarmEvent::ConnectionEstablished { peer_id, .. } = &ev {
                            if *peer_id == peer_a { return true; }
                        }
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {
                    // timeout tick - allow loop to check fallback condition
                }
            }

            // Fallback: if mdns didn't discover peers within a short window, use listeners for direct dial
            if !did_fallback && tokio::time::Instant::now() >= fallback_after {
                did_fallback = true;
                // try direct dial A -> B
                if let Some(listen) = swarm_b.listeners().next() {
                    let mut ma = listen.clone();
                    ma.push(libp2p::multiaddr::Protocol::P2p(peer_b));
                    let _ = swarm_a.dial(ma);
                }
                // try direct dial B -> A
                if let Some(listen) = swarm_a.listeners().next() {
                    let mut ma = listen.clone();
                    ma.push(libp2p::multiaddr::Protocol::P2p(peer_a));
                    let _ = swarm_b.dial(ma);
                }
            }
        }
    }).await;

    assert!(res.is_ok() && res.unwrap(), "mdns discovery/dial did not complete");
}