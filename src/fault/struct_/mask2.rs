#[doc = "Register `MASK2` reader"]
pub struct R(crate::R<MASK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK2` writer"]
pub struct W(crate::W<MASK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK2_SPEC>;
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
impl From<crate::W<MASK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SOURCE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
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
#[doc = "Fault mask 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask2](index.html) module"]
pub struct MASK2_SPEC;
impl crate::RegisterSpec for MASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask2::R](R) reader structure"]
impl crate::Readable for MASK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask2::W](W) writer structure"]
impl crate::Writable for MASK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK2 to value 0"]
impl crate::Resettable for MASK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
