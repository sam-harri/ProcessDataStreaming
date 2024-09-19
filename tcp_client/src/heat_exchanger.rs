use avro_rs::Schema;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeatExchanger {
    timestamp: i64,
    tc_in: f32,
    tc_out: f32,
    th_in: f32,
    th_out: f32,
    flow_rate: f32,
    pressure_drop: f32,
}

impl HeatExchanger {
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        HeatExchanger {
            timestamp: now,
            tc_in: rand::random::<f32>() * 100.0,
            tc_out: rand::random::<f32>() * 100.0,
            th_in: rand::random::<f32>() * 100.0,
            th_out: rand::random::<f32>() * 100.0,
            flow_rate: rand::random::<f32>() * 10.0,
            pressure_drop: rand::random::<f32>() * 5.0,
        }
    }
}

pub fn get_schema() -> Schema {
    let raw_schema = r#"
    {
      "type": "record",
      "name": "HeatExchanger",
      "namespace": "com.process.simulator",
      "fields": [
        {"name": "timestamp", "type": "long", "doc": "Unix timestamp in milliseconds"},
        {"name": "tc_in", "type": "float", "doc": "Coolant temperature in (°C)"},
        {"name": "tc_out", "type": "float", "doc": "Coolant temperature out (°C)"},
        {"name": "th_in", "type": "float", "doc": "Hot fluid temperature in (°C)"},
        {"name": "th_out", "type": "float", "doc": "Hot fluid temperature out (°C)"},
        {"name": "flow_rate", "type": "float", "doc": "Flow rate in m^3/h"},
        {"name": "pressure_drop", "type": "float", "doc": "Pressure drop in bar"}
      ]
    }
    "#;
    Schema::parse_str(raw_schema).unwrap()
}