mod settings;

mod providers;

use settings::Settings;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;

    let ip = public_ip::addr()
        .await
        .ok_or(eyre::eyre!("Unable to get public ip"))?;

    settings.provider()?.update(ip).await?;

    Ok(())
}
