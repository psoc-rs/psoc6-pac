#[doc = "Reader of register NEXT_SUP_TO_STATUS"]
pub type R = crate::R<u32, super::NEXT_SUP_TO_STATUS>;
#[doc = "Reader of field `NEXT_SUP_TO`"]
pub type NEXT_SUP_TO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW updates this register for the SuperVision timeout next instant, granularity is 625us"]
    #[inline(always)]
    pub fn next_sup_to(&self) -> NEXT_SUP_TO_R {
        NEXT_SUP_TO_R::new((self.bits & 0xffff) as u16)
    }
}
