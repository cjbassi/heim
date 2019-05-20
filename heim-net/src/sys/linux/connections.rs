use heim_common::prelude::*;

use crate::{ConnectionKind, Connection};
pub use self::tcp::TcpConnection;

mod tcp;

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Result<Connection>> {
    let mut streams = Vec::new();

    if kind.contains(ConnectionKind::TCPV4) {
        let stream = self::tcp::tcp_connections()
            .map_ok(|conn| Connection::Tcp(conn.into()));
        streams.push(stream);
    }

    stream::select_all(streams)
}
