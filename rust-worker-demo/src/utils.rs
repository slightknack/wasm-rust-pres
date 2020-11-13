use cfg_if::cfg_if;

// This is a panic hook, it's supposed to make errors propogate more nicely
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

// Because we want to keep the WASM bundle light,
// we use wee_alloc instead of the default rust allocator
// This decreases the size of the langauge runtime
// (Yes, even Rust has a runtime! - it's just pretty small)
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
