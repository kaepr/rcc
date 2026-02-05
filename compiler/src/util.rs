use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn make_temporary() -> String {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{counter}")
}
