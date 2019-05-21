use std::net;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::parse::ParseIterator;

use super::inet::parse_addr;

#[derive(Debug)]
pub struct UdpConnection {
    laddr: net::SocketAddrV4,
    raddr: net::SocketAddrV4,
}

impl UdpConnection {
    pub fn source(&self) -> &net::SocketAddrV4 {
        &self.laddr
    }

    pub fn destination(&self) -> &net::SocketAddrV4 {
        &self.raddr
    }
}

impl FromStr for UdpConnection {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace().skip(1);
        let laddr = parse_addr::<net::SocketAddrV4>(parts.try_next()?)?;
        let raddr = parse_addr::<net::SocketAddrV4>(parts.try_next()?)?;

        Ok(Self{
            laddr,
            raddr,
        })
    }
}

pub fn udp_connections() -> impl Stream<Item = Result<UdpConnection>> {
    // TODO: Trailing `.skip(1)` will still try to parse file header
    utils::fs::read_lines_into("/proc/net/udp").into_stream().skip(1)
}
