#[doc = "Reader of register INTR_SIE_MASKED"]
pub type R = crate::R<u32, super::INTR_SIE_MASKED>;
#[doc = "Reader of field `SOF_INTR_MASKED`"]
pub type SOF_INTR_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUS_RESET_INTR_MASKED`"]
pub type BUS_RESET_INTR_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0_INTR_MASKED`"]
pub type EP0_INTR_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPM_INTR_MASKED`"]
pub type LPM_INTR_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME_INTR_MASKED`"]
pub type RESUME_INTR_MASKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sof_intr_masked(&self) -> SOF_INTR_MASKED_R {
        SOF_INTR_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn bus_reset_intr_masked(&self) -> BUS_RESET_INTR_MASKED_R {
        BUS_RESET_INTR_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ep0_intr_masked(&self) -> EP0_INTR_MASKED_R {
        EP0_INTR_MASKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn lpm_intr_masked(&self) -> LPM_INTR_MASKED_R {
        LPM_INTR_MASKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn resume_intr_masked(&self) -> RESUME_INTR_MASKED_R {
        RESUME_INTR_MASKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
