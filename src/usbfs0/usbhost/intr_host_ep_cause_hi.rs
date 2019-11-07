#[doc = "Reader of register INTR_HOST_EP_CAUSE_HI"]
pub type R = crate::R<u32, super::INTR_HOST_EP_CAUSE_HI>;
#[doc = "Reader of field `EP1DRQ_INT`"]
pub type EP1DRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1SPK_INT`"]
pub type EP1SPK_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2DRQ_INT`"]
pub type EP2DRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2SPK_INT`"]
pub type EP2SPK_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - EP1DRQ interrupt"]
    #[inline(always)]
    pub fn ep1drq_int(&self) -> EP1DRQ_INT_R {
        EP1DRQ_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EP1SPK interrupt"]
    #[inline(always)]
    pub fn ep1spk_int(&self) -> EP1SPK_INT_R {
        EP1SPK_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EP2DRQ interrupt"]
    #[inline(always)]
    pub fn ep2drq_int(&self) -> EP2DRQ_INT_R {
        EP2DRQ_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EP2SPK interrupt"]
    #[inline(always)]
    pub fn ep2spk_int(&self) -> EP2SPK_INT_R {
        EP2SPK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
