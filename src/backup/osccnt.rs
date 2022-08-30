#[doc = "Register `OSCCNT` reader"]
pub struct R(crate::R<OSCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT32KHZ` reader - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
pub type CNT32KHZ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub fn cnt32khz(&self) -> CNT32KHZ_R {
        CNT32KHZ_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "32kHz oscillator counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccnt](index.html) module"]
pub struct OSCCNT_SPEC;
impl crate::RegisterSpec for OSCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osccnt::R](R) reader structure"]
impl crate::Readable for OSCCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCCNT to value 0"]
impl crate::Resettable for OSCCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
