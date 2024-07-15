use ratinhos::{modules::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    ratinhos::execute!(ip::dns::v4, ip::dns::v6, ip::http::v4, ip::http::v6).await;
    Ok(())
}
