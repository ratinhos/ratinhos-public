const ENABLED: bool = true;

use crate::Result;
use tokio::io::{stdout, AsyncWriteExt};

pub async fn write(content: &str) -> Result<()> {
    if !ENABLED {
        Err("module not enabled")?;
    };

    stdout().write_all(content.as_bytes()).await?;

    Ok(())
}
