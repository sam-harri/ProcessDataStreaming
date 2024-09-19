use heat_exchanger::HeatExchanger;
use tokio::task;

mod producer;
mod heat_exchanger;

#[tokio::main]
async fn main() {
    let heat_exchanger = HeatExchanger::new();
    let topic = "test-topic";

    let producer_handle = task::spawn(async move {
        heat_exchanger.produce_data(topic).await;
    });

    producer_handle.await.unwrap();
}
