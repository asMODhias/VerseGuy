use crate::dial_opts::{DialOpts, Opts, WithPeerId, WithPeerIdWithAddresses, WithoutPeerIdWithAddress};
use libp2p_core::transport::{DialOpts as CoreDialOpts, PortUse};

impl From<DialOpts> for CoreDialOpts {
    fn from(d: DialOpts) -> CoreDialOpts {
        match d.0 {
            Opts::WithPeerId(WithPeerId { role_override, .. }) => CoreDialOpts {
                role: role_override,
                port_use: PortUse::Reuse,
            },
            Opts::WithPeerIdWithAddresses(WithPeerIdWithAddresses { role_override, .. }) => CoreDialOpts {
                role: role_override,
                port_use: PortUse::Reuse,
            },
            Opts::WithoutPeerIdWithAddress(WithoutPeerIdWithAddress { role_override, .. }) => CoreDialOpts {
                role: role_override,
                port_use: PortUse::Reuse,
            },
        }
    }
}
