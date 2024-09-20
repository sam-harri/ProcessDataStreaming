// heat_exchanger.rs

use avro_rs::{to_avro_datum, to_value, Schema};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::producer::UnitDataProducer;
use crate::schema_loader::get_schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeatExchangerData {
    uuid: String,
    timestamp: i64,
    tc_in: f32,
    tc_out: f32,
    th_in: f32,
    th_out: f32,
    flow_rate: f32,
    pressure_drop: f32,
}

pub struct HeatExchanger {
    uuid: String,
    producer: UnitDataProducer,
    schema: Schema,
}

impl HeatExchanger {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        let schema = get_schema(
            "heat_exchanger",
            "../schemas/heat_exchanger.avsc",
        );
        let producer = UnitDataProducer::new();

        HeatExchanger {
            uuid,
            producer,
            schema,
        }
    }

    fn generate_data(&self) -> HeatExchangerData {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        HeatExchangerData {
            uuid: self.uuid.clone(),
            timestamp: now,
            tc_in: rand::random::<f32>() * 100.0,
            tc_out: rand::random::<f32>() * 100.0,
            th_in: rand::random::<f32>() * 100.0,
            th_out: rand::random::<f32>() * 100.0,
            flow_rate: rand::random::<f32>() * 10.0,
            pressure_drop: rand::random::<f32>() * 5.0,
        }
    }

    pub async fn produce_data(&self, topic: &str) {
        loop {
            let data = self.generate_data();
            let value = to_value(data).unwrap();
            let encoded = to_avro_datum(&self.schema, value).unwrap();
            self.producer.produce(topic, &encoded).await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
