use std::time::Instant;

use colored::Colorize;

use crate::logger::time::time;

mod time;

pub fn _log_prompt<S: AsRef<str>>(message: S) -> String {
    format!("{} {}", time(), message.as_ref().bright_white())
}

pub fn log_measure<S: AsRef<str>>(message: S) -> LogMeasure {
    let measure = LogMeasure::new(message);
    measure.log(Measure::Start);

    measure
}

pub fn log_info<S: AsRef<str>>(message: S) {
    let message = format_log_message(message);

    println!("{} {:10} {}", time(), "Info".bright_blue(), message);
}

pub fn log_warn<S: AsRef<str>>(message: S) {
    let message = format_log_message(message);

    println!("{} {:10} {}", time(), "Warning".bright_yellow(), message);
}

fn format_log_message<S: AsRef<str>>(message: S) -> String {
    format!("'{}'", message.as_ref().cyan())
}

pub struct LogMeasure {
    message: String,
    start: Instant,
}

enum Measure {
    Start,
    End,
}

impl LogMeasure {
    pub fn finish(self) {
        self.log(Measure::End);
    }

    fn new<S: AsRef<str>>(message: S) -> LogMeasure {
        let message = message.as_ref().to_string();

        LogMeasure {
            message,
            start: Instant::now(),
        }
    }

    fn log(&self, measure: Measure) {
        let time = time();
        let message = self.message.cyan();

        let (measure, suffix) = match measure {
            Measure::Start => ("Starting:", "…".to_string()),
            Measure::End => {
                let elapsed = self.start.elapsed();
                let time = format!("{:?}", elapsed).purple();
                let suffix = format!("after {}", time);

                ("Finished:", suffix)
            }
        };

        println!(
            "{time} {measure:10} '{message}' {suffix}",
            time = time,
            measure = measure.white(),
            message = message,
            suffix = suffix
        )
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn log_sample() {
        let log = log_measure("test measurement");
        std::thread::sleep(Duration::from_micros(200));

        log.finish()
    }
}
