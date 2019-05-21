use heim_common::prelude::*;

mod tcp;
mod tcp6;
mod udp;
mod udp6;

pub use self::tcp::*;
pub use self::tcp6::*;
pub use self::udp::*;
pub use self::udp6::*;

use crate::sys;

bitflags::bitflags! {
    pub struct ConnectionKind: u32 {
        const TCPV4 = 0b0000_0001;
        const TCPV6 = 0b0000_0010;
        const UDPV4 = 0b0000_0100;
        const UDPV6 = 0b0000_1000;
    }
}

#[derive(Debug)]
pub enum Connection {
    Tcp(TcpConnection),
    Tcp6(Tcp6Connection),
    Udp(UdpConnection),
    Udp6(Udp6Connection),
}

pub fn connections(kind: ConnectionKind) -> impl Stream<Item = Result<Connection>> {
    sys::connections(kind)
}
