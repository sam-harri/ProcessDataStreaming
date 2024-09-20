// distillation_column.rs

use avro_rs::{to_avro_datum, to_value, Schema};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::producer::UnitDataProducer;
use crate::schema_loader::get_schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct DistillationColumnData {
    uuid: String,
    timestamp: i64,
    temperature: f32,
    pressure: f32,
    reflux_ratio: f32,
    bottom_flow_rate: f32,
    top_flow_rate: f32,
}

pub struct DistillationColumn {
    uuid: String,
    producer: UnitDataProducer,
    schema: Schema,
}

impl DistillationColumn {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        let schema = get_schema(
            "distillation_column",
            "../schemas/distillation_column.avsc",
        );
        let producer = UnitDataProducer::new();

        DistillationColumn {
            uuid,
            producer,
            schema,
        }
    }

    fn generate_data(&self) -> DistillationColumnData {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        DistillationColumnData {
            uuid: self.uuid.clone(),
            timestamp: now,
            temperature: rand::random::<f32>() * 200.0,
            pressure: rand::random::<f32>() * 10.0,
            reflux_ratio: rand::random::<f32>(),
            bottom_flow_rate: rand::random::<f32>() * 100.0,
            top_flow_rate: rand::random::<f32>() * 100.0,
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
