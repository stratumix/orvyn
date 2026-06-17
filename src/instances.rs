use std::process::exit;

use containerd_client::{
    connect, services::v1::containers_client::ContainersClient, tonic::transport::Channel,
};
use tokio::sync::OnceCell;

static CONTAINERS_CLIENT: OnceCell<ContainersClient<Channel>> = OnceCell::const_new();

pub async fn get_containers_client() -> ContainersClient<Channel> {
    CONTAINERS_CLIENT
        .get_or_init(|| async {
            let channel = connect("/run/containerd/containerd.sock")
                .await
                .unwrap_or_else(|err| {
                    eprintln!("failed to connect to channel: {err:?}");
                    exit(1);
                });

            ContainersClient::new(channel)
        })
        .await
        .clone()
}
