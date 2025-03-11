use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub ph: f32,
    pub ec: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub water_level: f32,
}

