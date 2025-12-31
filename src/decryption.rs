use std::sync::OnceLock;

use log::warn;

use crate::packets::structures::SkillDamageEvent;

static DAMAGE_ENCRYPTION_HANDLER_IMPL: OnceLock<Box<dyn DamageEncryptionHandlerTrait>> = OnceLock::new();

pub trait DamageEncryptionHandlerTrait: std::fmt::Debug + Send + Sync + 'static {
    fn start(&self) -> anyhow::Result<()>;
    fn decrypt_damage_event(&self, event: &mut SkillDamageEvent) -> bool;
    fn update_zone_instance_id(&self, channel_id: u32);
}

#[derive(Debug)]
struct NoOpDamageEncryptionHandler;

impl DamageEncryptionHandlerTrait for NoOpDamageEncryptionHandler {
    fn decrypt_damage_event(&self, _event: &mut SkillDamageEvent) -> bool {
        true
    }

    fn update_zone_instance_id(&self, _channel_id: u32) {
        // no-op
    }
    
    fn start(&self) -> anyhow::Result<()> {
        // no-op
        Ok(())
    }
}

pub fn set_damage_encryption_handler_impl<D: DamageEncryptionHandlerTrait>(handler: D) {
    DAMAGE_ENCRYPTION_HANDLER_IMPL
        .set(Box::new(handler))
        .expect("PacketCapture implementation already set");
}

fn damage_encryption_handler_impl() -> &'static dyn DamageEncryptionHandlerTrait {
    DAMAGE_ENCRYPTION_HANDLER_IMPL.get_or_init(|| {
        warn!(
            "DamageEncryptionHandler implementation not registered; \
             defaulting to no-op handler. \
             Register one via set_damage_encryption_handler_impl(...) before using."
        );
        Box::new(NoOpDamageEncryptionHandler)
    }).as_ref()
}

#[derive(Debug)]
pub struct DamageEncryptionHandler(&'static dyn DamageEncryptionHandlerTrait);

impl DamageEncryptionHandler {
    pub fn new() -> Self {
        Self(damage_encryption_handler_impl())
    }

    #[inline]
    pub fn start(self) -> anyhow::Result<Self> {
        self.0.start()?;
        Ok(self)
    }

    #[inline]
    pub fn decrypt_damage_event(&self, event: &mut SkillDamageEvent) -> bool {
        self.0.decrypt_damage_event(event)
    }

    #[inline]
    pub fn update_zone_instance_id(&self, channel_id: u32) {
        self.0.update_zone_instance_id(channel_id)
    }
}