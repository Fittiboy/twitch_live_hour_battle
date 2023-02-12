use crate::time_utils::{current_month, timespan, timestamp};
use chrono::{DateTime, Datelike};
use chrono_tz::Tz;
use serde::Deserialize;
use serde_json::{Number, Value};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Videos {
    pub data: Vec<Video>,
    pub pagination: Pagination,
}

impl Videos {
    pub fn hours(self) -> f64 {
        let videos = self.videos_this_month();
        videos
            .into_iter()
            .map(|video| video.hours_this_month())
            .sum()
    }

    fn videos_this_month(self) -> Vec<Video> {
        self.data
            .into_iter()
            .filter(|video| video.this_month())
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: String,
    pub stream_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub published_at: String,
    pub url: String,
    pub thumbnail_url: String,
    pub viewable: String,
    pub view_count: Number,
    pub language: String,
    pub r#type: String,
    pub duration: String,
    pub muted_segments: Option<Value>,
}

impl Video {
    fn this_month(&self) -> bool {
        let end_time = self.end_time();
        end_time.month() == current_month()
    }

    // TODO: This returns the total duration for the stream, not the duration of the part of the
    // stream that matches the month. Remove the time from before the month from the total duration
    // to return the correct duration
    fn hours_this_month(&self) -> f64 {
        let duration = timespan(&self.duration);
        let seconds = duration.num_seconds();
        seconds as f64 / 3600.
    }

    fn end_time(&self) -> DateTime<Tz> {
        let start_time = timestamp(&self.created_at);
        let duration = timespan(&self.duration);
        start_time + duration
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub data: Vec<User>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: Option<Number>,
    pub email: Option<String>,
    pub created_at: String,
}
