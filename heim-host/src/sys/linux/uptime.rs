use heim_common::prelude::*;
use heim_common::units::Time;

#[allow(clippy::redundant_closure)]
pub fn uptime() -> impl Future<Output=Result<Time>> {
    utils::fs::read_to_string("/proc/uptime")
        .and_then(|contents| {
            let result = match contents.splitn(2, ' ').next() {
                Some(raw_value) => raw_value.parse::<f64>().map_err(Into::into),
                None => Err(Error::missing_entity("/proc/uptime"))
            };

            future::ready(result)
        })
        .map_ok(|seconds: f64| Time::new(seconds))
}
