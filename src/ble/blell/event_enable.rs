#[doc = "Reader of register EVENT_ENABLE"]
pub type R = crate::R<u32, super::EVENT_ENABLE>;
#[doc = "Writer for register EVENT_ENABLE"]
pub type W = crate::W<u32, super::EVENT_ENABLE>;
#[doc = "Register EVENT_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_INT_EN`"]
pub type ADV_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_INT_EN`"]
pub struct ADV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SCN_INT_EN`"]
pub type SCN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_INT_EN`"]
pub struct SCN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INIT_INT_EN`"]
pub type INIT_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_INT_EN`"]
pub struct INIT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CONN_INT_EN`"]
pub type CONN_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_INT_EN`"]
pub struct CONN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SM_INT_EN`"]
pub type SM_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM_INT_EN`"]
pub struct SM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_INT_EN_W<'a> {
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
#[doc = "Reader of field `DSM_INT_EN`"]
pub type DSM_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_INT_EN`"]
pub struct DSM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_INT_EN_W<'a> {
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
#[doc = "Reader of field `ENC_INT_EN`"]
pub type ENC_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENC_INT_EN`"]
pub struct ENC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RSSI_RX_DONE_INT_EN`"]
pub type RSSI_RX_DONE_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSSI_RX_DONE_INT_EN`"]
pub struct RSSI_RX_DONE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_RX_DONE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&self) -> ADV_INT_EN_R {
        ADV_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&self) -> SCN_INT_EN_R {
        SCN_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&self) -> INIT_INT_EN_R {
        INIT_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&self) -> CONN_INT_EN_R {
        CONN_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&self) -> SM_INT_EN_R {
        SM_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&self) -> DSM_INT_EN_R {
        DSM_INT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&self) -> ENC_INT_EN_R {
        ENC_INT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&self) -> RSSI_RX_DONE_INT_EN_R {
        RSSI_RX_DONE_INT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Advertiser interrupt enable. 1 - enable advertiser procedure to interrupt the firmware. 0 - disable advertiser procedure interrupt to firmware."]
    #[inline(always)]
    pub fn adv_int_en(&mut self) -> ADV_INT_EN_W {
        ADV_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Scanner interrupt enable. 1 - enable scan procedure to interrupt the firmware. 0 - disable scan procedure interrupt to firmware."]
    #[inline(always)]
    pub fn scn_int_en(&mut self) -> SCN_INT_EN_W {
        SCN_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Initiator interrupt enable. 1 - enable initiator procedure to interrupt the firmware. 0 - disable initiator procedure interrupt to firmware."]
    #[inline(always)]
    pub fn init_int_en(&mut self) -> INIT_INT_EN_W {
        INIT_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Connection interrupt enable. 1 - enable connection procedure to interrupt the firmware. 0 - disable connection procedure interrupt to firmware."]
    #[inline(always)]
    pub fn conn_int_en(&mut self) -> CONN_INT_EN_W {
        CONN_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Sleep-mode-exit interrupt enable. 1 - enable sleep mode exit event to interrupt the firmware. 0 - disable sleep mode exit interrupt to firmware. This interrupt is deprecated and should not be used."]
    #[inline(always)]
    pub fn sm_int_en(&mut self) -> SM_INT_EN_W {
        SM_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Deep Sleep-mode-exit interrupt enable. 1 - enable deep sleep mode exit event to interrupt the firmware. 0 - disable deep sleep mode exit interrupt to firmware."]
    #[inline(always)]
    pub fn dsm_int_en(&mut self) -> DSM_INT_EN_W {
        DSM_INT_EN_W { w: self }
    }
    #[doc = "Bit 6 - Encryption module interrupt enable. 1 - Enable encryption module interrupt to firmware. 0 - disable encryption module interrupt to firmware. This interrupt is deprecated and should not be used"]
    #[inline(always)]
    pub fn enc_int_en(&mut self) -> ENC_INT_EN_W {
        ENC_INT_EN_W { w: self }
    }
    #[doc = "Bit 7 - RSSI Rx interrupt enable. 1 - Enable RSSI Rx done interrupt to firmware. 0 - Disable RSSI Rx done interrupt to firmware."]
    #[inline(always)]
    pub fn rssi_rx_done_int_en(&mut self) -> RSSI_RX_DONE_INT_EN_W {
        RSSI_RX_DONE_INT_EN_W { w: self }
    }
}
