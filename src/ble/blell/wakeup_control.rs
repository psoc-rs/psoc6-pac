#[doc = "Reader of register WAKEUP_CONTROL"]
pub type R = crate::R<u32, super::WAKEUP_CONTROL>;
#[doc = "Writer for register WAKEUP_CONTROL"]
pub type W = crate::W<u32, super::WAKEUP_CONTROL>;
#[doc = "Register WAKEUP_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKEUP_INSTANT`"]
pub type WAKEUP_INSTANT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WAKEUP_INSTANT`"]
pub struct WAKEUP_INSTANT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_INSTANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
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
    pub fn wakeup_instant(&mut self) -> WAKEUP_INSTANT_W {
        WAKEUP_INSTANT_W { w: self }
    }
}
