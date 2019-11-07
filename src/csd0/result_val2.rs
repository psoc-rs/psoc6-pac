#[doc = "Reader of register RESULT_VAL2"]
pub type R = crate::R<u32, super::RESULT_VAL2>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
