use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

pub const SELF_TRIGGER_WINDOW: Duration = Duration::from_millis(20);

pub struct VolumeChangeGuard {
    last_programmatic_change: Mutex<Option<Instant>>,
}

impl VolumeChangeGuard {
    pub fn new() -> Self {
        Self {
            last_programmatic_change: Mutex::default(),
        }
    }

    pub fn mark(&self) {
        if let Ok(mut t) = self.last_programmatic_change.lock() {
            *t = Some(Instant::now())
        }
    }

    pub fn should_ignore(&self, within: Duration) -> bool {
        self.last_programmatic_change
            .lock()
            .unwrap()
            .is_some_and(|t| t.elapsed() < within)
    }
}
