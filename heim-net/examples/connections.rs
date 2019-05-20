#![feature(async_await)]

use heim_common::prelude::*;
use heim_net as net;

#[runtime::main]
async fn main() -> Result<()> {
    let mut conns = net::connections(net::ConnectionKind::TCPV4);
    while let Some(conn) = conns.next().await {
        dbg!(conn?);
    }

    Ok(())
}
