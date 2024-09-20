# pipelines/distillation_pipeline.py
from pyspark.sql import SparkSession, DataFrame
from pyspark.sql.functions import col
from pyspark.sql.avro.functions import from_avro

def distillation_pipeline(spark: SparkSession) -> DataFrame:
    # Read schema
    distillation_schema = open("../schemas/distillation_column.avsc", "r").read()

    # Create DataFrame
    df_distillation = (spark
        .readStream
        .format("kafka")
        .option("kafka.bootstrap.servers", "localhost:9092")
        .option("subscribe", "distillation-topic")
        .load()
    )

    df_distillation = df_distillation.select(
        from_avro("value", distillation_schema).alias("distillation"),
        col("key").cast("string")
    ).select(
        col("key"),
        col("distillation.uuid").alias("uuid"),
        col("distillation.timestamp").alias("timestamp"),
        col("distillation.temperature").alias("temperature"),
        col("distillation.pressure").alias("pressure"),
        col("distillation.reflux_ratio").alias("reflux_ratio"),
        col("distillation.bottom_flow_rate").alias("bottom_flow_rate"),
        col("distillation.top_flow_rate").alias("top_flow_rate")
    )

    return df_distillation
