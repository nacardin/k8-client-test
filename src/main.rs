use k8_client::meta_client::MetadataClient;
use k8_client::K8Client;
use k8_types::core::pod::{PodSpec, PodStatus};

#[async_std::main]
async fn main() {
    let client = K8Client::default().expect("cluster not initialized");

    let pod_items = client
        .retrieve_items::<PodSpec, _>("default")
        .await
        .expect("pods should exist");

    for pod in pod_items.items {
        println!("pod: {:#?}", pod);
    }
}
