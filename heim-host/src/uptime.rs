use heim_common::prelude::*;

use crate::sys;
use crate::units::Time;

/// Returns future which resolves into [Time] amount from the system boot.
pub fn uptime() -> impl Future<Item = Time, Error = Error> {
    sys::uptime()
}