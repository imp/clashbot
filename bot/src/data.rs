use std::borrow::Borrow;
use std::ops;

#[derive(Debug)]
pub(super) struct Timestamped<T> {
    timestamp: time::OffsetDateTime,
    data: T,
}

impl<T> Timestamped<T> {
    pub(super) fn timestamp(&self) -> time::OffsetDateTime {
        self.timestamp
    }

    pub(super) fn data(&self) -> &T {
        &self.data
    }
}

impl<T> From<T> for Timestamped<T> {
    fn from(data: T) -> Self {
        Self {
            timestamp: time::OffsetDateTime::now_utc(),
            data,
        }
    }
}

impl<T> ops::Deref for Timestamped<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Borrow<T> for &&Timestamped<T> {
    fn borrow(&self) -> &T {
        &self.data
    }
}
