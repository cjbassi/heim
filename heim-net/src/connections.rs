use heim_common::prelude::*;

mod tcp;

pub use self::tcp::*;
use crate::sys;

bitflags::bitflags! {
    pub struct ConnectionKind: u32 {
        const TCPV4 = 0b0000_0001;
        const TCPV6 = 0b0000_0010;
    }
}

#[derive(Debug)]
pub enum Connection {
    Tcp(TcpConnection),
}

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Result<Connection>> {
    sys::connections(kind)
}
