use std::time::Duration;

use aws_credentials::container_credentials::AwsContainerCredentialsProvider;
use aws_timestream::timestream::DimensionValueType::VARCHAR;
use aws_timestream::timestream::MeasureValueType::{BIGINT, DOUBLE};
use aws_timestream::timestream::TimeUnit::NANOSECONDS;
use aws_timestream::timestream::{Dimension, Record, Timestream, WriteRequest};
use chrono::Utc;
use rand::Rng;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    env_logger::init();
    let aws_credentials_provider = AwsContainerCredentialsProvider::new().await;
    let timestream = Timestream::new("ingest", "us-east-1", aws_credentials_provider.clone()).await;

    let dimensions = vec![
        Dimension {
            dimension_value_type: VARCHAR,
            name: "microservice".to_owned(),
            value: "test-ms".to_owned(),
        },
        Dimension {
            dimension_value_type: VARCHAR,
            name: "host".to_owned(),
            value: "192.168.10.10".to_owned(),
        },
    ];

    loop {
        let mut rng = rand::thread_rng();
        let timestream = timestream.read().await;

        sleep(Duration::from_secs(1)).await;

        let time = Utc::now().timestamp_nanos();
        let time = time.to_string();

        let write_request = WriteRequest {
            database_name: "sampleDB",
            table_name: "prueba",
            records: vec![
                Record::new(
                    &dimensions,
                    "cpu",
                    rng.gen::<f64>().to_string(),
                    DOUBLE,
                    &time,
                    NANOSECONDS,
                    1,
                ),
                Record::new(
                    &dimensions,
                    "request",
                    rng.gen::<u32>().to_string(),
                    BIGINT,
                    &time,
                    NANOSECONDS,
                    1,
                ),
            ],
        };

        let response = timestream.write(&write_request).await;
        println!("write:{:?}", response);
    }
}
