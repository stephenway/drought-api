use crate::models::sensor::SensorData;
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn process_sensor_data(data: SensorData) {
    println!("Processing sensor data: {:?}", data);
    // TODO: Store in DB or send via MQTT
}

pub async fn fetch_latest_sensor_data() -> SensorData {
    SensorData { ph: 6.5, ec: 1.2, temperature: 22.5, humidity: 70.0, water_level: 80.0 }
}

