#[doc = "Register `TRIM` reader"]
pub struct R(crate::R<TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM` writer"]
pub struct W(crate::W<TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - WCO trim"]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - WCO trim"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](index.html) module"]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim::R](R) reader structure"]
impl crate::Readable for TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim::W](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
