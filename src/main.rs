use eyre::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let ip = public_ip::addr().await.ok_or(eyre::eyre!("not found"))?;
    println!("public ip address: {:?}", ip);
    Ok(())
}
