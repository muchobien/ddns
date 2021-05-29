use chrono::{DateTime, TimeZone};
use cron_parser::parse;
use std::time::Duration;
pub struct Cron<Tz: TimeZone> {
    current: DateTime<Tz>,
    next: DateTime<Tz>,
    cron: String,
}

impl<Tz: TimeZone> Cron<Tz> {
    pub fn new(cron: String, now: &DateTime<Tz>) -> eyre::Result<Self> {
        let current = parse(&cron, &now)?;
        let next = parse(&cron, &current)?;

        Ok(Self {
            current,
            next,
            cron,
        })
    }

    pub fn next(&mut self) -> eyre::Result<()> {
        self.current = self.next.clone();
        self.next = parse(&self.cron, &self.current)?;

        Ok(())
    }

    pub fn duration(&self) -> eyre::Result<Duration> {
        let diff = self.next.clone() - self.current.clone();
        Ok(diff.to_std()?)
    }
}
