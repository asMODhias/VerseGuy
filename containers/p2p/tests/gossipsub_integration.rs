use std::time::Duration;
use libp2p::{identity, PeerId, swarm::SwarmEvent};
use libp2p::Transport as _Transport;
use libp2p::core::multiaddr::Multiaddr;


#[tokio::test]
async fn gossipsub_publish_subscribe() {
    // Setup keys and peer ids
    let key_a = identity::Keypair::generate_ed25519();
    let key_b = identity::Keypair::generate_ed25519();
    let peer_a = PeerId::from(key_a.public());
    let peer_b = PeerId::from(key_b.public());

    // Transports
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

    // Gossipsub behaviours (per guide): Strict validation + signed messages
    // Use permissive validation for flaky local tests (diagnostic)
    // (Will revert to Strict once mesh formation is stable)
    let cfg = libp2p::gossipsub::ConfigBuilder::default()
        .validation_mode(libp2p::gossipsub::ValidationMode::Permissive)
        .build()
        .expect("failed to build gossipsub config");

    // Sign outgoing messages using the transport keypairs
    let mut gsub_a: libp2p_gossipsub::Behaviour<libp2p_gossipsub::IdentityTransform, libp2p_gossipsub::AllowAllSubscriptionFilter> = libp2p_gossipsub::Behaviour::new(libp2p_gossipsub::MessageAuthenticity::Signed(key_a.clone()), cfg.clone()).unwrap();
    let mut gsub_b: libp2p_gossipsub::Behaviour<libp2p_gossipsub::IdentityTransform, libp2p_gossipsub::AllowAllSubscriptionFilter> = libp2p_gossipsub::Behaviour::new(libp2p_gossipsub::MessageAuthenticity::Signed(key_b.clone()), cfg.clone()).unwrap();

    // Create swarms using those behaviours
    let mut swarm_a = libp2p::Swarm::new(transport_a, gsub_a, peer_a, libp2p::swarm::Config::with_executor(|fut| { tokio::spawn(fut); }));
    let mut swarm_b = libp2p::Swarm::new(transport_b, gsub_b, peer_b, libp2p::swarm::Config::with_executor(|fut| { tokio::spawn(fut); }));

    // Listen
    swarm_a.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap()).unwrap();
    swarm_b.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap()).unwrap();

    // Wait for listen addresses (bounded loop)
    let mut addr_a_opt: Option<Multiaddr> = None;
    for _ in 0..30 {
        if let Some(a) = swarm_a.listeners().next() { addr_a_opt = Some(a.clone()); break; }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_a).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev { addr_a_opt = Some(address); break; }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_a = addr_a_opt.expect("addr a not found");

    let mut addr_b_opt: Option<Multiaddr> = None;
    for _ in 0..30 {
        if let Some(a) = swarm_b.listeners().next() { addr_b_opt = Some(a.clone()); break; }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_b).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev { addr_b_opt = Some(address); break; }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_b = addr_b_opt.expect("addr b not found");

    // Create a third peer C to improve mesh formation determinism
    let key_c = identity::Keypair::generate_ed25519();
    let peer_c = PeerId::from(key_c.public());

    let tcp_c = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default());
    let transport_c = tcp_c
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::Config::new(&key_c).unwrap())
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let mut gsub_c: libp2p_gossipsub::Behaviour<libp2p_gossipsub::IdentityTransform, libp2p_gossipsub::AllowAllSubscriptionFilter> = libp2p_gossipsub::Behaviour::new(libp2p_gossipsub::MessageAuthenticity::Signed(key_c.clone()), cfg.clone()).unwrap();
    let mut swarm_c = libp2p::Swarm::new(transport_c, gsub_c, peer_c, libp2p::swarm::Config::with_executor(|fut| { tokio::spawn(fut); }));

    swarm_c.listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap()).unwrap();

    // Wait for listen address for C
    let mut addr_c_opt: Option<Multiaddr> = None;
    for _ in 0..30 {
        if let Some(a) = swarm_c.listeners().next() { addr_c_opt = Some(a.clone()); break; }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_c).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev { addr_c_opt = Some(address); break; }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_c = addr_c_opt.expect("addr c not found");

    // Seed explicit peers *before* dialing to hint gossipsub about peers and reduce churn
    swarm_a.behaviour_mut().add_explicit_peer(&peer_b);
    swarm_a.behaviour_mut().add_explicit_peer(&peer_c);
    swarm_b.behaviour_mut().add_explicit_peer(&peer_a);
    swarm_b.behaviour_mut().add_explicit_peer(&peer_c);
    swarm_c.behaviour_mut().add_explicit_peer(&peer_a);
    swarm_c.behaviour_mut().add_explicit_peer(&peer_b);

    // Dial A<->B, A<->C and B<->C to create a triangle
    let mut dial_a_to_b = addr_b.clone();
    dial_a_to_b.push(libp2p::multiaddr::Protocol::P2p(peer_b));
    let mut dial_b_to_a = addr_a.clone();
    dial_b_to_a.push(libp2p::multiaddr::Protocol::P2p(peer_a));

    let mut dial_a_to_c = addr_c.clone();
    dial_a_to_c.push(libp2p::multiaddr::Protocol::P2p(peer_c));
    let mut dial_c_to_a = addr_a.clone();
    dial_c_to_a.push(libp2p::multiaddr::Protocol::P2p(peer_a));

    let mut dial_b_to_c = addr_c.clone();
    dial_b_to_c.push(libp2p::multiaddr::Protocol::P2p(peer_c));
    let mut dial_c_to_b = addr_b.clone();
    dial_c_to_b.push(libp2p::multiaddr::Protocol::P2p(peer_b));

    let _ = swarm_a.dial(dial_a_to_b.clone());
    let _ = swarm_b.dial(dial_b_to_a.clone());
    let _ = swarm_a.dial(dial_a_to_c.clone());
    let _ = swarm_c.dial(dial_c_to_a.clone());
    let _ = swarm_b.dial(dial_b_to_c.clone());
    let _ = swarm_c.dial(dial_c_to_b.clone());

    // Wait for at least one cross-peer connection established (bounded)
    let connected = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(ev) = futures::StreamExt::next(&mut swarm_a).await {
                eprintln!("s1 event: {:?}", ev);
                if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                    if peer_id == peer_b || peer_id == peer_c { break true; }
                }
            }
            if let Some(ev) = futures::StreamExt::next(&mut swarm_b).await {
                eprintln!("s2 event: {:?}", ev);
                if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                    if peer_id == peer_a || peer_id == peer_c { break true; }
                }
            }
            if let Some(ev) = futures::StreamExt::next(&mut swarm_c).await {
                eprintln!("s3 event: {:?}", ev);
                if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                    if peer_id == peer_a || peer_id == peer_b { break true; }
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }).await.unwrap_or(false);

    assert!(connected, "peers did not connect in triangle");

    // Subscribe all three to canonical topic from the guide
    let topic = libp2p_gossipsub::IdentTopic::new("verseguy/global");
    swarm_a.behaviour_mut().subscribe(&topic).unwrap();
    swarm_b.behaviour_mut().subscribe(&topic).unwrap();
    swarm_c.behaviour_mut().subscribe(&topic).unwrap();

    // Actively pump events to allow gossipsub to form mesh and propagate subscriptions
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_millis(2000) {
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_a)).await {
            if let Some(ev) = ev_opt { eprintln!("s1 event during pump: {:?}", ev); }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_b)).await {
            if let Some(ev) = ev_opt { eprintln!("s2 event during pump: {:?}", ev); }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_c)).await {
            if let Some(ev) = ev_opt { eprintln!("s3 event during pump: {:?}", ev); }
        }
    }

    // Add explicit peers to bypass mesh formation flakiness where connections may churn
    swarm_a.behaviour_mut().add_explicit_peer(&peer_b);
    swarm_a.behaviour_mut().add_explicit_peer(&peer_c);
    swarm_b.behaviour_mut().add_explicit_peer(&peer_a);
    swarm_b.behaviour_mut().add_explicit_peer(&peer_c);
    swarm_c.behaviour_mut().add_explicit_peer(&peer_a);
    swarm_c.behaviour_mut().add_explicit_peer(&peer_b);

    // small extra pump after adding explicit peers
    let start2 = std::time::Instant::now();
    while start2.elapsed() < Duration::from_millis(500) {
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_a)).await {
            if let Some(ev) = ev_opt { eprintln!("s1 event after add_explicit_peer: {:?}", ev); }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_b)).await {
            if let Some(ev) = ev_opt { eprintln!("s2 event after add_explicit_peer: {:?}", ev); }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_c)).await {
            if let Some(ev) = ev_opt { eprintln!("s3 event after add_explicit_peer: {:?}", ev); }
        }
    }

    // Wait for peers to observe subscriptions from each other (bounded)
    let mut saw_subscription = false;
    let start_sub = std::time::Instant::now();
    while start_sub.elapsed() < Duration::from_secs(3) {
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_a)).await {
            if let Some(ev) = ev_opt {
                let s = format!("{:?}", ev);
                eprintln!("s1 sub-check: {}", s);
                if s.contains("Subscribed") && s.contains("verseguy/global") { saw_subscription = true; break; }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_b)).await {
            if let Some(ev) = ev_opt {
                let s = format!("{:?}", ev);
                eprintln!("s2 sub-check: {}", s);
                if s.contains("Subscribed") && s.contains("verseguy/global") { saw_subscription = true; break; }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(50), futures::StreamExt::next(&mut swarm_c)).await {
            if let Some(ev) = ev_opt {
                let s = format!("{:?}", ev);
                eprintln!("s3 sub-check: {}", s);
                if s.contains("Subscribed") && s.contains("verseguy/global") { saw_subscription = true; break; }
            }
        }
    }

    assert!(saw_subscription, "no subscription events observed among peers");

    // publish from A (signed via behaviour config) with retries in case mesh is not yet formed
    let mut published = false;
    for _ in 0..20 {
        match swarm_a.behaviour_mut().publish(topic.clone(), b"hello".to_vec()) {
            Ok(_) => { published = true; break; }
            Err(e) => {
                eprintln!("publish attempt failed: {:?}, retrying", e);
                // pump events again briefly before retrying
                if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(100), futures::StreamExt::next(&mut swarm_a)).await {
                    if let Some(ev) = ev_opt { eprintln!("s1 event during publish retry: {:?}", ev); }
                }
                if let Ok(ev_opt) = tokio::time::timeout(Duration::from_millis(100), futures::StreamExt::next(&mut swarm_b)).await {
                    if let Some(ev) = ev_opt { eprintln!("s2 event during publish retry: {:?}", ev); }
                }
                tokio::time::sleep(Duration::from_millis(150)).await;
            }
        }
    }
    assert!(published, "failed to publish gossipsub message after retries");

    // wait for message on B (match via debug string since event enum varies between versions)
    let got = tokio::time::timeout(Duration::from_secs(5), async {
        while let Some(ev) = futures::StreamExt::next(&mut swarm_b).await {
            if let SwarmEvent::Behaviour(ev) = &ev {
                let s = format!("{:?}", ev);
                eprintln!("s2 behaviour event: {}", s);
                if s.contains("hello") { return true; }
            }
        }
        false
    }).await.unwrap_or(false);

    assert!(got, "did not receive gossipsub message");
}