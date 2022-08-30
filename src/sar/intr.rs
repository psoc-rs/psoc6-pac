#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOS_INTR` reader - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EOS_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EOS_INTR` writer - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EOS_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `OVERFLOW_INTR` reader - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OVERFLOW_INTR_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_INTR` writer - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OVERFLOW_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `FW_COLLISION_INTR` reader - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FW_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_INTR` writer - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FW_COLLISION_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `DSI_COLLISION_INTR` reader - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DSI_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_INTR` writer - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DSI_COLLISION_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INJ_EOC_INTR` reader - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type INJ_EOC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_INTR` writer - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type INJ_EOC_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INJ_SATURATE_INTR` reader - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type INJ_SATURATE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_INTR` writer - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type INJ_SATURATE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INJ_RANGE_INTR` reader - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type INJ_RANGE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_INTR` writer - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type INJ_RANGE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `INJ_COLLISION_INTR` reader - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type INJ_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_INTR` writer - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type INJ_COLLISION_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn eos_intr(&self) -> EOS_INTR_R {
        EOS_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn overflow_intr(&self) -> OVERFLOW_INTR_R {
        OVERFLOW_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn fw_collision_intr(&self) -> FW_COLLISION_INTR_R {
        FW_COLLISION_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn dsi_collision_intr(&self) -> DSI_COLLISION_INTR_R {
        DSI_COLLISION_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_eoc_intr(&self) -> INJ_EOC_INTR_R {
        INJ_EOC_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_saturate_intr(&self) -> INJ_SATURATE_INTR_R {
        INJ_SATURATE_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_range_intr(&self) -> INJ_RANGE_INTR_R {
        INJ_RANGE_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_collision_intr(&self) -> INJ_COLLISION_INTR_R {
        INJ_COLLISION_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn eos_intr(&mut self) -> EOS_INTR_W<0> {
        EOS_INTR_W::new(self)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn overflow_intr(&mut self) -> OVERFLOW_INTR_W<1> {
        OVERFLOW_INTR_W::new(self)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn fw_collision_intr(&mut self) -> FW_COLLISION_INTR_W<2> {
        FW_COLLISION_INTR_W::new(self)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn dsi_collision_intr(&mut self) -> DSI_COLLISION_INTR_W<3> {
        DSI_COLLISION_INTR_W::new(self)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_eoc_intr(&mut self) -> INJ_EOC_INTR_W<4> {
        INJ_EOC_INTR_W::new(self)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_saturate_intr(&mut self) -> INJ_SATURATE_INTR_W<5> {
        INJ_SATURATE_INTR_W::new(self)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_range_intr(&mut self) -> INJ_RANGE_INTR_W<6> {
        INJ_RANGE_INTR_W::new(self)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_collision_intr(&mut self) -> INJ_COLLISION_INTR_W<7> {
        INJ_COLLISION_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
