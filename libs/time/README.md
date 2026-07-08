# Time library

This library provides utilities for handling time-related operations. It includes functions for parsing, formatting, and manipulating time values.

## Duration

### Code

Inside the code, any field or variable whose type representing a duration, such as `std::time::Duration`, must not include a unit suffix.

Example:

```rust
pub struct Job {
    pub pending_duration: std::time::Duration,
}
```

Reasoning:

- The type (`Duration`) already encodes the meaning.
- Unit suffixes (`_ms`, `_seconds`, ...) introduce redundancy and reduce readability.

### JSON and database

When serialized as JSON or stored in a database, a `Duration` must be converted into a numeric value representing a specific time unit. To avoid ambiguity, the field name must include a unit suffix.

Rust example:

```rust
#[serde(rename = "pending_duration_ms")]
#[serde_as(as = "DurationMilliSeconds<u64>")]
pub pending_duration: Duration;
```

JSON example:

```json
{
    "pending_duration_ms": 1500
}
```

MySQL schema example:

```sql
pending_duration_ms BIGINT NOT NULL
```

### Allowed unit suffixes

The following unit suffixes are allowed for duration fields in JSON and database representations:

- Nanoseconds: `_ns`, `Ns`
- Microseconds: `_us`, `Us`
- Milliseconds: `_ms`, `Ms`
- Seconds: `_seconds`, `Seconds`
- Minutes: `_minutes`, `Minutes`
- Hours: `_hours`, `Hours`
- Days: `_days`, `Days`
- Weeks: `_weeks`, `Weeks`
- Years: `_years`, `Years`
