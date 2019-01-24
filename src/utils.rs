use std::sync::atomic::{AtomicU64, Ordering};

/// The thread safe generator if Inos
/// Used when spawning a new handler
#[derive(Debug)]
pub(crate) struct InoGenerator {
    next_ino : AtomicU64
}

impl InoGenerator {

    pub(crate) fn new() -> Self {
        InoGenerator{
            next_ino : AtomicU64::new(1)
        }
    }

    pub(crate) fn generate(&self) -> u64 {
        self.next_ino.fetch_add(1, Ordering::SeqCst)
    }

}