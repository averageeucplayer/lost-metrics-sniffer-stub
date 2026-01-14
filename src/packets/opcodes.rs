#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Pkt {
    BattleItemUseNotify,
    CounterAttackNotify,
    DeathNotify,
    IdentityGaugeChangeNotify,
    InitEnv,
    InitPC,
    NewPC,
    NewNpc,
    NewVehicle,
    NewNpcSummon,
    NewProjectile,
    NewTrap,
    RaidBegin,
    RaidBossKillNotify,
    RaidResult,
    RemoveObject,
    SkillCastNotify,
    SkillStartNotify,
    SkillCooldownNotify,
    SkillDamageAbnormalMoveNotify,
    SkillDamageNotify,
    PartyInfo,
    PartyLeaveResult,
    PartyStatusEffectAddNotify,
    PartyStatusEffectRemoveNotify,
    PartyStatusEffectResultNotify,
    StatusEffectAddNotify,
    StatusEffectRemoveNotify,
    TriggerBossBattleStatus,
    TriggerStartNotify,
    ZoneMemberLoadStatusNotify,
    ZoneObjectUnpublishNotify,
    StatusEffectSyncDataNotify,
    TroopMemberUpdateMinNotify,
    NewTransit
}