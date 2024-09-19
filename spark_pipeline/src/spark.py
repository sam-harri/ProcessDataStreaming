from pyspark.sql import SparkSession
from pyspark.sql.functions import explode
from pyspark.sql.functions import split
from pyspark.sql import SparkSession, DataFrame
from pyspark.sql.streaming import StreamingQuery
from pyspark.sql.avro.functions import from_avro
from pyspark.sql.functions import col

def spark_init() -> None:
    spark: SparkSession = (SparkSession
        .builder
        .appName("StructuredNetworkWordCount")
        .config("spark.jars.packages", "org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0,org.apache.spark:spark-avro_2.12:3.5.2")
        .getOrCreate()
    )
    
    if not isinstance(spark, SparkSession):
        raise Exception("Spark session is not initialized")
    
    jsonFormatSchema = open("schemas/heat_exchanger.avsc", "r").read()

    df: DataFrame = (spark
        .readStream
        .format("kafka")
        .option("kafka.bootstrap.servers", "localhost:9092")
        .option("subscribe", "test-topic")
        # .option("includeHeaders", "true")
        .load()
    )

    df = (df
    .select(from_avro("value", jsonFormatSchema).alias("hx"), col("key").cast("string"))
    )

    # Project each field in the struct to its own column
    df = df.select(
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
        
    query: StreamingQuery = (df
        .writeStream
        .outputMode("append")
        .format("console")
        .start()
    )

    query.awaitTermination()