use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DisplaySettingsType {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32
}