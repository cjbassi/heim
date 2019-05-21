use std::net;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::parse::ParseIterator;

use super::inet::parse_addr;

#[derive(Debug)]
pub struct Udp6Connection {
    laddr: net::SocketAddrV6,
    raddr: net::SocketAddrV6,
}

impl Udp6Connection {
    pub fn local_address(&self) -> &net::SocketAddrV6 {
        &self.laddr
    }

    pub fn remote_address(&self) -> &net::SocketAddrV6 {
        &self.raddr
    }
}

impl FromStr for Udp6Connection {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace().skip(1);
        let laddr = parse_addr::<net::SocketAddrV6>(parts.try_next()?)?;
        let raddr = parse_addr::<net::SocketAddrV6>(parts.try_next()?)?;

        Ok(Self{
            laddr,
            raddr,
        })
    }
}

pub fn udp6_connections() -> impl Stream<Item = Result<Udp6Connection>> {
    // TODO: Trailing `.skip(1)` will still try to parse file header
    // TODO: Return empty stream if `/proc/net/udp6` file is missing
    utils::fs::read_lines_into("/proc/net/udp6").into_stream().skip(1)
}
