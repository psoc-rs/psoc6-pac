#[doc = "Register `INTR_USBHOST_CAUSE_MED` reader"]
pub struct R(crate::R<INTR_USBHOST_CAUSE_MED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_CAUSE_MED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_CAUSE_MED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_CAUSE_MED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOFIRQ_INT` reader - SOFIRQ interrupt"]
pub type SOFIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `DIRQ_INT` reader - DIRQ interrupt"]
pub type DIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `CNNIRQ_INT` reader - CNNIRQ interrupt"]
pub type CNNIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `CMPIRQ_INT` reader - CMPIRQ interrupt"]
pub type CMPIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `URIRQ_INT` reader - URIRQ interrupt"]
pub type URIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `RWKIRQ_INT` reader - RWKIRQ interrupt"]
pub type RWKIRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader<bool>;
#[doc = "Field `TCAN_INT` reader - TCAN interrupt"]
pub type TCAN_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SOFIRQ interrupt"]
    #[inline(always)]
    pub fn sofirq_int(&self) -> SOFIRQ_INT_R {
        SOFIRQ_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIRQ interrupt"]
    #[inline(always)]
    pub fn dirq_int(&self) -> DIRQ_INT_R {
        DIRQ_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNIRQ interrupt"]
    #[inline(always)]
    pub fn cnnirq_int(&self) -> CNNIRQ_INT_R {
        CNNIRQ_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMPIRQ interrupt"]
    #[inline(always)]
    pub fn cmpirq_int(&self) -> CMPIRQ_INT_R {
        CMPIRQ_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - URIRQ interrupt"]
    #[inline(always)]
    pub fn urirq_int(&self) -> URIRQ_INT_R {
        URIRQ_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWKIRQ interrupt"]
    #[inline(always)]
    pub fn rwkirq_int(&self) -> RWKIRQ_INT_R {
        RWKIRQ_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCAN interrupt"]
    #[inline(always)]
    pub fn tcan_int(&self) -> TCAN_INT_R {
        TCAN_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Cause Medium Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_cause_med](index.html) module"]
pub struct INTR_USBHOST_CAUSE_MED_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_CAUSE_MED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_cause_med::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_CAUSE_MED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_USBHOST_CAUSE_MED to value 0"]
impl crate::Resettable for INTR_USBHOST_CAUSE_MED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
