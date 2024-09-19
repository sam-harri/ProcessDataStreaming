use rdkafka::consumer::{self, Consumer};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::Message;
use rdkafka::config::ClientConfig;


pub async fn start() {
    let consumer : StreamConsumer = create().await;
    consume(&consumer).await;
}

pub async fn create() -> StreamConsumer{
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "test-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");
    consumer
}

pub async fn consume(consumer: &StreamConsumer) {
    consumer.subscribe(&["test-topic"]).expect("Subscription failed");

    loop {
        match consumer.recv().await {
            Ok(msg) => {
               match msg.payload_view::<str>() {
                   Some(Ok(payload)) => {
                       println!("Received message: {}", payload);
                   }
                   Some(Err(e)) => {
                       eprintln!("Error while deserializing message payload: {:?}", e);
                   }
                   None => {
                       eprintln!("Payload is null");
                   }
               }
               consumer.commit_message(&msg, consumer::CommitMode::Async).expect("Commit failed");
            }
            Err(e) => {
                eprintln!("Error while receiving message: {:?}", e);
            }
        }
    }
}