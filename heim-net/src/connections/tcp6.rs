use std::net::SocketAddrV6;
use std::fmt;

use crate::{sys, TcpState};

#[derive(heim_derive::ImplWrap)]
pub struct Tcp6Connection(sys::Tcp6Connection);

impl Tcp6Connection {
    pub fn state(&self) -> TcpState {
        self.as_ref().state()
    }

    pub fn local_address(&self) -> &SocketAddrV6 {
        self.as_ref().local_address()
    }

    pub fn remote_address(&self) -> &SocketAddrV6 {
        self.as_ref().remote_address()
    }
}

impl fmt::Debug for Tcp6Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tcp6Connection")
            .field("state", &self.state())
            .field("local_address", &self.local_address())
            .field("remote_address", &self.remote_address())
            .finish()
    }
}
