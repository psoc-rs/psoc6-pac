#[doc = "Reader of register INTR_CAUSE_LO"]
pub type R = crate::R<u32, super::INTR_CAUSE_LO>;
#[doc = "Reader of field `SOF_INTR`"]
pub type SOF_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUS_RESET_INTR`"]
pub type BUS_RESET_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0_INTR`"]
pub type EP0_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPM_INTR`"]
pub type LPM_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME_INTR`"]
pub type RESUME_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARB_EP_INTR`"]
pub type ARB_EP_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1_INTR`"]
pub type EP1_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2_INTR`"]
pub type EP2_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3_INTR`"]
pub type EP3_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4_INTR`"]
pub type EP4_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5_INTR`"]
pub type EP5_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6_INTR`"]
pub type EP6_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7_INTR`"]
pub type EP7_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP8_INTR`"]
pub type EP8_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - USB SOF Interrupt"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SOF_INTR_R {
        SOF_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BUS RESET Interrupt"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BUS_RESET_INTR_R {
        BUS_RESET_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EP0 Interrupt"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> EP0_INTR_R {
        EP0_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPM Interrupt"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LPM_INTR_R {
        LPM_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Resume Interrupt"]
    #[inline(always)]
    pub fn resume_intr(&self) -> RESUME_INTR_R {
        RESUME_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn arb_ep_intr(&self) -> ARB_EP_INTR_R {
        ARB_EP_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EP1 Interrupt"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EP2 Interrupt"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EP3 Interrupt"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EP4 Interrupt"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP5 Interrupt"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP6 Interrupt"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP7 Interrupt"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP8 Interrupt"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
