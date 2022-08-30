#[doc = "Register `MASK0` reader"]
pub struct R(crate::R<MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK0` writer"]
pub struct W(crate::W<MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK0_SPEC>;
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
impl From<crate::W<MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
pub type SOURCE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 31 to 0."]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault mask 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](index.html) module"]
pub struct MASK0_SPEC;
impl crate::RegisterSpec for MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask0::R](R) reader structure"]
impl crate::Readable for MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask0::W](W) writer structure"]
impl crate::Writable for MASK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK0 to value 0"]
impl crate::Resettable for MASK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
