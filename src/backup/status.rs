#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_BUSY` reader - pending RTC write"]
pub type RTC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `WCO_OK` reader - Indicates that output has transitioned."]
pub type WCO_OK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - pending RTC write"]
    #[inline(always)]
    pub fn rtc_busy(&self) -> RTC_BUSY_R {
        RTC_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that output has transitioned."]
    #[inline(always)]
    pub fn wco_ok(&self) -> WCO_OK_R {
        WCO_OK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
