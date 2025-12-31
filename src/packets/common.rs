use serde::{Deserialize, Serialize};

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillMoveOptionData {
    pub down_time: Option<f32>,
    pub stand_up_time: Option<f32>,
    pub move_time: Option<f32>,
}