from pyspark.sql import SparkSession

def get_spark_session(app_name: str = "ConcurrentStreaming") -> SparkSession:
    spark = (SparkSession
             .builder
             .appName(app_name)
             .config("spark.jars.packages", 
                     "org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0,"
                     "org.apache.spark:spark-avro_2.12:3.5.2")
             .getOrCreate())

    if not isinstance(spark, SparkSession):
        raise Exception("Spark session is not initialized")

    return spark
