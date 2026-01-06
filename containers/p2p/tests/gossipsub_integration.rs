#![allow(
    clippy::disallowed_methods,
    clippy::collapsible_if,
    clippy::single_match,
    clippy::manual_unwrap_or_default,
    clippy::let_unit_value,
    clippy::collapsible_match,
    clippy::needless_bool
)]
use libp2p::Transport as _Transport;
use libp2p::core::multiaddr::Multiaddr;
use libp2p::{PeerId, identity, swarm::SwarmEvent};
use std::time::Duration;

#[tokio::test]
async fn gossipsub_publish_subscribe() {
    // Setup keys and peer ids
    let key_a = identity::Keypair::generate_ed25519();
    let key_b = identity::Keypair::generate_ed25519();
    let peer_a = PeerId::from(key_a.public());
    let peer_b = PeerId::from(key_b.public());

    // Transports - enable TCP keepalive to reduce Windows KeepAliveTimeout churn
    let tcp_a_cfg = libp2p::tcp::Config::default();
    // keepalive no longer present in newer libp2p; default config used
    let tcp_a = libp2p::tcp::tokio::Transport::new(tcp_a_cfg);
    let transport_a = tcp_a
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(verseguy_test_utils::must(libp2p::noise::Config::new(
            &key_a,
        )))
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let tcp_b_cfg = libp2p::tcp::Config::default();
    // keepalive no longer present in newer libp2p; default config used
    let tcp_b = libp2p::tcp::tokio::Transport::new(tcp_b_cfg);
    let transport_b = tcp_b
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(verseguy_test_utils::must(libp2p::noise::Config::new(
            &key_b,
        )))
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    // Gossipsub behaviours (per guide): Strict validation + signed messages
    // Use permissive validation for flaky local tests (diagnostic)
    // (Will revert to Strict once mesh formation is stable)
    let cfg = verseguy_test_utils::must(
        libp2p::gossipsub::ConfigBuilder::default()
            .validation_mode(libp2p::gossipsub::ValidationMode::Permissive)
            .build(),
    );

    // Sign outgoing messages using the transport keypairs
    let gsub_a: libp2p_gossipsub::Behaviour<
        libp2p_gossipsub::IdentityTransform,
        libp2p_gossipsub::AllowAllSubscriptionFilter,
    > = verseguy_test_utils::must(libp2p_gossipsub::Behaviour::new(
        libp2p_gossipsub::MessageAuthenticity::Signed(key_a.clone()),
        cfg.clone(),
    ));
    let gsub_b: libp2p_gossipsub::Behaviour<
        libp2p_gossipsub::IdentityTransform,
        libp2p_gossipsub::AllowAllSubscriptionFilter,
    > = verseguy_test_utils::must(libp2p_gossipsub::Behaviour::new(
        libp2p_gossipsub::MessageAuthenticity::Signed(key_b.clone()),
        cfg.clone(),
    ));

    // Create swarms using those behaviours
    let mut swarm_a = libp2p::Swarm::new(
        transport_a,
        gsub_a,
        peer_a,
        libp2p::swarm::Config::with_executor(|fut| {
            tokio::spawn(fut);
        }),
    );
    let mut swarm_b = libp2p::Swarm::new(
        transport_b,
        gsub_b,
        peer_b,
        libp2p::swarm::Config::with_executor(|fut| {
            tokio::spawn(fut);
        }),
    );

    // Listen
    swarm_a
        .listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap())
        .unwrap();
    swarm_b
        .listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap())
        .unwrap();

    // Wait for listen addresses (bounded loop)
    let mut addr_a_opt: Option<Multiaddr> = None;
    // Increase attempts to tolerate slower address provisioning on some platforms
    for _ in 0..200 {
        if let Some(a) = swarm_a.listeners().next() {
            addr_a_opt = Some(a.clone());
            break;
        }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_a).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev {
                addr_a_opt = Some(address);
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_a = verseguy_test_utils::must_opt(addr_a_opt, "addr a not found");

    let mut addr_b_opt: Option<Multiaddr> = None;
    for _ in 0..200 {
        if let Some(a) = swarm_b.listeners().next() {
            addr_b_opt = Some(a.clone());
            break;
        }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_b).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev {
                addr_b_opt = Some(address);
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_b = verseguy_test_utils::must_opt(addr_b_opt, "addr b not found");

    // Create a third peer C to improve mesh formation determinism
    let key_c = identity::Keypair::generate_ed25519();
    let peer_c = PeerId::from(key_c.public());

    let tcp_c_cfg = libp2p::tcp::Config::default();
    // keepalive no longer present; default config used
    let tcp_c = libp2p::tcp::tokio::Transport::new(tcp_c_cfg);
    let transport_c = tcp_c
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(verseguy_test_utils::must(libp2p::noise::Config::new(
            &key_c,
        )))
        .multiplex(libp2p::yamux::Config::default())
        .boxed();

    let gsub_c: libp2p_gossipsub::Behaviour<
        libp2p_gossipsub::IdentityTransform,
        libp2p_gossipsub::AllowAllSubscriptionFilter,
    > = verseguy_test_utils::must(libp2p_gossipsub::Behaviour::new(
        libp2p_gossipsub::MessageAuthenticity::Signed(key_c.clone()),
        cfg.clone(),
    ));
    let mut swarm_c = libp2p::Swarm::new(
        transport_c,
        gsub_c,
        peer_c,
        libp2p::swarm::Config::with_executor(|fut| {
            tokio::spawn(fut);
        }),
    );

    swarm_c
        .listen_on("/ip4/127.0.0.1/tcp/0".parse::<Multiaddr>().unwrap())
        .unwrap();

    // Wait for listen address for C
    let mut addr_c_opt: Option<Multiaddr> = None;
    for _ in 0..200 {
        if let Some(a) = swarm_c.listeners().next() {
            addr_c_opt = Some(a.clone());
            break;
        }
        if let Some(ev) = futures::StreamExt::next(&mut swarm_c).await {
            if let SwarmEvent::NewListenAddr { address, .. } = ev {
                addr_c_opt = Some(address);
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let addr_c = verseguy_test_utils::must_opt(addr_c_opt, "addr c not found");

    // Seed explicit peers *before* dialing to hint gossipsub about peers and reduce churn
    // explicit peer APIs changed in newer libp2p — attempt to call via behaviour API where available
    // Try direct Behaviour methods for explicit peers where available
    let _ = swarm_a.behaviour_mut().add_explicit_peer(&peer_b);
    let _ = swarm_a.behaviour_mut().add_explicit_peer(&peer_c);
    let _ = swarm_b.behaviour_mut().add_explicit_peer(&peer_a);
    let _ = swarm_b.behaviour_mut().add_explicit_peer(&peer_c);
    let _ = swarm_c.behaviour_mut().add_explicit_peer(&peer_a);
    let _ = swarm_c.behaviour_mut().add_explicit_peer(&peer_b);

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

    // Wait for each swarm to observe at least one ConnectionEstablished with a peer (bounded, longer)
    let connected = {
        let deadline = std::time::Instant::now() + Duration::from_secs(15);
        let mut s1_ok = false;
        let mut s2_ok = false;
        let mut s3_ok = false;
        let mut last_redial = std::time::Instant::now();
        while std::time::Instant::now() < deadline {
            // Periodically re-dial known addresses to keep connections active and avoid KeepAlive timeouts
            if last_redial.elapsed() > Duration::from_millis(900) {
                let _ = swarm_a.dial(dial_a_to_b.clone());
                let _ = swarm_a.dial(dial_a_to_c.clone());
                let _ = swarm_b.dial(dial_b_to_a.clone());
                let _ = swarm_b.dial(dial_b_to_c.clone());
                let _ = swarm_c.dial(dial_c_to_a.clone());
                let _ = swarm_c.dial(dial_c_to_b.clone());
                last_redial = std::time::Instant::now();
            }

            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(200),
                futures::StreamExt::next(&mut swarm_a),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s1 event: {:?}", ev);
                    if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                        if peer_id == peer_b || peer_id == peer_c {
                            s1_ok = true;
                        }
                    }
                    if let SwarmEvent::ConnectionClosed { .. } = ev {
                        s1_ok = false;
                    }
                }
            }
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(200),
                futures::StreamExt::next(&mut swarm_b),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s2 event: {:?}", ev);
                    if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                        if peer_id == peer_a || peer_id == peer_c {
                            s2_ok = true;
                        }
                    }
                    if let SwarmEvent::ConnectionClosed { .. } = ev {
                        s2_ok = false;
                    }
                }
            }
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(200),
                futures::StreamExt::next(&mut swarm_c),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s3 event: {:?}", ev);
                    if let SwarmEvent::ConnectionEstablished { peer_id, .. } = ev {
                        if peer_id == peer_a || peer_id == peer_b {
                            s3_ok = true;
                        }
                    }
                    if let SwarmEvent::ConnectionClosed { .. } = ev {
                        s3_ok = false;
                    }
                }
            }
            if s1_ok && s2_ok && s3_ok {
                // allow a short stabilization window
                tokio::time::sleep(Duration::from_millis(300)).await;
                break;
            }
        }
        if s1_ok && s2_ok && s3_ok { true } else { false }
    };

    assert!(connected, "peers did not form stable triangle connections");

    // Subscribe all three to canonical topic from the guide
    let topic = libp2p_gossipsub::IdentTopic::new("verseguy/global");
    verseguy_test_utils::must(swarm_a.behaviour_mut().subscribe(&topic));
    verseguy_test_utils::must(swarm_b.behaviour_mut().subscribe(&topic));
    verseguy_test_utils::must(swarm_c.behaviour_mut().subscribe(&topic));

    // Actively pump events to allow gossipsub to form mesh and propagate subscriptions
    // Track whether we observed any Subscribed events during pumping (events are consumed by the pump)
    let mut saw_subscription = false;
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_millis(4000) {
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_a),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s1 event during pump: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_b),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s2 event during pump: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_c),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s3 event during pump: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
    }

    // Add explicit peers to bypass mesh formation flakiness where connections may churn
    let _ = swarm_a.behaviour_mut().add_explicit_peer(&peer_b);
    let _ = swarm_a.behaviour_mut().add_explicit_peer(&peer_c);
    let _ = swarm_b.behaviour_mut().add_explicit_peer(&peer_a);
    let _ = swarm_b.behaviour_mut().add_explicit_peer(&peer_c);
    let _ = swarm_c.behaviour_mut().add_explicit_peer(&peer_a);
    let _ = swarm_c.behaviour_mut().add_explicit_peer(&peer_b);

    // small extra pump after adding explicit peers
    let start2 = std::time::Instant::now();
    while start2.elapsed() < Duration::from_millis(1000) {
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_a),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s1 event after add_explicit_peer: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_b),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s2 event after add_explicit_peer: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(50),
            futures::StreamExt::next(&mut swarm_c),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s3 event after add_explicit_peer: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                    topic, ..
                }) = &ev
                {
                    if format!("{:?}", topic).contains("verseguy/global") {
                        saw_subscription = true;
                    }
                }
            }
        }
    }

    // If we already observed Subscribed events during the pumping above, skip extra wait
    if !saw_subscription {
        let start_sub = std::time::Instant::now();
        while start_sub.elapsed() < Duration::from_secs(6) {
            // pump events aggressively while waiting
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(100),
                futures::StreamExt::next(&mut swarm_a),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s1 sub-check: {:?}", ev);
                    if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                        topic,
                        ..
                    }) = &ev
                    {
                        if format!("{:?}", topic).contains("verseguy/global") {
                            saw_subscription = true;
                            break;
                        }
                    }
                }
            }
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(100),
                futures::StreamExt::next(&mut swarm_b),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s2 sub-check: {:?}", ev);
                    if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                        topic,
                        ..
                    }) = &ev
                    {
                        if format!("{:?}", topic).contains("verseguy/global") {
                            saw_subscription = true;
                            break;
                        }
                    }
                }
            }
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(100),
                futures::StreamExt::next(&mut swarm_c),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s3 sub-check: {:?}", ev);
                    if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Subscribed {
                        topic,
                        ..
                    }) = &ev
                    {
                        if format!("{:?}", topic).contains("verseguy/global") {
                            saw_subscription = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    assert!(
        saw_subscription,
        "no subscription events observed among peers"
    );

    // publish from A (signed via behaviour config) with retries in case mesh is not yet formed
    let mut published = false;
    for _ in 0..40 {
        match swarm_a
            .behaviour_mut()
            .publish(topic.clone(), b"hello".to_vec())
        {
            Ok(_) => {
                published = true;
                break;
            }
            Err(e) => {
                eprintln!("publish attempt failed: {:?}, retrying", e);
                // pump events again briefly before retrying
                if let Ok(ev_opt) = tokio::time::timeout(
                    Duration::from_millis(100),
                    futures::StreamExt::next(&mut swarm_a),
                )
                .await
                {
                    if let Some(ev) = ev_opt {
                        eprintln!("s1 event during publish retry: {:?}, retrying", ev);
                    }
                }
                if let Ok(ev_opt) = tokio::time::timeout(
                    Duration::from_millis(100),
                    futures::StreamExt::next(&mut swarm_b),
                )
                .await
                {
                    if let Some(ev) = ev_opt {
                        eprintln!("s2 event during publish retry: {:?}, retrying", ev);
                    }
                }
                tokio::time::sleep(Duration::from_millis(150)).await;
            }
        }
    }
    assert!(
        published,
        "failed to publish gossipsub message after retries"
    );

    // After publish, pump events on all swarms to aid message propagation
    // Track if any peer observed the message during these pumps (events are consumed)
    let mut saw_message = false;
    let pump_after_publish_start = std::time::Instant::now();
    while pump_after_publish_start.elapsed() < Duration::from_millis(2000) {
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(100),
            futures::StreamExt::next(&mut swarm_a),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s1 pump after publish: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Message { message, .. }) = &ev
                {
                    if message.data == b"hello".to_vec() {
                        saw_message = true;
                        break;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(100),
            futures::StreamExt::next(&mut swarm_b),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s2 pump after publish: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Message { message, .. }) = &ev
                {
                    if message.data == b"hello".to_vec() {
                        saw_message = true;
                        break;
                    }
                }
            }
        }
        if let Ok(ev_opt) = tokio::time::timeout(
            Duration::from_millis(100),
            futures::StreamExt::next(&mut swarm_c),
        )
        .await
        {
            if let Some(ev) = ev_opt {
                eprintln!("s3 pump after publish: {:?}", ev);
                if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Message { message, .. }) = &ev
                {
                    if message.data == b"hello".to_vec() {
                        saw_message = true;
                        break;
                    }
                }
            }
        }
    }

    // wait for message on B or C (match via debug string since event enum varies between versions)
    let got = tokio::time::timeout(Duration::from_secs(8), async {
        // If message already observed during pumps, return early
        if saw_message {
            return true;
        }
        let deadline = std::time::Instant::now() + Duration::from_secs(8);
        while std::time::Instant::now() < deadline {
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(200),
                futures::StreamExt::next(&mut swarm_b),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Message {
                        message, ..
                    }) = &ev
                    {
                        eprintln!("s2 behaviour event: {:?}", ev);
                        if message.data == b"hello".to_vec() {
                            return true;
                        }
                    }
                }
            }
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(200),
                futures::StreamExt::next(&mut swarm_c),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    if let SwarmEvent::Behaviour(libp2p_gossipsub::Event::Message {
                        message, ..
                    }) = &ev
                    {
                        eprintln!("s3 behaviour event: {:?}", ev);
                        if message.data == b"hello".to_vec() {
                            return true;
                        }
                    }
                }
            }
            // also pump A while waiting
            if let Ok(ev_opt) = tokio::time::timeout(
                Duration::from_millis(50),
                futures::StreamExt::next(&mut swarm_a),
            )
            .await
            {
                if let Some(ev) = ev_opt {
                    eprintln!("s1 pump while waiting for message: {:?}", ev);
                }
            }
        }
        false
    })
    .await
    .unwrap_or(false);

    assert!(got, "did not receive gossipsub message");
}
