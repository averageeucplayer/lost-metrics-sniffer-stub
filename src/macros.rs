use bincode::config;

use crate::packets::definitions::*;

static CONFIG: config::Configuration = config::standard();

macro_rules! impl_new_default {
    ($($struct_name:ident),*) => {
        $(
            impl $struct_name {
                pub fn new(data: &[u8]) -> anyhow::Result<Self>
                where
                    Self: bincode::de::Decode<()>,
                {
                    let (value, _): (Self, _) = bincode::decode_from_slice(data, CONFIG)?;
                    Ok(value)
                }

                pub fn encode(self) -> anyhow::Result<Vec<u8>>
                where
                    Self: bincode::Encode
                {
                    let value = bincode::encode_to_vec(self, CONFIG)?;
                    Ok(value)
                }
            }
        )*
    };
}

impl_new_default!(
    PKTNewVehicle,
    PKTSkillCooldownNotify,
    PKTPartyStatusEffectAddNotify,
    PKTPartyStatusEffectRemoveNotify,
    PKTZoneObjectUnpublishNotify,
    PKTPartyStatusEffectResultNotify,
    PKTStatusEffectAddNotify,
    PKTStatusEffectRemoveNotify,
    PKTTriggerStartNotify,
    PKTZoneMemberLoadStatusNotify,
    PKTStatusEffectSyncDataNotify,
    PKTTroopMemberUpdateMinNotify,
    PKTNewTransit,
    PKTNewPC,
    PKTPartyLeaveResult,
    PKTCounterAttackNotify,
    PKTDeathNotify,
    PKTIdentityGaugeChangeNotify,
    PKTInitEnv,
    PKTInitPC,
    PKTNewNpc,
    PKTNewNpcSummon,
    PKTNewProjectile,
    PKTSkillStartNotify,
    PKTSkillCastNotify,
    PKTRaidBegin,
    PKTNewTrap,
    PKTRemoveObject,
    PKTPartyInfo,
    PKTSkillDamageAbnormalMoveNotify,
    PKTSkillDamageNotify
);

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::{encode_to_vec, decode_from_slice};
    use bincode::config::Configuration;

    static CONFIG: Configuration = bincode::config::standard();
    
    macro_rules! test_serialization {
        ($($struct_name:ident),*) => {
            $(
                #[test]
                fn $struct_name() {
                    let original = $struct_name::default();
                    let encoded = encode_to_vec(&original, CONFIG).expect("Serialization failed");
                    let (decoded, _): ($struct_name, _) = 
                        decode_from_slice(&encoded, CONFIG).expect("Deserialization failed");
                }
            )*
        };
    }

    test_serialization!(
        PKTPartyStatusEffectAddNotify,
        PKTPartyStatusEffectRemoveNotify,
        PKTPartyStatusEffectResultNotify,
        PKTStatusEffectAddNotify,
        PKTStatusEffectRemoveNotify,
        PKTZoneObjectUnpublishNotify,
        PKTTriggerStartNotify,
        PKTZoneMemberLoadStatusNotify,
        PKTStatusEffectSyncDataNotify,
        PKTTroopMemberUpdateMinNotify,
        PKTNewTransit,
        PKTNewPC,
        PKTPartyLeaveResult,
        PKTCounterAttackNotify,
        PKTDeathNotify,
        PKTIdentityGaugeChangeNotify,
        PKTInitEnv,
        PKTInitPC,
        PKTNewNpc,
        PKTNewNpcSummon,
        PKTNewProjectile,
        PKTSkillStartNotify,
        PKTSkillCastNotify,
        PKTRaidBegin,
        PKTNewTrap,
        PKTRemoveObject,
        PKTPartyInfo,
        PKTSkillDamageAbnormalMoveNotify,
        PKTSkillDamageNotify
    );
}