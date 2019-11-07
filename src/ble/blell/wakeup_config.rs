#[doc = "Reader of register WAKEUP_CONFIG"]
pub type R = crate::R<u32, super::WAKEUP_CONFIG>;
#[doc = "Writer for register WAKEUP_CONFIG"]
pub type W = crate::W<u32, super::WAKEUP_CONFIG>;
#[doc = "Register WAKEUP_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSC_STARTUP_DELAY`"]
pub type OSC_STARTUP_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSC_STARTUP_DELAY`"]
pub struct OSC_STARTUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_STARTUP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DSM_OFFSET_TO_WAKEUP_INSTANT`"]
pub type DSM_OFFSET_TO_WAKEUP_INSTANT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSM_OFFSET_TO_WAKEUP_INSTANT`"]
pub struct DSM_OFFSET_TO_WAKEUP_INSTANT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_OFFSET_TO_WAKEUP_INSTANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\] is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\] sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
    #[inline(always)]
    pub fn osc_startup_delay(&self) -> OSC_STARTUP_DELAY_R {
        OSC_STARTUP_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - Number of 'slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The slot is of 0.625ms period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant(&self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_R {
        DSM_OFFSET_TO_WAKEUP_INSTANT_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\] is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\] sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
    #[inline(always)]
    pub fn osc_startup_delay(&mut self) -> OSC_STARTUP_DELAY_W {
        OSC_STARTUP_DELAY_W { w: self }
    }
    #[doc = "Bits 10:15 - Number of 'slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The slot is of 0.625ms period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant(&mut self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_W {
        DSM_OFFSET_TO_WAKEUP_INSTANT_W { w: self }
    }
}
