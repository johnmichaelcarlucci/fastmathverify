//! Cooperative, thread-local deadlines (replaces upstream's signal.alarm timeouts,
//! which only work in the main thread).

use std::cell::Cell;
use std::time::{Duration, Instant};

thread_local! {
    static DEADLINE: Cell<Option<Instant>> = const { Cell::new(None) };
}

/// True once the current thread's deadline (if any) has passed.
#[inline]
pub fn expired() -> bool {
    DEADLINE.with(|d| d.get().map(|t| Instant::now() >= t).unwrap_or(false))
}

/// Run `f` with a deadline of `secs` seconds (None or <= 0 disables it).
/// Returns `(result, timed_out)`.
pub fn run<T>(secs: Option<f64>, f: impl FnOnce() -> T) -> (T, bool) {
    let new = secs.filter(|s| *s > 0.0).map(|s| Instant::now() + Duration::from_secs_f64(s));
    let prev = DEADLINE.with(|d| d.replace(new));
    let out = f();
    let hit = expired();
    DEADLINE.with(|d| d.set(prev));
    (out, hit)
}
