use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GameMsg {
    pub success: bool,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]

pub struct InitResponse {
    pub msgs: Vec<GameMsg>,
    pub platform: String,
}
