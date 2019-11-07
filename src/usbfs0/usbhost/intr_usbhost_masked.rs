#[doc = "Reader of register INTR_USBHOST_MASKED"]
pub type R = crate::R<u32, super::INTR_USBHOST_MASKED>;
#[doc = "Reader of field `SOFIRQED`"]
pub type SOFIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRQED`"]
pub type DIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNNIRQED`"]
pub type CNNIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPIRQED`"]
pub type CMPIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `URIRQED`"]
pub type URIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKIRQED`"]
pub type RWKIRQED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD_6`"]
pub type RSVD_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCANED`"]
pub type TCANED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates the interrupt by SOF flag. '0' : Doesn't request the interrupt by SOF '1' : Request the interrupt by SOF"]
    #[inline(always)]
    pub fn sofirqed(&self) -> SOFIRQED_R {
        SOFIRQED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates the interrupt by DIRQ flag. '0' : Doesn't request the interrupt by DIRQ '1' : Request the interrupt by DIRQ"]
    #[inline(always)]
    pub fn dirqed(&self) -> DIRQED_R {
        DIRQED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates the interrupt by CNNIRQ flag. '0' : Doesn't request the interrupt by CNNIRQ '1' : Request the interrupt by CNNIRQ"]
    #[inline(always)]
    pub fn cnnirqed(&self) -> CNNIRQED_R {
        CNNIRQED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by CMPIRQ flag. '0' : Doesn't request the interrupt by CMPIRQ '1' : Request the interrupt by CMPIRQ"]
    #[inline(always)]
    pub fn cmpirqed(&self) -> CMPIRQED_R {
        CMPIRQED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by URIRQ flag. '0' : Doesn't request the interrupt by URIRQ '1' : Request the interrupt by URIRQ"]
    #[inline(always)]
    pub fn urirqed(&self) -> URIRQED_R {
        URIRQED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by RWKIRQ flag. '0' : Doesn't request the interrupt by RWKIRQ '1' : Request the interrupt by RWKIRQ"]
    #[inline(always)]
    pub fn rwkirqed(&self) -> RWKIRQED_R {
        RWKIRQED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit indicates the interrupt by TCAN flag. '0' : Doesn't request the interrupt by TCAN '1' : Request the interrupt by TCAN"]
    #[inline(always)]
    pub fn tcaned(&self) -> TCANED_R {
        TCANED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
