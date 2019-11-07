#[doc = "Reader of register US_OFFSET_STATUS"]
pub type R = crate::R<u32, super::US_OFFSET_STATUS>;
#[doc = "Reader of field `US_OFFSET`"]
pub type US_OFFSET_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - During slave connection event, HW updates this register with the calculated us_offset at anchor point, granularity is 1us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0x00D5."]
    #[inline(always)]
    pub fn us_offset(&self) -> US_OFFSET_R {
        US_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
