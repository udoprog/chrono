use core::fmt;

/// Internal representation of the chrono error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ChronoErrorKind {
    InvalidDate,
    InvalidTime,
    InvalidDateTime,
    AmbiguousDate,
    SystemTimeBeforeEpoch,
}

/// The error raised for an invalid date time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChronoError {
    kind: ChronoErrorKind,
}

impl ChronoError {
    /// Internal constructor for a chrono error.
    #[inline]
    pub(crate) fn new(kind: ChronoErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for ChronoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ChronoErrorKind::InvalidDate => write!(f, "invalid date"),
            ChronoErrorKind::InvalidTime => write!(f, "invalid time"),
            ChronoErrorKind::InvalidDateTime => write!(f, "invalid date time"),
            ChronoErrorKind::AmbiguousDate => write!(f, "tried to operate over ambiguous date"),
            ChronoErrorKind::SystemTimeBeforeEpoch => write!(f, "system time before Unix epoch"),
        }
    }
}

impl From<ChronoErrorKind> for ChronoError {
    #[inline]
    fn from(kind: ChronoErrorKind) -> Self {
        Self::new(kind)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChronoError {}
