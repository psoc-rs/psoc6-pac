#[doc = "Register `MASK1` reader"]
pub struct R(crate::R<MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK1` writer"]
pub struct W(crate::W<MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK1_SPEC>;
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
impl From<crate::W<MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SOURCE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASK1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
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
#[doc = "Fault mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask1](index.html) module"]
pub struct MASK1_SPEC;
impl crate::RegisterSpec for MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask1::R](R) reader structure"]
impl crate::Readable for MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask1::W](W) writer structure"]
impl crate::Writable for MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK1 to value 0"]
impl crate::Resettable for MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
