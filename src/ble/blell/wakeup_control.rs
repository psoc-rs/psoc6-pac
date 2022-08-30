#[doc = "Register `WAKEUP_CONTROL` reader"]
pub struct R(crate::R<WAKEUP_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_CONTROL` writer"]
pub struct W(crate::W<WAKEUP_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_CONTROL_SPEC>;
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
impl From<crate::W<WAKEUP_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_INSTANT` reader - Instant, with reference to the internal 16-bit clock reference, at which the hardware must wakeup from deep sleep mode. This is calculated by firmware based on the next closest instant where a controller operation is required (like advertiser/scanner). Firmware reads the next instant of the procedures in the corresponding *_NEXT_INSTANT registers. This value is used only when hardware auto wakeup from deep sleep mode is enabled in the clock control register. Note: it is recommended to program wakeup_instant such a way that the actual instant to wakeup shall be at least two counts (two slots of 625 us) ahead of reference clock when entering DSM. The actual instant to wakeup is 'wakeup_instant - dsm_offset_to_wakeup_instant - osc_startup_delay, and it shall be greater than 'reference clock + 2'"]
pub type WAKEUP_INSTANT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAKEUP_INSTANT` writer - Instant, with reference to the internal 16-bit clock reference, at which the hardware must wakeup from deep sleep mode. This is calculated by firmware based on the next closest instant where a controller operation is required (like advertiser/scanner). Firmware reads the next instant of the procedures in the corresponding *_NEXT_INSTANT registers. This value is used only when hardware auto wakeup from deep sleep mode is enabled in the clock control register. Note: it is recommended to program wakeup_instant such a way that the actual instant to wakeup shall be at least two counts (two slots of 625 us) ahead of reference clock when entering DSM. The actual instant to wakeup is 'wakeup_instant - dsm_offset_to_wakeup_instant - osc_startup_delay, and it shall be greater than 'reference clock + 2'"]
pub type WAKEUP_INSTANT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEUP_CONTROL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Instant, with reference to the internal 16-bit clock reference, at which the hardware must wakeup from deep sleep mode. This is calculated by firmware based on the next closest instant where a controller operation is required (like advertiser/scanner). Firmware reads the next instant of the procedures in the corresponding *_NEXT_INSTANT registers. This value is used only when hardware auto wakeup from deep sleep mode is enabled in the clock control register. Note: it is recommended to program wakeup_instant such a way that the actual instant to wakeup shall be at least two counts (two slots of 625 us) ahead of reference clock when entering DSM. The actual instant to wakeup is 'wakeup_instant - dsm_offset_to_wakeup_instant - osc_startup_delay, and it shall be greater than 'reference clock + 2'"]
    #[inline(always)]
    pub fn wakeup_instant(&self) -> WAKEUP_INSTANT_R {
        WAKEUP_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Instant, with reference to the internal 16-bit clock reference, at which the hardware must wakeup from deep sleep mode. This is calculated by firmware based on the next closest instant where a controller operation is required (like advertiser/scanner). Firmware reads the next instant of the procedures in the corresponding *_NEXT_INSTANT registers. This value is used only when hardware auto wakeup from deep sleep mode is enabled in the clock control register. Note: it is recommended to program wakeup_instant such a way that the actual instant to wakeup shall be at least two counts (two slots of 625 us) ahead of reference clock when entering DSM. The actual instant to wakeup is 'wakeup_instant - dsm_offset_to_wakeup_instant - osc_startup_delay, and it shall be greater than 'reference clock + 2'"]
    #[inline(always)]
    pub fn wakeup_instant(&mut self) -> WAKEUP_INSTANT_W<0> {
        WAKEUP_INSTANT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_control](index.html) module"]
pub struct WAKEUP_CONTROL_SPEC;
impl crate::RegisterSpec for WAKEUP_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_control::R](R) reader structure"]
impl crate::Readable for WAKEUP_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_control::W](W) writer structure"]
impl crate::Writable for WAKEUP_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_CONTROL to value 0"]
impl crate::Resettable for WAKEUP_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
