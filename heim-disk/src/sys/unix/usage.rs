use std::io;
use std::mem;
use std::ffi::CString;
use std::path::Path;

use heim_common::prelude::*;
use heim_common::units::si::f64::Ratio;
use heim_common::units::si::ratio::ratio;
use heim_common::units::iec::u64::Information;
use heim_common::units::iec::information::byte;

use crate::os::unix::Flags;

pub struct Usage(libc::statvfs);

impl Usage {
    pub fn total(&self) -> Information {
        let value = self.0.f_blocks * self.0.f_frsize;

        Information::new::<byte>(value)
    }

    pub fn used(&self) -> Information {
        let avail_to_root = self.0.f_bfree * self.0.f_frsize;

        self.total() - Information::new::<byte>(avail_to_root)
    }

    pub fn free(&self) -> Information {
        let value = self.0.f_bavail * self.0.f_frsize;

        Information::new::<byte>(value)
    }

    pub fn ratio(&self) -> Ratio {
        // FIXME: Possible value truncation while casting into f64.
        // Lucky us, it is a 2019 and we are good for the next couple of decades
        let used = self.used().value as f64;
        let avail_to_user = self.0.f_bavail * self.0.f_frsize;
        let total_user = used + avail_to_user as f64;

        Ratio::new::<ratio>(used / total_user)
    }

    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.0.f_flag)
    }
}

pub fn usage<T: AsRef<Path>>(path: T) -> impl Future<Item=Usage, Error=Error> {
    future::lazy(move || {
        unsafe {
            let path = path.as_ref().to_str()
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
                .and_then(|string| {
                    CString::new(string).map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))
                })?;

            let mut vfs: libc::statvfs = mem::uninitialized();
            let result = libc::statvfs(path.into_raw(), &mut vfs);

            if result == 0 {
                Ok(Usage(vfs))
            } else {
                Err(io::Error::last_os_error().into())
            }
        }
    })
}

//    unsigned long  f_bsize;    /* file system block size */
//    unsigned long  f_frsize;   /* fragment size */
//    fsblkcnt_t     f_blocks;   /* size of fs in f_frsize units */
//    fsblkcnt_t     f_bfree;    /* # free blocks */
//    fsblkcnt_t     f_bavail;   /* # free blocks for unprivileged users */
//    fsfilcnt_t     f_files;    /* # inodes */
//    fsfilcnt_t     f_ffree;    /* # free inodes */
//    fsfilcnt_t     f_favail;   /* # free inodes for unprivileged users */
//    unsigned long  f_fsid;     /* file system ID */
//    unsigned long  f_flag;     /* mount flags */
//    unsigned long  f_namemax;  /* maximum filename length */