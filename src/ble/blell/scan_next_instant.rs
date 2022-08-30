#[doc = "Register `SCAN_NEXT_INSTANT` reader"]
pub struct R(crate::R<SCAN_NEXT_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_NEXT_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_NEXT_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_NEXT_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEXT_SCAN_INSTANT` reader - Shows the instant with respect to internal reference clock of resolution 625 us at which next scanning event begins."]
pub type NEXT_SCAN_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the instant with respect to internal reference clock of resolution 625 us at which next scanning event begins."]
    #[inline(always)]
    pub fn next_scan_instant(&self) -> NEXT_SCAN_INSTANT_R {
        NEXT_SCAN_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Advertising next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_next_instant](index.html) module"]
pub struct SCAN_NEXT_INSTANT_SPEC;
impl crate::RegisterSpec for SCAN_NEXT_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_next_instant::R](R) reader structure"]
impl crate::Readable for SCAN_NEXT_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCAN_NEXT_INSTANT to value 0"]
impl crate::Resettable for SCAN_NEXT_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
