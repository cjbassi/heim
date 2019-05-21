use heim_common::prelude::*;

mod tcp;
mod tcp6;

pub use self::tcp::*;
pub use self::tcp6::*;

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
    Tcp6(Tcp6Connection),
}

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Result<Connection>> {
    sys::connections(kind)
}
