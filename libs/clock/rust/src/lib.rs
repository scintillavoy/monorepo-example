use chrono::{DateTime, Utc};

/// A trait for getting the current time, which can be implemented by different clock types.
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

/// A real clock that returns the current UTC time using `chrono`.
#[derive(Clone, Default)]
pub struct UtcClock;

impl Clock for UtcClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// A clock that synchronizes `chrono`'s real-world time with `tokio::time`'s internal timer.
///
/// While it functions as a standard, reliable system clock in a normal production environment,
/// it is particularly powerful for asynchronous testing. Because it respects Tokio's internal
/// timer, it seamlessly integrates with Tokio's paused time capabilities. This allows you to
/// instantly fast-forward through timeouts and delays in your tests while maintaining completely
/// accurate and predictable `chrono` timestamps.
///
/// # Examples
///
/// Using `TokioClock` in a test with paused time:
///
/// ```rust
/// use chrono::Utc;
/// use clock::{Clock, TokioClock};
/// use tokio::time::{sleep, Duration};
///
/// #[tokio::test(start_paused = true)]
/// async fn test_with_paused_time() {
///     let clock = TokioClock::new_at_now();
///     let start_time = clock.now();
///
///     // Sleep for 1 hour. Because start_paused is true, Tokio auto-advances
///     // the timer, and this resolves instantly without actually waiting.
///     sleep(Duration::from_secs(3600)).await;
///
///     // The clock automatically and mathematically reflects the 1 hour jump.
///     let elapsed_seconds = (clock.now() - start_time).num_seconds();
///     assert_eq!(elapsed_seconds, 3600);
/// }
/// ```
#[derive(Clone)]
pub struct TokioClock {
    base_date_time: DateTime<Utc>,
    base_instant: tokio::time::Instant,
}

impl TokioClock {
    /// Creates a new clock anchored to the provided `base_date_time`.
    ///
    /// # Panics
    ///
    /// This method must be called from within the context of a Tokio runtime.
    /// If called outside of a Tokio runtime context (e.g., a standard synchronous test setup),
    /// `tokio::time::Instant::now()` will panic.
    pub fn new(base_date_time: DateTime<Utc>) -> Self {
        Self {
            base_date_time,
            base_instant: tokio::time::Instant::now(),
        }
    }

    /// Convenience method to create a clock starting at the current UTC time.
    ///
    /// # Panics
    ///
    /// This method must be called from within the context of a Tokio runtime.
    /// If called outside of a Tokio runtime context, `tokio::time::Instant::now()` will panic.
    pub fn new_at_now() -> Self {
        Self::new(Utc::now())
    }
}

impl Default for TokioClock {
    /// Creates a clock starting at the current UTC time.
    ///
    /// # Panics
    ///
    /// This implementation must be called from within the context of a Tokio runtime.
    /// If called outside of a Tokio runtime context, `tokio::time::Instant::now()` will panic.
    fn default() -> Self {
        Self::new_at_now()
    }
}

impl Clock for TokioClock {
    fn now(&self) -> DateTime<Utc> {
        // Calculate how much time Tokio thinks has passed.
        let elapsed = tokio::time::Instant::now().saturating_duration_since(self.base_instant);

        // Convert std::time::Duration to chrono::Duration and add to the baseline.
        let chrono_duration = chrono::Duration::from_std(elapsed)
            .expect("Elapsed time exceeded chrono::Duration limits");

        self.base_date_time + chrono_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, advance, sleep};

    #[tokio::test]
    async fn test_tokio_clock_advances_by_sleeping_in_real_time() {
        let clock = TokioClock::new_at_now();
        let time_before = clock.now();

        sleep(Duration::from_millis(10)).await;

        let time_after = clock.now();

        // Because this is running in real time, we only assert that time moved strictly forward.
        // Checking for exact millisecond matches here leads to flaky tests.
        assert!(time_after > time_before);
    }

    #[tokio::test(start_paused = true)]
    async fn test_tokio_clock_advances_by_calling_advance() {
        let clock = TokioClock::new_at_now();
        let time_before = clock.now();

        advance(Duration::from_secs(3600)).await;

        let time_after = clock.now();
        let elapsed_seconds = (time_after - time_before).num_seconds();

        assert_eq!(elapsed_seconds, 3600);
    }

    #[tokio::test(start_paused = true)]
    async fn test_tokio_clock_advances_by_sleeping_with_paused_time() {
        let clock = TokioClock::new_at_now();
        let time_before = clock.now();

        sleep(Duration::from_secs(300)).await;

        let time_after = clock.now();
        let elapsed_seconds = (time_after - time_before).num_seconds();

        assert_eq!(elapsed_seconds, 300);
    }
}
