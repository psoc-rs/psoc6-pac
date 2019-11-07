#[doc = "Reader of register EVENT_INTR"]
pub type R = crate::R<u32, super::EVENT_INTR>;
#[doc = "Writer for register EVENT_INTR"]
pub type W = crate::W<u32, super::EVENT_INTR>;
#[doc = "Register EVENT_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENT_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_INTR`"]
pub type ADV_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCAN_INTR`"]
pub type SCAN_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INIT_INTR`"]
pub type INIT_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONN_INTR`"]
pub type CONN_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SM_INTR`"]
pub type SM_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM_INTR`"]
pub struct SM_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DSM_INTR`"]
pub type DSM_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_INTR`"]
pub struct DSM_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_INTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ENC_INTR`"]
pub type ENC_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSSI_RX_DONE_INTR`"]
pub type RSSI_RX_DONE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Advertiser interrupt. If bit is set to 1, it indicates an event occurred in the advertising procedure. The source of the event needs to be read from the ADV_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the ADV_INTR register."]
    #[inline(always)]
    pub fn adv_intr(&self) -> ADV_INTR_R {
        ADV_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scanner interrupt. If bit is set to 1, it indicates an event occurred in the scanning procedure. The source of the event needs to be read from the SCAN_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the SCAN_INTR register."]
    #[inline(always)]
    pub fn scan_intr(&self) -> SCAN_INTR_R {
        SCAN_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initiator interrupt. If bit is set to 1, it indicates an event occurred in the initiating procedure. The source of the event needs to be read from the INIT_INTR register. This bit is cleared, when firmware clears ALL interrupts by writing to the INIT_INTR register."]
    #[inline(always)]
    pub fn init_intr(&self) -> INIT_INTR_R {
        INIT_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Connection interrupt. If bit is set to 1, it indicates an event occurred in the connection operation. This interrupt is aggregation of interrupts for all the connections. The source of the event for the specific connection, needs to be read from the CONN_INTR register specific to the connection. This bit is cleared, when firmware clears ALL interrupts by writing to the CONN_INTR register."]
    #[inline(always)]
    pub fn conn_intr(&self) -> CONN_INTR_R {
        CONN_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_intr(&self) -> SM_INTR_R {
        SM_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn dsm_intr(&self) -> DSM_INTR_R {
        DSM_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Encryption module interrupt. This interrupt id deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_intr(&self) -> ENC_INTR_R {
        ENC_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RSSI RX done interrupt."]
    #[inline(always)]
    pub fn rssi_rx_done_intr(&self) -> RSSI_RX_DONE_INTR_R {
        RSSI_RX_DONE_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Read: Sleep-mode-exit interrupt. This bit is set, when link layer hardware exits from sleep mode. Write: Clear sleep-mode-exit interrupt. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_intr(&mut self) -> SM_INTR_W {
        SM_INTR_W { w: self }
    }
    #[doc = "Bit 5 - Read: Deep sleep mode exit interrupt. This bit is set, when link layer hardware exits from deep sleep mode. Write: Clear deep sleep mode exit interrupt. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn dsm_intr(&mut self) -> DSM_INTR_W {
        DSM_INTR_W { w: self }
    }
}
