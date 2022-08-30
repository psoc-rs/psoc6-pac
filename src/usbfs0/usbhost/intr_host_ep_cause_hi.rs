#[doc = "Register `INTR_HOST_EP_CAUSE_HI` reader"]
pub struct R(crate::R<INTR_HOST_EP_CAUSE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_HOST_EP_CAUSE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_HOST_EP_CAUSE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_HOST_EP_CAUSE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EP1DRQ_INT` reader - EP1DRQ interrupt"]
pub type EP1DRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `EP1SPK_INT` reader - EP1SPK interrupt"]
pub type EP1SPK_INT_R = crate::BitReader<bool>;
#[doc = "Field `EP2DRQ_INT` reader - EP2DRQ interrupt"]
pub type EP2DRQ_INT_R = crate::BitReader<bool>;
#[doc = "Field `EP2SPK_INT` reader - EP2SPK interrupt"]
pub type EP2SPK_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - EP1DRQ interrupt"]
    #[inline(always)]
    pub fn ep1drq_int(&self) -> EP1DRQ_INT_R {
        EP1DRQ_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP1SPK interrupt"]
    #[inline(always)]
    pub fn ep1spk_int(&self) -> EP1SPK_INT_R {
        EP1SPK_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP2DRQ interrupt"]
    #[inline(always)]
    pub fn ep2drq_int(&self) -> EP2DRQ_INT_R {
        EP2DRQ_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP2SPK interrupt"]
    #[inline(always)]
    pub fn ep2spk_int(&self) -> EP2SPK_INT_R {
        EP2SPK_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_cause_hi](index.html) module"]
pub struct INTR_HOST_EP_CAUSE_HI_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_CAUSE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_host_ep_cause_hi::R](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_CAUSE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_HOST_EP_CAUSE_HI to value 0"]
impl crate::Resettable for INTR_HOST_EP_CAUSE_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
