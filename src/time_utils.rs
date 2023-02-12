use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::US::Pacific;
use chrono_tz::UTC;

pub fn current_month() -> u32 {
    let now = Utc::now().with_timezone(&Pacific);
    now.month()
}

pub fn timestamp(timestring: &str) -> DateTime<Tz> {
    let utc_timestamp = UTC
        .datetime_from_str(timestring, "%Y-%m-%dT%H:%M:%SZ")
        .expect("API should always provide corret timestamp");
    utc_timestamp.with_timezone(&Pacific)
}

pub fn timespan(timestring: &str) -> Duration {
    let std_duration = parse_duration::parse(timestring)
        .expect("Twitch should always return a parseable duration string");
    Duration::from_std(std_duration).expect("any parsed duration from Twitch should be in range")
}
