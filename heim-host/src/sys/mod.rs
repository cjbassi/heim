cfg_if::cfg_if! {
    if #[cfg(unix)] {
        // Since there is a lot of shared into between all UNIX-like systems,
        // aggregating them into a separate module.
        mod unix;

        pub use self::unix::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    }
}