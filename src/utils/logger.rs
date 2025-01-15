use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init(debug: bool) {
    DEBUG_ENABLED.store(debug, Ordering::SeqCst);
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::utils::logger::is_debug_enabled() {
            println!($($arg)*);
        }
    };
}

pub fn is_debug_enabled() -> bool {
    DEBUG_ENABLED.load(Ordering::SeqCst)
}
