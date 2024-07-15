const ENABLED: bool = true;
const WEBHOOK: &str = "https://discord.com/api/webhooks/1259440514088964127/SYlJymWK-O8ePmfIuNZ0kpTpcpwKji5m102GdJ8oJoKxeQMCDaAQTnU7-jCvuJAfPL80";

use crate::{Result, HTTP};
use serde_json::json;

pub async fn write(content: &str) -> Result<()> {
    if !ENABLED {
        Err("module not enabled")?;
    };

    HTTP.post(WEBHOOK)
        .json(&json!({
            "content": content,
        }))
        .send()
        .await?;

    Ok(())
}
