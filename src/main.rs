use std::{sync::Arc, time::Duration};

use aws_timestream::timestream::DimensionValueType::VARCHAR;
use aws_timestream::timestream::MeasureValueType::{BIGINT, DOUBLE};
use aws_timestream::timestream::TimeUnit::NANOSECONDS;
use aws_timestream::timestream::{Dimension, Record, Timestream, WriteRequest};
use chrono::Utc;
use rand::Rng;
use tokio::sync::RwLock;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    let aws_access_key_id = env!("AWS_ACCESS_KEY_ID");
    let aws_secret_access_key = env!("AWS_SECRET_ACCESS_KEY");

    let timestream = Arc::new(RwLock::new(
        Timestream::new(
            "ingest".to_string(),
            "us-east-1".to_string(),
            aws_access_key_id.to_string(),
            aws_secret_access_key.to_string(),
        )
        .await
        .unwrap(),
    ));

    let refresh_timestream = timestream.clone();

    tokio::spawn(async move {
        loop {
            let timestream = refresh_timestream.read().await;
            timestream.await_to_reload().await;
            drop(timestream);
            let mut timestream = refresh_timestream.write().await;
            timestream.reload_enpoints().await;
        }
    });

    let dimensions = vec![
        &Dimension {
            dimension_value_type: VARCHAR,
            name: "microservice",
            value: "test-ms",
        },
        &Dimension {
            dimension_value_type: VARCHAR,
            name: "host",
            value: "192.168.10.10",
        },
    ];

    loop {
        let mut rng = rand::thread_rng();
        let timestream = timestream.clone();
        let timestream = timestream.read().await;

        sleep(Duration::from_secs(1)).await;

        let time = Utc::now().timestamp_nanos();
        let time = time.to_string();

        let write_request = WriteRequest {
            database_name: "sampleDB",
            table_name: "prueba",
            records: vec![
                Record {
                    dimensions: &dimensions,
                    measure_name: "cpu",
                    measure_value: rng.gen::<f64>().to_string(),
                    measure_value_type: DOUBLE,
                    time: &time,
                    time_unit: NANOSECONDS,
                    version: 1,
                },
                Record {
                    dimensions: &dimensions,
                    measure_name: "request",
                    measure_value: rng.gen::<u32>().to_string(),
                    measure_value_type: BIGINT,
                    time: &time,
                    time_unit: NANOSECONDS,
                    version: 1,
                },
            ],
        };

        let response = timestream.write(write_request).await;
        println!("{:?}", response);
    }
}
