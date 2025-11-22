use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize)]
pub struct DeployRequest {
    pub program_path : String,
    pub network : String
}