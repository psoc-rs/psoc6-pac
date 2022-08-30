#[doc = "Register `WAKEUP_CONFIG` reader"]
pub struct R(crate::R<WAKEUP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_CONFIG` writer"]
pub struct W(crate::W<WAKEUP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_CONFIG_SPEC>;
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
impl From<crate::W<WAKEUP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC_STARTUP_DELAY` reader - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\]
is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\]
sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
pub type OSC_STARTUP_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC_STARTUP_DELAY` writer - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\]
is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\]
sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
pub type OSC_STARTUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEUP_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DSM_OFFSET_TO_WAKEUP_INSTANT` reader - Number of 'slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The slot is of 0.625ms period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant."]
pub type DSM_OFFSET_TO_WAKEUP_INSTANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSM_OFFSET_TO_WAKEUP_INSTANT` writer - Number of 'slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The slot is of 0.625ms period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant."]
pub type DSM_OFFSET_TO_WAKEUP_INSTANT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEUP_CONFIG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\]
is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\]
sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
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
    #[doc = "Bits 0:7 - Oscillator stabilization/startup delay. This is in X.Y for-mat where X is in terms of number of BT slots (625 us) and Y is in terms of number of clock periods of 16KHz clock input, required for RF oscillator to stabilize the clock output to the controller on its output pin, after oscillator is turned ON. In this period the clock is as-sumed to be unstable, and so the controller does not turn on the clock to internal logic till this period is over. This means, the wake up from deep sleep mode must account for this delay before the wakeup instant. Osc_startup_delay\\[7:5\\]
is number of slots(625us) Osc_startup_delay\\[4:0 is number of clock periods of 16KHz clock (Warning: Min. value of Osc_startup_delay \\[4:0\\]
sup-ported is 1 and Max. value is 9. Therefore programma-ble range is 1 to 9)"]
    #[inline(always)]
    pub fn osc_startup_delay(&mut self) -> OSC_STARTUP_DELAY_W<0> {
        OSC_STARTUP_DELAY_W::new(self)
    }
    #[doc = "Bits 10:15 - Number of 'slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The slot is of 0.625ms period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant."]
    #[inline(always)]
    pub fn dsm_offset_to_wakeup_instant(&mut self) -> DSM_OFFSET_TO_WAKEUP_INSTANT_W<10> {
        DSM_OFFSET_TO_WAKEUP_INSTANT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_config](index.html) module"]
pub struct WAKEUP_CONFIG_SPEC;
impl crate::RegisterSpec for WAKEUP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_config::R](R) reader structure"]
impl crate::Readable for WAKEUP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_config::W](W) writer structure"]
impl crate::Writable for WAKEUP_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_CONFIG to value 0"]
impl crate::Resettable for WAKEUP_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
