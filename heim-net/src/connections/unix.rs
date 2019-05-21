use std::fmt;

use crate::sys;

#[cfg(unix)]
#[derive(heim_derive::ImplWrap)]
pub struct UnixConnection(sys::UnixConnection);

impl UnixConnection {
    pub fn bound_path(&self) -> Option<&str> {
        self.as_ref().bound_path()
    }
}

impl fmt::Debug for UnixConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnixConnection")
            .field("bound_path", &self.bound_path())
            .finish()
    }
}
