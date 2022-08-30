#[doc = "Register `US_CAPT_STATUS` reader"]
pub struct R(crate::R<US_CAPT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_CAPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_CAPT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_CAPT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `US_CAPT` reader - During slave connection event, HW updates this register with the captured microsecond at anchor point, granularity is 1us"]
pub type US_CAPT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - During slave connection event, HW updates this register with the captured microsecond at anchor point, granularity is 1us"]
    #[inline(always)]
    pub fn us_capt(&self) -> US_CAPT_R {
        US_CAPT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Micro-second Capture Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_capt_status](index.html) module"]
pub struct US_CAPT_STATUS_SPEC;
impl crate::RegisterSpec for US_CAPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_capt_status::R](R) reader structure"]
impl crate::Readable for US_CAPT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_CAPT_STATUS to value 0"]
impl crate::Resettable for US_CAPT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
