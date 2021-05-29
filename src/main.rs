mod cron;
mod providers;
mod settings;

use chrono::Utc;
use cron::Cron;
use settings::Settings;
use std::thread;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;
    let mut cron = Cron::new(&settings.cron, &Utc::now())?;
    let provider = settings.provider()?;

    loop {
        let ip = public_ip::addr()
            .await
            .ok_or(eyre::eyre!("Unable to get public ip"))?;
        provider.update(ip).await?;
        let duration = cron.duration()?;
        println!("next update at {:?}", duration);
        thread::sleep(duration);
        cron.next();
    }
}
