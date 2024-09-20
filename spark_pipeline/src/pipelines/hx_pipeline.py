# pipelines/hx_pipeline.py
from pyspark.sql import SparkSession, DataFrame
from pyspark.sql.functions import col
from pyspark.sql.avro.functions import from_avro

def hx_pipeline(spark: SparkSession) -> DataFrame:
    # Read schema
    hx_schema = open("../schemas/heat_exchanger.avsc", "r").read()

    # Create DataFrame
    df_hx = (spark
        .readStream
        .format("kafka")
        .option("kafka.bootstrap.servers", "localhost:9092")
        .option("subscribe", "hx-topic")
        .load()
    )

    df_hx = df_hx.select(
        from_avro("value", hx_schema).alias("hx"),
        col("key").cast("string")
    ).select(
        col("key"),
        col("hx.uuid").alias("uuid"),
        col("hx.timestamp").alias("timestamp"),
        col("hx.tc_in").alias("tc_in"),
        col("hx.tc_out").alias("tc_out"),
        col("hx.th_in").alias("th_in"),
        col("hx.th_out").alias("th_out"),
        col("hx.flow_rate").alias("flow_rate"),
        col("hx.pressure_drop").alias("pressure_drop")
    )

    return df_hx
