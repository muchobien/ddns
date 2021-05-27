mod settings;

mod providers;

use std::thread::{self};

use settings::Settings;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;

    loop {
        let ip = public_ip::addr()
            .await
            .ok_or(eyre::eyre!("Unable to get public ip"))?;

        settings.provider()?.update(ip).await?;
        thread::sleep(Duration::from_secs(5));
    }
}
