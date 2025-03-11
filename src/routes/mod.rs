use actix_web::web;

mod sensors;
mod actuators;
mod websocket;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(sensors::receive_sensor_data);
    cfg.service(sensors::get_sensor_data);
    cfg.service(actuators::actuate_device);
    cfg.service(websocket::ws_endpoint);
}

