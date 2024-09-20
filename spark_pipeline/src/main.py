# main.py
from spark_session import get_spark_session
from pipelines.hx_pipeline import hx_pipeline
from pipelines.distillation_pipeline import distillation_pipeline
from pyspark.sql.streaming import StreamingQuery

def main():
    spark = get_spark_session()

    # Heat Exchanger Pipeline
    df_hx = hx_pipeline(spark)
    query_hx: StreamingQuery = (df_hx
        .writeStream
        .outputMode("append")
        .format("console")
        .option("truncate", "false")
        .start()
    )

    # Distillation Column Pipeline
    df_distillation = distillation_pipeline(spark)
    query_distillation: StreamingQuery = (df_distillation
        .writeStream
        .outputMode("append")
        .format("console")
        .option("truncate", "false")
        .start()
    )

    # Await termination
    spark.streams.awaitAnyTermination()

if __name__ == "__main__":
    main()
