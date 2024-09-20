use avro_rs::Schema;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

static SCHEMA_CACHE: Lazy<Mutex<HashMap<String, Schema>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn get_schema(schema_name: &str, schema_path: &str) -> Schema {
    let mut cache = SCHEMA_CACHE.lock().unwrap();

    if let Some(schema) = cache.get(schema_name) {
        return schema.clone();
    }

    let mut file = File::open(schema_path)
        .unwrap_or_else(|_| panic!("Unable to open schema file: {}", schema_path));
    let mut raw_schema = String::new();
    file.read_to_string(&mut raw_schema)
        .unwrap_or_else(|_| panic!("Unable to read schema file: {}", schema_path));
    let schema = Schema::parse_str(&raw_schema)
        .unwrap_or_else(|_| panic!("Unable to parse schema: {}", schema_path));

    cache.insert(schema_name.to_string(), schema.clone());

    schema
}
