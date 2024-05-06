use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GameMsg {
    pub success: bool,
    pub msg: String
}