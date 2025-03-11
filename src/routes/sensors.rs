use actix_web::{post, get, web, HttpResponse, Responder};
use crate::models::sensor::SensorData;
use crate::services::sensor_service;

#[post("/sensor-data")]
async fn receive_sensor_data(data: web::Json<SensorData>) -> impl Responder {
    sensor_service::process_sensor_data(data.into_inner()).await;
    HttpResponse::Ok().json({ "message": "Sensor data updated" })
}

#[get("/sensor-data")]
async fn get_sensor_data() -> impl Responder {
    let data = sensor_service::fetch_latest_sensor_data().await;
    HttpResponse::Ok().json(data)
}

