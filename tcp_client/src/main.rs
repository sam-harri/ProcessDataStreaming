use producer::{create, produce_heat_exchanger_data};
use tokio::task;

mod producer;
mod consumer;
mod heat_exchanger;


#[tokio::main]
async fn main() {

    let producer = create();

    let producer_handle = task::spawn(async move {
        produce_heat_exchanger_data(&producer, "test-topic").await;
    });

    producer_handle.await.unwrap();
}
