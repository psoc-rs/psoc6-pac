#[doc = "Register `EVENT_INTR` reader"]
pub struct R(crate::R<EVENT_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENT_INTR` writer"]
pub struct W(crate::W<EVENT_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENT_INTR_SPEC>;
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
impl From<crate::W<EVENT_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENT_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_INTR` reader - Advertiser interrupt. If bit is set to 1, it indicates an event occurred in the advertising procedure. The source of the event needs to be read from the ADV_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the ADV_INTR register."]
pub type ADV_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_INTR` reader - Scanner interrupt. If bit is set to 1, it indicates an event occurred in the scanning procedure. The source of the event needs to be read from the SCAN_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the SCAN_INTR register."]
pub type SCAN_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INIT_INTR` reader - Initiator interrupt. If bit is set to 1, it indicates an event occurred in the initiating procedure. The source of the event needs to be read from the INIT_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the INIT_INTR register."]
pub type INIT_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CONN_INTR` reader - Connection interrupt. If bit is set to 1, it indicates an event occurred in the connection operation. This interrupt is aggregation of interrupts for all the connections. The source of the event for the specific connection, needs to be read from the CONN_INTR register specific to the connection. This bit is cleared, when firmware clears ALL interrupts by writing to the CONN_INTR register."]
pub type CONN_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SM_INTR` reader - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
pub type SM_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SM_INTR` writer - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
pub type SM_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_INTR_SPEC, bool, O>;
#[doc = "Field `DSM_INTR` reader - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
pub type DSM_INTR_R = crate::BitReader<bool>;
#[doc = "Field `DSM_INTR` writer - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
pub type DSM_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVENT_INTR_SPEC, bool, O>;
#[doc = "Field `ENC_INTR` reader - Encryption module interrupt. This interrupt id deprecated and should not be used"]
pub type ENC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `RSSI_RX_DONE_INTR` reader - RSSI RX done interrupt."]
pub type RSSI_RX_DONE_INTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Advertiser interrupt. If bit is set to 1, it indicates an event occurred in the advertising procedure. The source of the event needs to be read from the ADV_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the ADV_INTR register."]
    #[inline(always)]
    pub fn adv_intr(&self) -> ADV_INTR_R {
        ADV_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scanner interrupt. If bit is set to 1, it indicates an event occurred in the scanning procedure. The source of the event needs to be read from the SCAN_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the SCAN_INTR register."]
    #[inline(always)]
    pub fn scan_intr(&self) -> SCAN_INTR_R {
        SCAN_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Initiator interrupt. If bit is set to 1, it indicates an event occurred in the initiating procedure. The source of the event needs to be read from the INIT_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the INIT_INTR register."]
    #[inline(always)]
    pub fn init_intr(&self) -> INIT_INTR_R {
        INIT_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Connection interrupt. If bit is set to 1, it indicates an event occurred in the connection operation. This interrupt is aggregation of interrupts for all the connections. The source of the event for the specific connection, needs to be read from the CONN_INTR register specific to the connection. This bit is cleared, when firmware clears ALL interrupts by writing to the CONN_INTR register."]
    #[inline(always)]
    pub fn conn_intr(&self) -> CONN_INTR_R {
        CONN_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_intr(&self) -> SM_INTR_R {
        SM_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn dsm_intr(&self) -> DSM_INTR_R {
        DSM_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Encryption module interrupt. This interrupt id deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_intr(&self) -> ENC_INTR_R {
        ENC_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RSSI RX done interrupt."]
    #[inline(always)]
    pub fn rssi_rx_done_intr(&self) -> RSSI_RX_DONE_INTR_R {
        RSSI_RX_DONE_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_intr(&mut self) -> SM_INTR_W<4> {
        SM_INTR_W::new(self)
    }
    #[doc = "Bit 5 - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn dsm_intr(&mut self) -> DSM_INTR_W<5> {
        DSM_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event(Interrupt) status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_intr](index.html) module"]
pub struct EVENT_INTR_SPEC;
impl crate::RegisterSpec for EVENT_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event_intr::R](R) reader structure"]
impl crate::Readable for EVENT_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [event_intr::W](W) writer structure"]
impl crate::Writable for EVENT_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENT_INTR to value 0"]
impl crate::Resettable for EVENT_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
