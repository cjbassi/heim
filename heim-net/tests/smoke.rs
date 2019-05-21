#![feature(async_await)]

use heim_common::prelude::*;
use heim_net as net;


#[runtime::test]
async fn smoke_io_counters() {
    let mut counters = net::io_counters();
    while let Some(counter) = counters.next().await {
        let counter = counter.unwrap();

    }
}

#[runtime::test]
async fn smoke_nic() {
    let mut nic = net::nic();
    while let Some(iface) = nic.next().await {
        let iface = iface.unwrap();

    }
}

