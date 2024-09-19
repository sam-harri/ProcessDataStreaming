#!/bin/bash

# Function to register a schema
register_schema() {
  local schema_file=$1
  local subject_name=$2
  echo "Registering schema for subject: ${subject_name}"

  curl -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json" \
    --data "{\"schema\": \"$(cat $schema_file | jq -Rs .)\"}" \
    http://schema-registry:8081/subjects/${subject_name}/versions

  echo "Schema ${subject_name} registered successfully!"
}

# Register schemas
register_schema "/schemas/heat_exchanger.avsc" "heat_exchanger-value"

