use std::net::SocketAddrV4;
use std::fmt;

use crate::sys;

#[derive(heim_derive::ImplWrap)]
pub struct UdpConnection(sys::UdpConnection);

impl UdpConnection {
    pub fn source(&self) -> &SocketAddrV4 {
        self.as_ref().source()
    }

    pub fn destination(&self) -> &SocketAddrV4 {
        self.as_ref().destination()
    }
}

impl fmt::Debug for UdpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UdpConnection")
            .field("source", &self.source())
            .field("destination", &self.destination())
            .finish()
    }
}
