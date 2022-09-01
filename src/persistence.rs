use influxdb2_client::models::Query;
use influxdb2_client::Client as InfluxDbClient;
use lazy_static::lazy_static;

lazy_static! {
    static ref INFLUXDB_URL: String =
        std::env::var("INFLUXDB_URL").expect("Can't read InfluxDB URL");
    static ref INFLUXDB_TOKEN: String =
        std::env::var("INFLUXDB_TOKEN").expect("Can't read InfluxDB token");
    static ref INFLUXDB_BUCKET: String =
        std::env::var("INFLUXDB_BUCKET").expect("Can't read InfluxDB bucket name");
    static ref INFLUXDB_ORG: String =
        std::env::var("INFLUXDB_ORG").expect("Can't read InfluxDB organization");
}

pub fn create_client() -> InfluxDbClient {
    InfluxDbClient::new(INFLUXDB_URL.as_str(), INFLUXDB_TOKEN.as_str())
}

pub async fn read(influxdb_client: &InfluxDbClient) -> String {
    let query = r#"
        from(bucket: "test_empty_db")
            |> range(start: -1h)
    "#;

    influxdb_client
        .query(INFLUXDB_ORG.as_str(), Some(Query::new(query.to_string())))
        .await
        .expect("Error reading data")
}
