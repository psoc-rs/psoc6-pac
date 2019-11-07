#[doc = "Reader of register STAT_CNTS"]
pub type R = crate::R<u32, super::STAT_CNTS>;
#[doc = "Reader of field `NUM_CONV`"]
pub type NUM_CONV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub fn num_conv(&self) -> NUM_CONV_R {
        NUM_CONV_R::new((self.bits & 0xffff) as u16)
    }
}
