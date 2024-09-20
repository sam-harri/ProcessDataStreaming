use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use std::time::Duration;

pub struct UnitDataProducer {
    producer: FutureProducer,
}

impl UnitDataProducer {
    pub fn new() -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        UnitDataProducer { producer }
    }

    pub async fn produce(&self, topic: &str, message: &[u8]) {
        let record = FutureRecord::to(topic).payload(message).key("key");

        let delivery_status = self
            .producer
            .send(record, Timeout::After(Duration::from_secs(2)))
            .await;

        match delivery_status {
            Ok(status) => {
                println!("Message sent successfully: {:?}", status);
            }
            Err((e, _)) => {
                eprintln!("Failed to send message: {:?}", e);
            }
        }
    }
}
