use std::net::SocketAddrV4;
use std::fmt;

use crate::sys;

use heim_common::prelude::*;

/// TCP connection states
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum TcpState {
    /// Represents an open connection, data received can be
    /// delivered to the user.
    /// The normal state for the data transfer phase of the connection.
    Established,
    /// Represents waiting for a matching connection request
    /// after having sent a connection request.
    SynSent,
    /// Represents waiting for a confirming connection
    /// request acknowledgment after having both received
    /// and sent a connection request.
    SynRecv,
    /// Represents waiting for a connection termination request
    /// from the remote TCP, or an acknowledgment of the connection
    /// termination request previously sent.
    FinWait1,
    /// Represents waiting for a connection termination request
    /// from the remote TCP.
    FinWait2,
    /// Represents waiting for enough time to pass to be sure
    /// the remote TCP received the acknowledgment of its connection
    /// termination request.
    TimeWait,
    /// Represents no connection state at all.
    Close,
    /// Represents waiting for a connection termination request
    /// from the local user.
    CloseWait,
    /// Represents waiting for an acknowledgment of the
    /// connection termination request previously sent to the remote TCP
    /// (which includes an acknowledgment of its connection termination request).
    LastAck,
    /// Represents waiting for a connection request
    /// from any remote TCP and port.
    Listen,
    /// Represents waiting for a connection termination request
    /// acknowledgment from the remote TCP.
    Closing,
}

#[derive(heim_derive::ImplWrap)]
pub struct TcpConnection(sys::TcpConnection);

impl TcpConnection {
    pub fn state(&self) -> TcpState {
        self.as_ref().state()
    }

    pub fn local_address(&self) -> &SocketAddrV4 {
        self.as_ref().local_address()
    }

    pub fn remote_address(&self) -> &SocketAddrV4 {
        self.as_ref().remote_address()
    }
}

impl fmt::Debug for TcpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TcpConnection")
            .field("state", &self.state())
            .field("local_address", &self.local_address())
            .field("remote_address", &self.remote_address())
            .finish()
    }
}
