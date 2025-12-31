use crate::types::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct StatusEffectDataValue {
    pub bytearray_0: Option<Vec<u8>>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillCooldownStructInner {
    pub has_stacks: u32,
    pub current_stack_cooldown: Option<f64>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillCooldownStruct {
    pub skill_cooldown_stack_data: SkillCooldownStructInner,
    pub current_cooldown: f64,
    pub skill_id: u32

}


#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct StatusEffectData {
    
    pub source_id: EntityId,
    pub status_effect_id: StatusEffectId,
    pub status_effect_instance_id: StatusEffectInstanceId,
    pub value: StatusEffectDataValue,
    pub total_time: f32,
    pub stack_count: u8,
    pub end_tick: u64
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct EquipItemData {

}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct PCStruct {
    pub player_id: EntityId,
    pub name: String,
    pub character_id: CharacterId,
    pub class_id: ClassId,
    pub gear_level: GearLevel,
    pub stat_pairs: Vec<StatPair>,
    pub max_item_level: GearLevel,
    pub status_effect_datas: Vec<StatusEffectData>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct NpcStructBalance {
    pub value: Option<u16>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct NpcStruct {
    pub object_id: EntityId,
    pub type_id: NpcId,
    pub level: u16,
    pub balance_level: NpcStructBalance,
    pub stat_pairs: Vec<StatPair>,
    pub status_effect_datas: Vec<StatusEffectData>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct StatPair {
    pub stat_type: u8,
    pub value: i64
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillDamageEventInner {
    pub p64_0: Option<i64>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillDamageEventShield {
    pub p64_0: Option<i64>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillDamageEventRdps {
    pub rdps_data: Option<SkillDamageEventRdpsInner>
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillDamageEventRdpsInner {
    pub event_type: Vec<u8>,
    pub value: Vec<i64>,
    pub source_character_id: Vec<u64>,
    pub skill_id: Vec<u32>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct SkillDamageEvent {
    pub target_id: u64,
    pub damage: i64,
    pub modifier: i32,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub damage_attr: Option<u8>,
    pub damage_type: u8,
    pub shield_damage: SkillDamageEventShield,
    pub rdps_data_conditional: SkillDamageEventRdps,
    pub stagger_amount: u32
}