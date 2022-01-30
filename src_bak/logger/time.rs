use chrono::{Local, Timelike};
use colored::Colorize;

pub(super) fn time() -> String {
    format!("[{}]", time_colored())
}

fn time_colored() -> String {
    time_raw().bright_black().to_string()
}

fn time_raw() -> String {
    let now = Local::now();

    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}
