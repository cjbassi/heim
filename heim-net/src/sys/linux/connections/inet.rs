// Common code for AF_INET connections

use std::net;

use heim_common::prelude::*;
use heim_common::utils::parse::*;

pub trait ParseAddr: Sized {
    fn try_parse(raw_ip: &str, port: u16) -> Result<Self>;
}

impl ParseAddr for net::SocketAddrV4 {
    fn try_parse(raw_ip: &str, port: u16) -> Result<net::SocketAddrV4> {
        let mut bytes: [u8; 4] = hex::FromHex::from_hex(raw_ip)
            .map_err(|_| Error::new(ErrorKind::Parse))?;
        #[cfg(target_endian = "little")]
        bytes.reverse();

        let ip = net::Ipv4Addr::from(bytes);

        Ok(net::SocketAddrV4::new(ip, port))
    }
}

impl ParseAddr for net::SocketAddrV6 {
    fn try_parse(raw_ip: &str, port: u16) -> Result<net::SocketAddrV6> {
        let mut bytes: [u8; 16] = hex::FromHex::from_hex(raw_ip)
            .map_err(|_| Error::new(ErrorKind::Parse))?;
        #[cfg(target_endian = "little")]
        bytes.reverse();

        let ip = net::Ipv6Addr::from(bytes);

        Ok(net::SocketAddrV6::new(ip, port, 0, 0))
    }
}

pub fn parse_addr<A: ParseAddr>(value: &str) -> Result<A> {
    let mut parts = value.splitn(2, ':');
    let raw_ip = parts.try_next()?;
    let port = u16::from_str_radix(parts.try_next()?, 16)?;

    A::try_parse(raw_ip, port)
}
