#[doc = "Register `VREF_TRIM2` reader"]
pub struct R(crate::R<VREF_TRIM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_TRIM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_TRIM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_TRIM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF_TRIM2` writer"]
pub struct W(crate::W<VREF_TRIM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_TRIM2_SPEC>;
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
impl From<crate::W<VREF_TRIM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_TRIM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREF_CURV_TRIM` reader - N/A"]
pub type VREF_CURV_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_CURV_TRIM` writer - N/A"]
pub type VREF_CURV_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VREF_TRIM2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&self) -> VREF_CURV_TRIM_R {
        VREF_CURV_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&mut self) -> VREF_CURV_TRIM_W<0> {
        VREF_CURV_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_trim2](index.html) module"]
pub struct VREF_TRIM2_SPEC;
impl crate::RegisterSpec for VREF_TRIM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref_trim2::R](R) reader structure"]
impl crate::Readable for VREF_TRIM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref_trim2::W](W) writer structure"]
impl crate::Writable for VREF_TRIM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF_TRIM2 to value 0"]
impl crate::Resettable for VREF_TRIM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
