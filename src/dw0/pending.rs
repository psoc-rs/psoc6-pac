#[doc = "Register `PENDING` reader"]
pub struct R(crate::R<PENDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_PENDING` reader - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
pub type CH_PENDING_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn ch_pending(&self) -> CH_PENDING_R {
        CH_PENDING_R::new(self.bits)
    }
}
#[doc = "Pending channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending](index.html) module"]
pub struct PENDING_SPEC;
impl crate::RegisterSpec for PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending::R](R) reader structure"]
impl crate::Readable for PENDING_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDING to value 0"]
impl crate::Resettable for PENDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
