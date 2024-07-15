const ENABLED: bool = true;
const PROVIDER: &str = "whoami.cloudflare";
const SERVER: &str = "2606:4700:4700::1111:53";
const TYPE: Type = Type::TXT;
const CLASS: Class = Class::Chaos;

//test

use crate::Result;
use rustdns::{
    types::{Class, Type},
    Message,
};
use tokio::net::UdpSocket;

pub async fn execute() -> Result<()> {
    if !ENABLED {
        Err("module not enabled")?;
    };

    let mut msg = Message::default();
    msg.add_question(PROVIDER, TYPE, CLASS);
    let socket = UdpSocket::bind("[::]:0").await?;
    socket.connect(SERVER).await?;
    socket.send(&msg.to_vec()?).await?;
    let mut answer = [0; 4096];
    let len = socket.recv(&mut answer).await?;
    let msg = Message::from_slice(&answer[..len])?;
    let answer = msg.answers.first().ok_or("no answers")?;

    crate::writeln!(
        "[DNS IPv6] {}",
        answer.resource.to_string().replace("\"", "")
    )
    .await;

    Ok(())
}
