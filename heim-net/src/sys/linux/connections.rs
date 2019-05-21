use heim_common::prelude::*;

use crate::{ConnectionKind, Connection};

mod inet;
mod tcp;
mod tcp6;
mod udp;
mod udp6;

pub use self::tcp::TcpConnection;
pub use self::tcp6::Tcp6Connection;
pub use self::udp::UdpConnection;
pub use self::udp6::Udp6Connection;

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Result<Connection>> {
    let mut streams: Vec<stream::BoxStream<'static, Result<Connection>>> = Vec::new();

    if kind.contains(ConnectionKind::TCPV4) {
        let stream = self::tcp::tcp_connections()
            .map_ok(|conn| Connection::Tcp(conn.into()));
        streams.push(Box::pin(stream));
    }

    if kind.contains(ConnectionKind::TCPV6) {
        let stream = self::tcp6::tcp6_connections()
            .map_ok(|conn| Connection::Tcp6(conn.into()));
        streams.push(Box::pin(stream));
    }

    if kind.contains(ConnectionKind::UDPV4) {
        let stream = self::udp::udp_connections()
            .map_ok(|conn| Connection::Udp(conn.into()));
        streams.push(Box::pin(stream));
    }

    if kind.contains(ConnectionKind::UDPV6) {
        let stream = self::udp6::udp6_connections()
            .map_ok(|conn| Connection::Udp6(conn.into()));
        streams.push(Box::pin(stream));
    }

    stream::select_all(streams)
}
