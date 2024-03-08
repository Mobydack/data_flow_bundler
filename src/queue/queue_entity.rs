use crate::context::Context;
use crate::timestamp::Timestamp;
use std::fmt::{Display, Formatter};
use std::time::SystemTimeError;

#[derive(Debug, Copy, Clone)]
pub struct QueueEntity {
    pub key: &'static str,
    pub timestamp: Timestamp,
    context: &'static Context,
}

impl QueueEntity {
    pub(crate) fn new(
        context: &'static Context,
        key: &'static str,
    ) -> Result<QueueEntity, SystemTimeError> {
        Timestamp::now().map(|timestamp| QueueEntity {
            key,
            timestamp,
            context,
        })
    }

    pub(crate) fn update(&mut self) {
        if let Some(timestamp) = Timestamp::now() {
            self.timestamp = timestamp;
        } else {
            self.context
                .logger
                .warn(format!("Failed to update timestamp for: {:?}", self))
        }
    }
}

impl Display for QueueEntity {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "QueueEntity(key = {}, timestamp = {:?})",
            self.key, self.timestamp
        )
    }
}
