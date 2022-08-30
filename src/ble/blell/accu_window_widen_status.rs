#[doc = "Register `ACCU_WINDOW_WIDEN_STATUS` reader"]
pub struct R(crate::R<ACCU_WINDOW_WIDEN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCU_WINDOW_WIDEN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCU_WINDOW_WIDEN_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCU_WINDOW_WIDEN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCU_WINDOW_WIDEN` reader - Accumulated Window Widen Value. HW updates this register at the close of slave connection event"]
pub type ACCU_WINDOW_WIDEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Accumulated Window Widen Value. HW updates this register at the close of slave connection event"]
    #[inline(always)]
    pub fn accu_window_widen(&self) -> ACCU_WINDOW_WIDEN_R {
        ACCU_WINDOW_WIDEN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Accumulated Window Widen Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accu_window_widen_status](index.html) module"]
pub struct ACCU_WINDOW_WIDEN_STATUS_SPEC;
impl crate::RegisterSpec for ACCU_WINDOW_WIDEN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accu_window_widen_status::R](R) reader structure"]
impl crate::Readable for ACCU_WINDOW_WIDEN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCU_WINDOW_WIDEN_STATUS to value 0"]
impl crate::Resettable for ACCU_WINDOW_WIDEN_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
