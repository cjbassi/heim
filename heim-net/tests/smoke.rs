use heim_net as net;


#[runtime::test]
fn smoke_io_counters() {
    let mut counters = net::io_counters();
    while let Some(counter) = counters.next().await {
        let counter = counter.unwrap();

    }
}

#[runtime::test]
fn smoke_nic() {
    let mut nic = net::nic();
    while let Some(iface) = nic.next().await {
        let iface = iface.unwrap();

    }
}

