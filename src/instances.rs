use std::process::exit;

use containerd_client::{
    connect, services::v1::containers_client::ContainersClient, tonic::transport::Channel,
};
use tokio::sync::OnceCell;

static CHANNEL: OnceCell<Channel> = OnceCell::const_new();
static CONTAINERS_CLIENT: OnceCell<ContainersClient<Channel>> = OnceCell::const_new();

async fn get_channel() -> Channel {
    match CHANNEL
        .get_or_try_init(|| async { connect("/run/containerd/containerd.sock").await })
        .await
    {
        Ok(channel) => channel.clone(),
        Err(err) => {
            eprintln!("failed to connect to channel: {err:?}");
            exit(1);
        }
    }
}

pub async fn get_containers_client() -> ContainersClient<Channel> {
    let client = CONTAINERS_CLIENT
        .get_or_init(|| async {
            let channel = get_channel().await;
            ContainersClient::new(channel)
        })
        .await;

    client.clone()
}
