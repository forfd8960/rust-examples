use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use tokio::{
    net::TcpListener,
};
use kvserver::{CommondRequest, CommandResponse, Service, MemTable};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt::init();

    let service = Service::new(MemTable::new());



    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("start listen on: {}", addr);

    loop {
        let (tcp_stream, addr) = listener.accept().await?;
        info!("client: {} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            let mut stream = AsyncProstStream::<_, CommondRequest, CommandResponse, _>::from(tcp_stream).for_async();
            while let Some(Ok(cmd)) = stream.next().await {
                info!("got a new command: {:?}", cmd);

                let resp = svc.execute(cmd);
                stream.send(resp).await.unwrap();
            }
            info!("client disconnected: {}", addr);
        });
    }
}