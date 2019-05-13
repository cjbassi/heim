use std::pin::Pin;

use crate::{Address, AddressFamily};
use nix::ifaddrs;
use nix::net::if_::InterfaceFlags;
use nix::sys::socket;

use heim_common::prelude::*;

#[derive(Debug)]
pub struct Nic(ifaddrs::InterfaceAddress);

impl Nic {
    pub fn name(&self) -> &str {
        self.0.interface_name.as_str()
    }

    pub fn family(&self) -> AddressFamily {
        self.0
            .address
            .as_ref()
            .expect("NIC stream should exclude entries without address")
            .family()
            .into()
    }

    pub fn address(&self) -> Address {
        self.0
            .address
            .as_ref()
            .expect("NIC stream should exclude entries without address")
            .into()
    }

    pub fn netmask(&self) -> Option<Address> {
        self.0.netmask.as_ref().map(Into::into)
    }

    pub fn broadcast(&self) -> Option<Address> {
        self.0.broadcast.as_ref().map(Into::into)
    }

    pub fn destination(&self) -> Option<Address> {
        self.0.destination.as_ref().map(Into::into)
    }

    pub fn is_up(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_UP)
    }

    pub fn is_broadcast(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_BROADCAST)
    }

    pub fn is_loopback(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_LOOPBACK)
    }

    pub fn is_point_to_point(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_POINTOPOINT)
    }

    pub fn is_multicast(&self) -> bool {
        self.0.flags.contains(InterfaceFlags::IFF_MULTICAST)
    }
}

pub fn nic() -> impl Stream<Item = Result<Nic>> {
    future::lazy(|_| {
        // `nix::ifaddrs` structs are not safe to send between threads,
        // so collecting them in a once
        let iter = ifaddrs::getifaddrs()?;
        let interfaces = iter.collect::<Vec<_>>();

        Ok(interfaces)
    })
    .map_ok(|interfaces| {
        let stream = stream::iter(interfaces).map(Ok);

        // TODO: https://github.com/rust-lang-nursery/futures-rs/issues/1444
        Box::pin(stream) as Pin<Box<dyn Stream<Item = _> + Send>>
    })
    .unwrap_or_else(|e| {
        Box::pin(stream::once(future::err(e)))
    })
    .flatten_stream()
    .try_filter_map(|addr: ifaddrs::InterfaceAddress| {
        // Skipping unsupported address families
        let result = if addr.address.is_some() {
            Some(Nic(addr))
        } else {
            None
        };

        future::ok(result)
    })
}

impl From<socket::AddressFamily> for AddressFamily {
    fn from(f: socket::AddressFamily) -> Self {
        use nix::sys::socket::AddressFamily::*;
        match f {
            Unix => AddressFamily::Unix,
            Inet => AddressFamily::Inet,
            Inet6 => AddressFamily::Inet6,
            Packet => AddressFamily::Packet,
            other => unimplemented!("Unknown address family: {:?}", other),
        }
    }
}

impl From<&socket::SockAddr> for Address {
    fn from(s: &socket::SockAddr) -> Self {
        use nix::sys::socket::SockAddr::*;

        match *s {
            Inet(addr) => Address::Inet(addr.to_std()),
            // TODO: Convert contents into Link addr
            Link(..) => Address::Link,
            other => unimplemented!("Unknown sockaddr: {:?}", other),
        }
    }
}
