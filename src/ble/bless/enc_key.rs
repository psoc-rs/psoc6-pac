#[doc = "Register `ENC_KEY[%s]` writer"]
pub struct W(crate::W<ENC_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_KEY_SPEC>;
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
impl From<crate::W<ENC_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC_KEY` writer - The encryption key / session key which is used in ECB encryption, CCM encryption and CCM decryption."]
pub type ENC_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENC_KEY_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - The encryption key / session key which is used in ECB encryption, CCM encryption and CCM decryption."]
    #[inline(always)]
    pub fn enc_key(&mut self) -> ENC_KEY_W<0> {
        ENC_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Key register 0-3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_key](index.html) module"]
pub struct ENC_KEY_SPEC;
impl crate::RegisterSpec for ENC_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enc_key::W](W) writer structure"]
impl crate::Writable for ENC_KEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_KEY[%s]
to value 0"]
impl crate::Resettable for ENC_KEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
