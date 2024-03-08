use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct Timestamp {
    // The number of
    pub time: u128,
}

impl Timestamp {
    pub(crate) fn new(time: u128) -> Timestamp {
        Timestamp { time }
    }

    pub(crate) fn now() -> Result<Timestamp, SystemTimeError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| Timestamp::new(duration.as_millis()))
    }
}
