#[doc = "Register `EVENT_ENABLE` reader"]
pub struct R(crate::R<EVENT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENT_ENABLE` writer"]
pub struct W(crate::W<EVENT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_ENABLE_SPEC>;
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
impl From<crate::W<EVENT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_INT_EN` reader - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
pub type ADV_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_INT_EN` writer - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
pub type ADV_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `SCN_INT_EN` reader - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
pub type SCN_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_INT_EN` writer - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
pub type SCN_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `INIT_INT_EN` reader - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
pub type INIT_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INIT_INT_EN` writer - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
pub type INIT_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `CONN_INT_EN` reader - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
pub type CONN_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONN_INT_EN` writer - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
pub type CONN_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `SM_INT_EN` reader - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
pub type SM_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SM_INT_EN` writer - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
pub type SM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `DSM_INT_EN` reader - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
pub type DSM_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSM_INT_EN` writer - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
pub type DSM_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ENC_INT_EN` reader - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
pub type ENC_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ENC_INT_EN` writer - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
pub type ENC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
#[doc = "Field `RSSI_RX_DONE_INT_EN` reader - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
pub type RSSI_RX_DONE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RSSI_RX_DONE_INT_EN` writer - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
pub type RSSI_RX_DONE_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENT_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&self) -> ADV_INT_EN_R {
        ADV_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&self) -> SCN_INT_EN_R {
        SCN_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&self) -> INIT_INT_EN_R {
        INIT_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&self) -> CONN_INT_EN_R {
        CONN_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&self) -> SM_INT_EN_R {
        SM_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&self) -> DSM_INT_EN_R {
        DSM_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&self) -> ENC_INT_EN_R {
        ENC_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&self) -> RSSI_RX_DONE_INT_EN_R {
        RSSI_RX_DONE_INT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&mut self) -> ADV_INT_EN_W<0> {
        ADV_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&mut self) -> SCN_INT_EN_W<1> {
        SCN_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&mut self) -> INIT_INT_EN_W<2> {
        INIT_INT_EN_W::new(self)
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&mut self) -> CONN_INT_EN_W<3> {
        CONN_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&mut self) -> SM_INT_EN_W<4> {
        SM_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&mut self) -> DSM_INT_EN_W<5> {
        DSM_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&mut self) -> ENC_INT_EN_W<6> {
        ENC_INT_EN_W::new(self)
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&mut self) -> RSSI_RX_DONE_INT_EN_W<7> {
        RSSI_RX_DONE_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event indications enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_enable](index.html) module"]
pub struct EVENT_ENABLE_SPEC;
impl crate::RegisterSpec for EVENT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event_enable::R](R) reader structure"]
impl crate::Readable for EVENT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [event_enable::W](W) writer structure"]
impl crate::Writable for EVENT_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENT_ENABLE to value 0"]
impl crate::Resettable for EVENT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
