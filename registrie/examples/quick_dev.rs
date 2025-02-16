use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8081")?;

    hc.do_get("/register").await?.print().await?;

    Ok(())
}
