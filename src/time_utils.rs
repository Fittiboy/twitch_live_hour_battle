use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::US::Pacific;
use chrono_tz::UTC;
use regex::Regex;

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
    let re = Regex::new(r"((\d{1,2})h)?((\d{1,2})m)?((\d{1,2})s)?")
        .expect("static regex will always work");
    let captures = re
        .captures(timestring)
        .expect("Twitch should always return matchable duration strings");
    let seconds = seconds_from_re_captures(captures);
    dbg!(Duration::seconds(seconds))
}

fn seconds_from_re_captures(captures: regex::Captures) -> i64 {
    let mut total: i64 = 0;
    let hours = &captures[2];
    if !hours.is_empty() {
        total += hours
            .parse::<i64>()
            .expect("hours returned from Twitch can only be integers")
            * 3600;
    }
    let minutes = &captures[4];
    if !minutes.is_empty() {
        total += minutes
            .parse::<i64>()
            .expect("minutes returned from Twitch can only be integers")
            * 60;
    }
    let seconds = &captures[6];
    if !seconds.is_empty() {
        total += seconds
            .parse::<i64>()
            .expect("seconds returned from Twitch can only be integers");
    }
    total
}
