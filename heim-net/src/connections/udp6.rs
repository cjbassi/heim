use std::net::SocketAddrV6;
use std::fmt;

use crate::sys;

#[derive(heim_derive::ImplWrap)]
pub struct Udp6Connection(sys::Udp6Connection);

impl Udp6Connection {
    pub fn local_address(&self) -> &SocketAddrV6 {
        self.as_ref().local_address()
    }

    pub fn remote_address(&self) -> &SocketAddrV6 {
        self.as_ref().remote_address()
    }
}

impl fmt::Debug for Udp6Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Udp6Connection")
            .field("local_address", &self.local_address())
            .field("remote_address", &self.remote_address())
            .finish()
    }
}
