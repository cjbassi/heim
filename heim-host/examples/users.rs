#![feature(async_await)]

use heim_common::prelude::*;
use heim_host as host;

#[runtime::main]
async fn main() -> Result<()> {
    let mut users = host::users();
    while let Some(user) = users.next().await {
        let user = user?;

        println!("{:?}", user);
    }

    Ok(())
}
