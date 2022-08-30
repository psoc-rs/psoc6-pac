#[doc = "Register `TRIM_MXD[%s]` reader"]
pub struct R(crate::R<TRIM_MXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_MXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_MXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_MXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_MXD[%s]` writer"]
pub struct W(crate::W<TRIM_MXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_MXD_SPEC>;
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
impl From<crate::W<TRIM_MXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_MXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXD_TRIM_BITS` reader - MXD trim bits"]
pub type MXD_TRIM_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MXD_TRIM_BITS` writer - MXD trim bits"]
pub type MXD_TRIM_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_MXD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MXD trim bits"]
    #[inline(always)]
    pub fn mxd_trim_bits(&self) -> MXD_TRIM_BITS_R {
        MXD_TRIM_BITS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MXD trim bits"]
    #[inline(always)]
    pub fn mxd_trim_bits(&mut self) -> MXD_TRIM_BITS_W<0> {
        MXD_TRIM_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MXD die Trim registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_mxd](index.html) module"]
pub struct TRIM_MXD_SPEC;
impl crate::RegisterSpec for TRIM_MXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_mxd::R](R) reader structure"]
impl crate::Readable for TRIM_MXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_mxd::W](W) writer structure"]
impl crate::Writable for TRIM_MXD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_MXD[%s]
to value 0"]
impl crate::Resettable for TRIM_MXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
