#[doc = "Register `MIC_IN0` reader"]
pub struct R(crate::R<MIC_IN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIC_IN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIC_IN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIC_IN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIC_IN0` writer"]
pub struct W(crate::W<MIC_IN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIC_IN0_SPEC>;
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
impl From<crate::W<MIC_IN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIC_IN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIC_IN` reader - This is the MIC field used for CCM decryption."]
pub type MIC_IN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MIC_IN` writer - This is the MIC field used for CCM decryption."]
pub type MIC_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIC_IN0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the MIC field used for CCM decryption."]
    #[inline(always)]
    pub fn mic_in(&self) -> MIC_IN_R {
        MIC_IN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the MIC field used for CCM decryption."]
    #[inline(always)]
    pub fn mic_in(&mut self) -> MIC_IN_W<0> {
        MIC_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIC input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mic_in0](index.html) module"]
pub struct MIC_IN0_SPEC;
impl crate::RegisterSpec for MIC_IN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mic_in0::R](R) reader structure"]
impl crate::Readable for MIC_IN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mic_in0::W](W) writer structure"]
impl crate::Writable for MIC_IN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIC_IN0 to value 0"]
impl crate::Resettable for MIC_IN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
