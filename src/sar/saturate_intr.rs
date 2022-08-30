#[doc = "Register `SATURATE_INTR` reader"]
pub struct R(crate::R<SATURATE_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SATURATE_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SATURATE_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SATURATE_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SATURATE_INTR` writer"]
pub struct W(crate::W<SATURATE_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SATURATE_INTR_SPEC>;
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
impl From<crate::W<SATURATE_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SATURATE_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SATURATE_INTR` reader - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SATURATE_INTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SATURATE_INTR` writer - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SATURATE_INTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SATURATE_INTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&self) -> SATURATE_INTR_R {
        SATURATE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&mut self) -> SATURATE_INTR_W<0> {
        SATURATE_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Saturate interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr](index.html) module"]
pub struct SATURATE_INTR_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saturate_intr::R](R) reader structure"]
impl crate::Readable for SATURATE_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saturate_intr::W](W) writer structure"]
impl crate::Writable for SATURATE_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SATURATE_INTR to value 0"]
impl crate::Resettable for SATURATE_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
