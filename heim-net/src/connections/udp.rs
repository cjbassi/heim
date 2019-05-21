use std::net::SocketAddrV4;
use std::fmt;

use crate::sys;

#[derive(heim_derive::ImplWrap)]
pub struct UdpConnection(sys::UdpConnection);

impl UdpConnection {
    pub fn local_address(&self) -> &SocketAddrV4 {
        self.as_ref().local_address()
    }

    pub fn remote_address(&self) -> &SocketAddrV4 {
        self.as_ref().remote_address()
    }
}

impl fmt::Debug for UdpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UdpConnection")
            .field("local_address", &self.local_address())
            .field("remote_address", &self.remote_address())
            .finish()
    }
}
