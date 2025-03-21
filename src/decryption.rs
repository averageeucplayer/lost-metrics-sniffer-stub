use crate::packets::structures::SkillDamageEvent;

pub trait DamageEncryptionHandlerTrait {
    fn start(&self) -> anyhow::Result<()>;
    fn decrypt_damage_event(&self, event: &mut SkillDamageEvent) -> bool;
    fn update_zone_instance_id(&self, channel_id: u32);
}

pub struct DamageEncryptionHandler{}

impl DamageEncryptionHandlerTrait for DamageEncryptionHandler{
   
    fn start(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn decrypt_damage_event(&self, event: &mut SkillDamageEvent) -> bool {
        true
    }

    fn update_zone_instance_id(&self, channel_id: u32) {

    }
}

impl DamageEncryptionHandler {
    pub fn new() -> Self {
        Self {}
    }
}