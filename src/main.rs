use dotenv::dotenv;

mod persistence;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = persistence::create_client();
    let data = persistence::read(&client).await;
    dbg!(data);

    Ok(())
}
