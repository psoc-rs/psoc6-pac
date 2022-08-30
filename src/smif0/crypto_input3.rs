#[doc = "Register `CRYPTO_INPUT3` reader"]
pub struct R(crate::R<CRYPTO_INPUT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_INPUT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_INPUT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_INPUT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_INPUT3` writer"]
pub struct W(crate::W<CRYPTO_INPUT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_INPUT3_SPEC>;
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
impl From<crate::W<CRYPTO_INPUT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_INPUT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT` reader - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type INPUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPUT` writer - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type INPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYPTO_INPUT3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W<0> {
        INPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography input 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input3](index.html) module"]
pub struct CRYPTO_INPUT3_SPEC;
impl crate::RegisterSpec for CRYPTO_INPUT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_input3::R](R) reader structure"]
impl crate::Readable for CRYPTO_INPUT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_input3::W](W) writer structure"]
impl crate::Writable for CRYPTO_INPUT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPTO_INPUT3 to value 0"]
impl crate::Resettable for CRYPTO_INPUT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
