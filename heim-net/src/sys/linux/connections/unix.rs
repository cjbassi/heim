use std::str::FromStr;

use heim_common::prelude::*;

#[derive(Debug)]
pub struct UnixConnection {
    path: Option<String>,
}

impl UnixConnection {
    pub fn bound_path(&self) -> Option<&str> {
        self.path.as_ref().map(|s| s.as_str())
    }
}

impl FromStr for UnixConnection {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let mut parts = line.split_whitespace().skip(7);
        let path = parts.next().map(|path| path.to_owned());

        Ok(Self{
            path,
        })
    }
}

pub fn unix_connections() -> impl Stream<Item = Result<UnixConnection>> {
    // TODO: Trailing `.skip(1)` will still try to parse file header
    utils::fs::read_lines_into("/proc/net/unix").into_stream().skip(1)
}
