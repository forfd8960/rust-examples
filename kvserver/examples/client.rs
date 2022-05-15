use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::{
    net::TcpStream,
};
use kvserver::{CommondRequest, CommandResponse};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let stream = TcpStream::connect(addr).await?;

    let mut client = AsyncProstStream::<_, CommandResponse, CommondRequest, _>::from(stream).for_async();

    let cmd = CommondRequest::new_hset("table1", "hello", "world".into());

    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response: {:?}", data);
    }
    Ok(())
}