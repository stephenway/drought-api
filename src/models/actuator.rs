use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActuatorCommand {
    pub device: String,
    pub state: String,
}

