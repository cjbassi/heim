use std::net::SocketAddrV6;
use std::fmt;

use crate::sys;

#[derive(heim_derive::ImplWrap)]
pub struct Udp6Connection(sys::Udp6Connection);

impl Udp6Connection {
    pub fn source(&self) -> &SocketAddrV6 {
        self.as_ref().source()
    }

    pub fn destination(&self) -> &SocketAddrV6 {
        self.as_ref().destination()
    }
}

impl fmt::Debug for Udp6Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Udp6Connection")
            .field("source", &self.source())
            .field("destination", &self.destination())
            .finish()
    }
}
