use std::net;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::parse::ParseIterator;

use crate::connections::TcpState;
use super::inet::parse_addr;

#[derive(Debug)]
pub struct Tcp6Connection {
    state: TcpState,
    laddr: net::SocketAddrV6,
    raddr: net::SocketAddrV6,
}

impl Tcp6Connection {
    pub fn state(&self) -> TcpState {
        self.state
    }

    pub fn source(&self) -> &net::SocketAddrV6 {
        &self.laddr
    }

    pub fn destination(&self) -> &net::SocketAddrV6 {
        &self.raddr
    }
}

impl FromStr for Tcp6Connection {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace().skip(1);
        let laddr = parse_addr(parts.try_next()?)?;
        let raddr = parse_addr(parts.try_next()?)?;
        let state: TcpState = parts.try_from_next()?;

        Ok(Self{
            state,
            laddr,
            raddr,
        })
    }
}

pub fn tcp6_connections() -> impl Stream<Item = Result<Tcp6Connection>> {
    // TODO: Trailing `.skip(1)` will still try to parse file header
    // TODO: Return empty stream if `/proc/net/tcp6` file is missing
    utils::fs::read_lines_into("/proc/net/tcp6").into_stream().skip(1)
}
