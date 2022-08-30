#[doc = "Register `TICKS` reader"]
pub struct R(crate::R<TICKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT128HZ` reader - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
pub type CNT128HZ_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub fn cnt128hz(&self) -> CNT128HZ_R {
        CNT128HZ_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "128Hz tick counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticks](index.html) module"]
pub struct TICKS_SPEC;
impl crate::RegisterSpec for TICKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ticks::R](R) reader structure"]
impl crate::Readable for TICKS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TICKS to value 0"]
impl crate::Resettable for TICKS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
