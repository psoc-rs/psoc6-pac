#[doc = "Register `RANGE_INTR` reader"]
pub struct R(crate::R<RANGE_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANGE_INTR` writer"]
pub struct W(crate::W<RANGE_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANGE_INTR_SPEC>;
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
impl From<crate::W<RANGE_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANGE_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANGE_INTR` reader - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RANGE_INTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_INTR` writer - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RANGE_INTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RANGE_INTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&self) -> RANGE_INTR_R {
        RANGE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&mut self) -> RANGE_INTR_W<0> {
        RANGE_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Range detect interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr](index.html) module"]
pub struct RANGE_INTR_SPEC;
impl crate::RegisterSpec for RANGE_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_intr::R](R) reader structure"]
impl crate::Readable for RANGE_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [range_intr::W](W) writer structure"]
impl crate::Writable for RANGE_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RANGE_INTR to value 0"]
impl crate::Resettable for RANGE_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
