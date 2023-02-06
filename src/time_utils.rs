use chrono::{Datelike, Utc};
use chrono_tz::US::Pacific;

pub fn current_month() -> u32 {
    let now = Utc::now().with_timezone(&Pacific);
    now.month()
}
