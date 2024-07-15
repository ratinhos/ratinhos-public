const ENABLED: bool = true;
const PROVIDER: &str = "https://v6.tnedi.me";

use crate::{Result, HTTP};
pub async fn execute() -> Result<()> {
    if !ENABLED {
        Err("module not enabled")?;
    };

    let response = HTTP
        .get(PROVIDER)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    crate::writeln!("[HTTP IPv6] {}\n", response.replace('\n', "")).await;

    Ok(())
}
