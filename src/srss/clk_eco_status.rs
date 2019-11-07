#[doc = "Reader of register CLK_ECO_STATUS"]
pub type R = crate::R<u32, super::CLK_ECO_STATUS>;
#[doc = "Reader of field `ECO_OK`"]
pub type ECO_OK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECO_READY`"]
pub type ECO_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
    #[inline(always)]
    pub fn eco_ok(&self) -> ECO_OK_R {
        ECO_OK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the ECO internal oscillator circuit has fully stabilized."]
    #[inline(always)]
    pub fn eco_ready(&self) -> ECO_READY_R {
        ECO_READY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
