#[doc = "Reader of register ARB_INT_SR"]
pub type R = crate::R<u32, super::ARB_INT_SR>;
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
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
