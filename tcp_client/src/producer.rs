use std::time::Duration;

use rdkafka::{producer::{FutureProducer, FutureRecord}, util::Timeout, ClientConfig};
use avro_rs::Writer;

use crate::heat_exchanger::{get_schema, HeatExchanger};


pub fn create() -> FutureProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    producer
}

pub async fn produce(future_producer: &FutureProducer, topic: &str, message: &[u8]) {
    let record = FutureRecord::to(topic)
        .payload(message)
        .key("key");

    let delivery_status = future_producer
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

pub async fn produce_heat_exchanger_data(future_producer: &FutureProducer, topic: &str) {
    let schema = get_schema();
    loop {
        let data = HeatExchanger::new();
        let mut writer = Writer::new(&schema, Vec::new());
        match writer.append_ser(data) {
            Ok(_) => {
                let encoded = writer.into_inner().unwrap();
                produce(future_producer, topic, &encoded).await;
            }
            Err(e) => {
                eprintln!("Failed to serialize data: {:?}", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}