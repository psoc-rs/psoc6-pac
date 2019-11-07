#[doc = "Reader of register INTR_USBHOST_CAUSE_HI"]
pub type R = crate::R<u32, super::INTR_USBHOST_CAUSE_HI>;
#[doc = "Reader of field `SOFIRQ_INT`"]
pub type SOFIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRQ_INT`"]
pub type DIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNNIRQ_INT`"]
pub type CNNIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPIRQ_INT`"]
pub type CMPIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `URIRQ_INT`"]
pub type URIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWKIRQ_INT`"]
pub type RWKIRQ_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD_6`"]
pub type RSVD_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCAN_INT`"]
pub type TCAN_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SOFIRQ interrupt"]
    #[inline(always)]
    pub fn sofirq_int(&self) -> SOFIRQ_INT_R {
        SOFIRQ_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIRQ interrupt"]
    #[inline(always)]
    pub fn dirq_int(&self) -> DIRQ_INT_R {
        DIRQ_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CNNIRQ interrupt"]
    #[inline(always)]
    pub fn cnnirq_int(&self) -> CNNIRQ_INT_R {
        CNNIRQ_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMPIRQ interrupt"]
    #[inline(always)]
    pub fn cmpirq_int(&self) -> CMPIRQ_INT_R {
        CMPIRQ_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - URIRQ interrupt"]
    #[inline(always)]
    pub fn urirq_int(&self) -> URIRQ_INT_R {
        URIRQ_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RWKIRQ interrupt"]
    #[inline(always)]
    pub fn rwkirq_int(&self) -> RWKIRQ_INT_R {
        RWKIRQ_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TCAN interrupt"]
    #[inline(always)]
    pub fn tcan_int(&self) -> TCAN_INT_R {
        TCAN_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
