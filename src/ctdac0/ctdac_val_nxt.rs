#[doc = "Register `CTDAC_VAL_NXT` reader"]
pub struct R(crate::R<CTDAC_VAL_NXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTDAC_VAL_NXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTDAC_VAL_NXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTDAC_VAL_NXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTDAC_VAL_NXT` writer"]
pub struct W(crate::W<CTDAC_VAL_NXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTDAC_VAL_NXT_SPEC>;
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
impl From<crate::W<CTDAC_VAL_NXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTDAC_VAL_NXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Next value for CTDAC_VAL.VALUE"]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - Next value for CTDAC_VAL.VALUE"]
pub type VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTDAC_VAL_NXT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Next value for CTDAC_VAL.VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Next value for CTDAC_VAL.VALUE"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next DAC value (double buffering)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_val_nxt](index.html) module"]
pub struct CTDAC_VAL_NXT_SPEC;
impl crate::RegisterSpec for CTDAC_VAL_NXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctdac_val_nxt::R](R) reader structure"]
impl crate::Readable for CTDAC_VAL_NXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctdac_val_nxt::W](W) writer structure"]
impl crate::Writable for CTDAC_VAL_NXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTDAC_VAL_NXT to value 0"]
impl crate::Resettable for CTDAC_VAL_NXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
