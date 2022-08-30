#[doc = "Register `ADV_NEXT_INSTANT` reader"]
pub struct R(crate::R<ADV_NEXT_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_NEXT_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_NEXT_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_NEXT_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADV_NEXT_INSTANT` reader - Shows the next start of advertising event with reference to the internal reference clock."]
pub type ADV_NEXT_INSTANT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shows the next start of advertising event with reference to the internal reference clock."]
    #[inline(always)]
    pub fn adv_next_instant(&self) -> ADV_NEXT_INSTANT_R {
        ADV_NEXT_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Advertising next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_next_instant](index.html) module"]
pub struct ADV_NEXT_INSTANT_SPEC;
impl crate::RegisterSpec for ADV_NEXT_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_next_instant::R](R) reader structure"]
impl crate::Readable for ADV_NEXT_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADV_NEXT_INSTANT to value 0"]
impl crate::Resettable for ADV_NEXT_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
