use std::net::SocketAddrV6;
use std::fmt;

use crate::{sys, TcpState};

#[derive(heim_derive::ImplWrap)]
pub struct Tcp6Connection(sys::Tcp6Connection);

impl Tcp6Connection {
    pub fn state(&self) -> TcpState {
        self.as_ref().state()
    }

    pub fn source(&self) -> &SocketAddrV6 {
        self.as_ref().source()
    }

    pub fn destination(&self) -> &SocketAddrV6 {
        self.as_ref().destination()
    }
}

impl fmt::Debug for Tcp6Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tcp6Connection")
            .field("state", &self.state())
            .field("source", &self.source())
            .field("destination", &self.destination())
            .finish()
    }
}
