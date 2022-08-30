#[doc = "Register `INIT_NEXT_INSTANT` reader"]
pub struct R(crate::R<INIT_NEXT_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_NEXT_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_NEXT_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_NEXT_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INIT_NEXT_INSTANT` reader - Shows the instant with respect to internal reference clock of resolution 625 us at which next initiator scanning event begins."]
pub type INIT_NEXT_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the instant with respect to internal reference clock of resolution 625 us at which next initiator scanning event begins."]
    #[inline(always)]
    pub fn init_next_instant(&self) -> INIT_NEXT_INSTANT_R {
        INIT_NEXT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Initiator next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_next_instant](index.html) module"]
pub struct INIT_NEXT_INSTANT_SPEC;
impl crate::RegisterSpec for INIT_NEXT_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_next_instant::R](R) reader structure"]
impl crate::Readable for INIT_NEXT_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INIT_NEXT_INSTANT to value 0"]
impl crate::Resettable for INIT_NEXT_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
