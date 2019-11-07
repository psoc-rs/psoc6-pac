#[doc = "Reader of register INTR_CAUSE"]
pub type R = crate::R<u32, super::INTR_CAUSE>;
#[doc = "Reader of field `COUNTER_INT`"]
pub type COUNTER_INT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_int(&self) -> COUNTER_INT_R {
        COUNTER_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
