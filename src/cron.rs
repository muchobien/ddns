use chrono::{DateTime, TimeZone};
use cron_parser::parse;
use std::time::Duration;
pub struct Cron<'a, Tz: TimeZone> {
    current: DateTime<Tz>,
    next: DateTime<Tz>,
    cron: &'a String,
}

impl<'a, Tz: TimeZone> Cron<'a, Tz> {
    pub fn new(cron: &'a String, now: &DateTime<Tz>) -> eyre::Result<Self> {
        let current = parse(cron, &now)?;
        let next = parse(cron, &current)?;

        Ok(Self {
            current,
            next,
            cron,
        })
    }

    pub fn next(&mut self) {
        self.current = self.next.clone();
        self.next = parse(self.cron, &self.current).unwrap();
    }

    pub fn duration(&self) -> eyre::Result<Duration> {
        let diff = self.next.clone() - self.current.clone();
        Ok(diff.to_std()?)
    }
}
