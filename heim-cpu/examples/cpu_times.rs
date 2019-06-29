#![allow(stable_features)]
#![feature(async_await, futures_api)]

use heim_common::prelude::*;
use heim_cpu as cpu;

#[runtime::main]
async fn main() -> Result<()> {
    dbg!(cpu::time().await?);

    let mut times = cpu::times();
    while let Some(time) = times.next().await {
        dbg!(time?);
    }

    Ok(())
}
