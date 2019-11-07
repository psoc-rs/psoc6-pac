#[doc = "Reader of register INTR_HOST_EP_MASKED"]
pub type R = crate::R<u32, super::INTR_HOST_EP_MASKED>;
#[doc = "Reader of field `EP1DRQED`"]
pub type EP1DRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1SPKED`"]
pub type EP1SPKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2DRQED`"]
pub type EP2DRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2SPKED`"]
pub type EP2SPKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
    #[inline(always)]
    pub fn ep1drqed(&self) -> EP1DRQED_R {
        EP1DRQED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
    #[inline(always)]
    pub fn ep1spked(&self) -> EP1SPKED_R {
        EP1SPKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
    #[inline(always)]
    pub fn ep2drqed(&self) -> EP2DRQED_R {
        EP2DRQED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
    #[inline(always)]
    pub fn ep2spked(&self) -> EP2SPKED_R {
        EP2SPKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
