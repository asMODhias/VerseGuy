//! Local compatibility shims to bridge API differences between vendored
//! `libp2p-swarm` sources and workspace `libp2p-core` version.

// Compatibility helpers for ProtocolName / NameAsBytes conversions.

/// Simple connection identifier used internally by this vendored crate.
/// Previously `libp2p_core` exported `ConnectionId`; provide a local
/// alias compatible with how swarm uses it (incrementable, hashable).
pub type ConnectionId = usize;

/// Pending point placeholder: map to `ConnectedPoint` from `libp2p_core`.
pub type PendingPoint = libp2p_core::ConnectedPoint;

/// Small compatibility aliases for `Either`-style outputs and errors used by
/// the original `libp2p-swarm` sources. Use `either::Either` to align with
/// other crates that expect the same enum.
#[allow(dead_code)]
pub type EitherOutput<A, B> = libp2p_core::either::EitherOutput<A, B>;

#[allow(dead_code)]
pub type EitherError<A, B> = libp2p_core::either::EitherError<A, B>;




// Helper trait used to obtain a byte slice from protocol name items. This
// allows `name_wrap` to generically support both `AsRef<[u8]>` and
// `AsRef<str>`-backed `Info` types.
pub trait NameAsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl NameAsBytes for String {
    fn as_bytes(&self) -> &[u8] {
        <String as AsRef<str>>::as_ref(self).as_bytes()
    }
}

impl<'a> NameAsBytes for &'a str {
    fn as_bytes(&self) -> &[u8] {
        <&str as AsRef<str>>::as_ref(self).as_bytes()
    }
}

impl NameAsBytes for Vec<u8> {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<'a> NameAsBytes for &'a [u8] {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<T: NameAsBytes, U: NameAsBytes> NameAsBytes for either::Either<T, U> {
    fn as_bytes(&self) -> &[u8] {
        match self {
            either::Either::Left(a) => a.as_bytes(),
            either::Either::Right(b) => b.as_bytes(),
        }
    }
}



#[allow(dead_code)]
pub trait ProtocolName: Clone + Send + 'static {
    fn protocol_name(&self) -> &[u8];
}

// Allow `&'static str` protocol info to satisfy the old `ProtocolName` trait.
impl ProtocolName for &'static str {
    fn protocol_name(&self) -> &[u8] {
        <&str as AsRef<str>>::as_ref(self).as_bytes()
    }
}

// Allow `String` protocol info to satisfy the old `ProtocolName` trait.
impl ProtocolName for String {
    fn protocol_name(&self) -> &[u8] {
        <String as AsRef<str>>::as_ref(self).as_bytes()
    }
}

// Allow byte slices as protocol names as well.
impl ProtocolName for Vec<u8> {
    fn protocol_name(&self) -> &[u8] {
        self.as_ref()
    }
}

impl ProtocolName for &'static [u8] {
    fn protocol_name(&self) -> &[u8] {
        self.as_ref()
    }
}

// Compose `ProtocolName` for `either::Either` when both sides implement it.
impl<T: ProtocolName, U: ProtocolName> ProtocolName for either::Either<T, U> {
    fn protocol_name(&self) -> &[u8] {
        match self {
            either::Either::Left(a) => a.protocol_name(),
            either::Either::Right(b) => b.protocol_name(),
        }
    }
}

// Support `EitherName` from `libp2p-core::upgrade` when used with `SelectUpgrade` / `EitherUpgrade`.
impl<T: NameAsBytes, U: NameAsBytes> NameAsBytes for libp2p_core::either::EitherName<T, U> {
    fn as_bytes(&self) -> &[u8] {
        match self {
            libp2p_core::either::EitherName::A(a) => a.as_bytes(),
            libp2p_core::either::EitherName::B(b) => b.as_bytes(),
        }
    }
}

/// Local UpgradeError mirror so vendored code can pattern-match on it.
#[derive(Debug)]
pub enum UpgradeError<E> {
    Select(multistream_select::NegotiationError),
    Apply(E),
}

impl<E> From<multistream_select::NegotiationError> for UpgradeError<E> {
    fn from(e: multistream_select::NegotiationError) -> Self {
        UpgradeError::Select(e)
    }
}

impl<E> std::fmt::Display for UpgradeError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeError::Select(_) => write!(f, "Multistream select failed"),
            UpgradeError::Apply(_) => write!(f, "Handshake failed"),
        }
    }
}

impl<E> std::error::Error for UpgradeError<E> where E: std::error::Error + 'static {}

// Convert between libp2p-core's UpgradeError and the local UpgradeError wrapper so
// pattern matches and forwarding are straightforward across the compatibility
// boundary.
impl<E> From<libp2p_core::upgrade::UpgradeError<E>> for UpgradeError<E> {
    fn from(e: libp2p_core::upgrade::UpgradeError<E>) -> Self {
        match e {
            libp2p_core::upgrade::UpgradeError::Select(err) => UpgradeError::Select(err),
            libp2p_core::upgrade::UpgradeError::Apply(e) => UpgradeError::Apply(e),
        }
    }
}

impl<E> From<UpgradeError<E>> for libp2p_core::upgrade::UpgradeError<E> {
    fn from(e: UpgradeError<E>) -> libp2p_core::upgrade::UpgradeError<E> {
        match e {
            UpgradeError::Select(err) => libp2p_core::upgrade::UpgradeError::Select(err),
            UpgradeError::Apply(e) => libp2p_core::upgrade::UpgradeError::Apply(e),
        }
    }
}

// Convert between libp2p-core's EitherError and this crate's alias (either::Either)


// --- Backport full apply_inbound / apply_outbound implementations ---
// Copied/adapted from `libp2p-core::upgrade::apply` to avoid depending on
// internal/private helpers that moved in newer `libp2p-core`.
use libp2p_core::{InboundUpgrade, OutboundUpgrade, Negotiated};
use futures::{future::Either, prelude::*};
use log::debug;
use multistream_select::{self, DialerSelectFuture, ListenerSelectFuture};
use std::{iter, mem, pin::Pin, task::Context, task::Poll};

pub use multistream_select::Version;

pub type NameWrapIter<I> = iter::Map<I, fn(<I as Iterator>::Item) -> NameWrap<<I as Iterator>::Item>>;

/// Wrapper type to expose an owned `String` representation for protocol names
/// while preserving the original `Info` value for upgrade callbacks.
#[derive(Clone)]
pub struct NameWrap<N>(N, std::string::String);

impl<N> NameWrap<N> {
    fn new(n: N) -> Self
    where
        N: NameAsBytes,
    {
        // Try to decode as UTF-8, fall back to lossy conversion.
        let s = match std::str::from_utf8(n.as_bytes()) {
            Ok(s) => s.to_owned(),
            Err(_) => String::from_utf8_lossy(n.as_bytes()).into_owned(),
        };
        NameWrap(n, s)
    }

    /// Extract the inner `N` value. This is `pub(crate)` so other modules in
    /// this vendored crate can consume the original `Info` type when needed.
    pub(crate) fn into_inner(self) -> N {
        self.0
    }
}

// Helper function used as a function pointer in iterator mapping sites.
pub(crate) fn name_wrap<N: NameAsBytes>(n: N) -> NameWrap<N> {
    NameWrap::new(n)
}

impl<N: NameAsBytes> AsRef<[u8]> for NameWrap<N> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<N> AsRef<str> for NameWrap<N> {
    fn as_ref(&self) -> &str {
        &self.1
    }
}

// Expose NameWrap as `NameAsBytes` so it can interoperate where protocol
// names are expected to provide bytes. This avoids having to propagate
// `AsRef<str>` bounds for wrapped `Info` types across the codebase.
impl<N: NameAsBytes> NameAsBytes for NameWrap<N> {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

// Tries to apply either the inbound or outbound upgrade depending on the
// connected point (dialer vs listener).
#[allow(dead_code)]
pub fn apply<C, U>(
    conn: C,
    up: U,
    cp: libp2p_core::connection::ConnectedPoint,
    v: Version,
) -> Either<InboundUpgradeApply<C, U>, OutboundUpgradeApply<C, U>>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>> + OutboundUpgrade<Negotiated<C>>,
    <U as libp2p_core::upgrade::UpgradeInfo>::Info: NameAsBytes,
{
    match cp {
        libp2p_core::connection::ConnectedPoint::Dialer { role_override, .. }
            if role_override.is_dialer() => Either::Right(apply_outbound(conn, up, v)),
        _ => Either::Left(apply_inbound(conn, up)),
    }
}

/// Tries to perform an upgrade on an inbound connection or substream.
pub fn apply_inbound<C, U>(conn: C, up: U) -> InboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>>,
    U::Info: NameAsBytes,
{
    let iter = up.protocol_info().into_iter().map(name_wrap as fn(_) -> NameWrap<_>);
    let future = multistream_select::listener_select_proto(conn, iter);
    InboundUpgradeApply {
        inner: InboundUpgradeApplyState::Init { future, upgrade: up },
    }
}

/// Tries to perform an upgrade on an outbound connection or substream.
pub fn apply_outbound<C, U>(conn: C, up: U, v: Version) -> OutboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: OutboundUpgrade<Negotiated<C>>,
    U::Info: NameAsBytes,
{
    let iter = up.protocol_info().into_iter().map(name_wrap as fn(_) -> NameWrap<_>);
    let future = multistream_select::dialer_select_proto(conn, iter, v);
    OutboundUpgradeApply {
        inner: OutboundUpgradeApplyState::Init { future, upgrade: up },
    }
}

/// Future returned by `apply_inbound`. Drives the upgrade process.
pub struct InboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>>,
{
    inner: InboundUpgradeApplyState<C, U>,
}

enum InboundUpgradeApplyState<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>>,
{
    Init {
        future: ListenerSelectFuture<C, NameWrap<U::Info>>,
        upgrade: U,
    },
    Upgrade { future: Pin<Box<U::Future>> },
    Undefined,
}

impl<C, U> Unpin for InboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>>,
{
}

impl<C, U> Future for InboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: InboundUpgrade<Negotiated<C>>,
    U::Info: NameAsBytes,
{
    type Output = Result<U::Output, UpgradeError<U::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match mem::replace(&mut self.inner, InboundUpgradeApplyState::Undefined) {
                InboundUpgradeApplyState::Init { mut future, upgrade } => {
                    let (info, io) = match Future::poll(Pin::new(&mut future), cx)? {
                        Poll::Ready(x) => x,
                        Poll::Pending => {
                            self.inner = InboundUpgradeApplyState::Init { future, upgrade };
                            return Poll::Pending;
                        }
                    };
                    self.inner = InboundUpgradeApplyState::Upgrade {
                        future: Box::pin(upgrade.upgrade_inbound(io, info.0)),
                    };
                }
                InboundUpgradeApplyState::Upgrade { mut future } => {
                    match Future::poll(Pin::new(&mut future), cx) {
                        Poll::Pending => {
                            self.inner = InboundUpgradeApplyState::Upgrade { future };
                            return Poll::Pending;
                        }
                        Poll::Ready(Ok(x)) => {
                            debug!("Successfully applied negotiated protocol");
                            return Poll::Ready(Ok(x));
                        }
                        Poll::Ready(Err(e)) => {
                            debug!("Failed to apply negotiated protocol");
                            return Poll::Ready(Err(UpgradeError::Apply(e)));
                        }
                    }
                }
                InboundUpgradeApplyState::Undefined => {
                    panic!("InboundUpgradeApplyState::poll called after completion")
                }
            }
        }
    }
}

/// Future returned by `apply_outbound`. Drives the upgrade process.
pub struct OutboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: OutboundUpgrade<Negotiated<C>>,
{
    inner: OutboundUpgradeApplyState<C, U>,
}

enum OutboundUpgradeApplyState<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: OutboundUpgrade<Negotiated<C>>,
{
    Init {
        future: DialerSelectFuture<C, NameWrapIter<<U::InfoIter as IntoIterator>::IntoIter>>,
        upgrade: U,
    },
    Upgrade { future: Pin<Box<U::Future>> },
    Undefined,
}

impl<C, U> Unpin for OutboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: OutboundUpgrade<Negotiated<C>>,
{
}

impl<C, U> Future for OutboundUpgradeApply<C, U>
where
    C: futures::io::AsyncRead + futures::io::AsyncWrite + Unpin,
    U: OutboundUpgrade<Negotiated<C>>,
    U::Info: NameAsBytes,
{
    type Output = Result<U::Output, UpgradeError<U::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match mem::replace(&mut self.inner, OutboundUpgradeApplyState::Undefined) {
                OutboundUpgradeApplyState::Init { mut future, upgrade } => {
                    let (info, connection) = match Future::poll(Pin::new(&mut future), cx)? {
                        Poll::Ready(x) => x,
                        Poll::Pending => {
                            self.inner = OutboundUpgradeApplyState::Init { future, upgrade };
                            return Poll::Pending;
                        }
                    };
                    self.inner = OutboundUpgradeApplyState::Upgrade {
                        future: Box::pin(upgrade.upgrade_outbound(connection, info.0)),
                    };
                }
                OutboundUpgradeApplyState::Upgrade { mut future } => {
                    match Future::poll(Pin::new(&mut future), cx) {
                        Poll::Pending => {
                            self.inner = OutboundUpgradeApplyState::Upgrade { future };
                            return Poll::Pending;
                        }
                        Poll::Ready(Ok(x)) => {
                            debug!("Successfully applied negotiated protocol");
                            return Poll::Ready(Ok(x));
                        }
                        Poll::Ready(Err(e)) => {
                            debug!("Failed to apply negotiated protocol");
                            return Poll::Ready(Err(UpgradeError::Apply(e)));
                        }
                    }
                }
                OutboundUpgradeApplyState::Undefined => {
                    panic!("OutboundUpgradeApplyState::poll called after completion")
                }
            }
        }
    }
}


