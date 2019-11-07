#[doc = "Reader of register ADV_RAND"]
pub type R = crate::R<u32, super::ADV_RAND>;
#[doc = "Reader of field `ADV_RAND`"]
pub type ADV_RAND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Random ADV delay, to be used for ADV next instant calculation. The granularity is in BT slot"]
    #[inline(always)]
    pub fn adv_rand(&self) -> ADV_RAND_R {
        ADV_RAND_R::new((self.bits & 0x0f) as u8)
    }
}
