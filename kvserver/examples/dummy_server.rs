use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use tokio::{
    net::TcpListener,
};
use kvserver::{CommondRequest, CommandResponse};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("start listen on: {}", addr);

    loop {
        let (tcp_stream, addr) = listener.accept().await?;
        info!("client: {} connected", addr);

        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, CommondRequest, CommandResponse, _>::from(tcp_stream).for_async();
            while let Some(Ok(msg)) = stream.next().await {
                info!("got a new command: {:?}", msg);

                let mut resp = CommandResponse::default();
                resp.status = 404;
                resp.message = "Not Found".to_string();
                stream.send(resp).await.unwrap();
            }
            info!("client disconnected: {}", addr);
        });
    }
}