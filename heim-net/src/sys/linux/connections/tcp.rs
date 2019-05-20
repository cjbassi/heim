use std::net;
use std::str::FromStr;

use heim_common::prelude::*;
use heim_common::utils::parse::ParseIterator;

use crate::connections::TcpState;

#[derive(Debug)]
pub struct TcpConnection {
    state: TcpState,
    laddr: net::SocketAddrV4,
    raddr: net::SocketAddrV4,
}

impl TcpConnection {
    pub fn state(&self) -> TcpState {
        self.state
    }

    pub fn local_address(&self) -> &net::SocketAddrV4 {
        &self.laddr
    }

    pub fn remote_address(&self) -> &net::SocketAddrV4 {
        &self.raddr
    }
}

impl FromStr for TcpConnection {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace().skip(1);
        let laddr = parse_addr(parts.try_next()?, libc::AF_INET)?;
        let raddr = parse_addr(parts.try_next()?, libc::AF_INET)?;
        let state: TcpState = parts.try_from_next()?;

        Ok(Self{
            state,
            laddr,
            raddr,
        })
    }
}

fn parse_addr(value: &str, family: libc::c_int) -> Result<net::SocketAddrV4> {
    let mut parts = value.splitn(2, ':');
    let raw_ip = parts.try_next()?;
    let port = u16::from_str_radix(parts.try_next()?, 16)?;

    match family {
        libc::AF_INET => {
            let bytes: [u8; 4] = hex::FromHex::from_hex(raw_ip)
                .map_err(|_| Error::new(ErrorKind::Parse))?;
            let ip = net::Ipv4Addr::new(bytes[3], bytes[2], bytes[1], bytes[0]);

            Ok(net::SocketAddrV4::new(ip, port))
        },
        _ => unreachable!(),
    }
}

impl FromStr for TcpState {
    type Err = Error;

    fn from_str(state: &str) -> Result<Self> {
        let state = match i32::from_str_radix(state, 16)? {
            1 => TcpState::Established,
            2 => TcpState::SynSent,
            3 => TcpState::SynRecv,
            4 => TcpState::FinWait1,
            5 => TcpState::FinWait2,
            6 => TcpState::TimeWait,
            7 => TcpState::Close,
            8 => TcpState::CloseWait,
            9 => TcpState::LastAck,
            10 => TcpState::Listen,
            11 => TcpState::Closing,
            // Linux have this additional "NEW-SYN-RECV" state
            // but it is linux-specific
            // https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=10feb428a504
            12 => TcpState::SynRecv,
            other => unreachable!("There is no other TCP states, but we got the {}", other),
        };

        Ok(state)
    }
}

pub fn tcp_connections() -> impl Stream<Item = Result<TcpConnection>> {
    // TODO: Trailing `.skip(1)` will still try to parse file header
    utils::fs::read_lines_into("/proc/net/tcp").into_stream().skip(1)
}
