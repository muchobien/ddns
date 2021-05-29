mod providers;
mod settings;

use chrono::Utc;
use cron::Schedule;
use settings::Settings;
use std::{str::FromStr, thread};

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;
    let provider = settings.provider()?;
    let schedule = Schedule::from_str(&settings.cron)?;

    let mut prev = Utc::now();
    let mut iter = schedule.after(&prev);

    loop {
        let current = iter.next().ok_or(eyre::eyre!("No more upcoming events"))?;
        let duration = (current - prev).to_std()?;

        prev = current;

        let ip = public_ip::addr()
            .await
            .ok_or(eyre::eyre!("Unable to get public ip"))?;

        provider.update(ip).await?;
        println!("next update at {:?}", duration);

        thread::sleep(duration);
    }
}
