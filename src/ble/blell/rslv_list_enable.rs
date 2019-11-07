#[doc = "Reader of register RSLV_LIST_ENABLE[%s]"]
pub type R = crate::R<u32, super::RSLV_LIST_ENABLE>;
#[doc = "Writer for register RSLV_LIST_ENABLE[%s]"]
pub type W = crate::W<u32, super::RSLV_LIST_ENABLE>;
#[doc = "Register RSLV_LIST_ENABLE[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::RSLV_LIST_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALID_ENTRY`"]
pub type VALID_ENTRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID_ENTRY`"]
pub struct VALID_ENTRY_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_ENTRY_W<'a> {
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
#[doc = "Reader of field `PEER_ADDR_IRK_SET`"]
pub type PEER_ADDR_IRK_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEER_ADDR_IRK_SET`"]
pub struct PEER_ADDR_IRK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_IRK_SET_W<'a> {
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
#[doc = "Reader of field `SELF_ADDR_IRK_SET_RX`"]
pub type SELF_ADDR_IRK_SET_RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_IRK_SET_RX`"]
pub struct SELF_ADDR_IRK_SET_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_IRK_SET_RX_W<'a> {
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
#[doc = "Reader of field `WHITELISTED_PEER`"]
pub type WHITELISTED_PEER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WHITELISTED_PEER`"]
pub struct WHITELISTED_PEER_W<'a> {
    w: &'a mut W,
}
impl<'a> WHITELISTED_PEER_W<'a> {
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
#[doc = "Reader of field `PEER_ADDR_TYPE`"]
pub type PEER_ADDR_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEER_ADDR_TYPE`"]
pub struct PEER_ADDR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_TYPE_W<'a> {
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
#[doc = "Reader of field `PEER_ADDR_RPA_VAL`"]
pub type PEER_ADDR_RPA_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEER_ADDR_RPA_VAL`"]
pub struct PEER_ADDR_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_RPA_VAL_W<'a> {
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
#[doc = "Reader of field `SELF_ADDR_RXD_RPA_VAL`"]
pub type SELF_ADDR_RXD_RPA_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_RXD_RPA_VAL`"]
pub struct SELF_ADDR_RXD_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_RXD_RPA_VAL_W<'a> {
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
#[doc = "Reader of field `SELF_ADDR_TX_RPA_VAL`"]
pub type SELF_ADDR_TX_RPA_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_TX_RPA_VAL`"]
pub struct SELF_ADDR_TX_RPA_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_TX_RPA_VAL_W<'a> {
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
#[doc = "Reader of field `SELF_ADDR_INIT_RPA_SEL`"]
pub type SELF_ADDR_INIT_RPA_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_INIT_RPA_SEL`"]
pub struct SELF_ADDR_INIT_RPA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_INIT_RPA_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SELF_ADDR_TYPE_TX`"]
pub type SELF_ADDR_TYPE_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_TYPE_TX`"]
pub struct SELF_ADDR_TYPE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_TYPE_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ENTRY_CONNECTED`"]
pub type ENTRY_CONNECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTRY_CONNECTED`"]
pub struct ENTRY_CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRY_CONNECTED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the index is valid"]
    #[inline(always)]
    pub fn valid_entry(&self) -> VALID_ENTRY_R {
        VALID_ENTRY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn peer_addr_irk_set(&self) -> PEER_ADDR_IRK_SET_R {
        PEER_ADDR_IRK_SET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn self_addr_irk_set_rx(&self) -> SELF_ADDR_IRK_SET_RX_R {
        SELF_ADDR_IRK_SET_RX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates if the listed peer device is in the whitelist"]
    #[inline(always)]
    pub fn whitelisted_peer(&self) -> WHITELISTED_PEER_R {
        WHITELISTED_PEER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates the address type of the listed peer device"]
    #[inline(always)]
    pub fn peer_addr_type(&self) -> PEER_ADDR_TYPE_R {
        PEER_ADDR_TYPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that the peer device RPA in the list is valid"]
    #[inline(always)]
    pub fn peer_addr_rpa_val(&self) -> PEER_ADDR_RPA_VAL_R {
        PEER_ADDR_RPA_VAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that the received self RPA in the list is valid"]
    #[inline(always)]
    pub fn self_addr_rxd_rpa_val(&self) -> SELF_ADDR_RXD_RPA_VAL_R {
        SELF_ADDR_RXD_RPA_VAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that the self RPA in the list to be transmitted is valid"]
    #[inline(always)]
    pub fn self_addr_tx_rpa_val(&self) -> SELF_ADDR_TX_RPA_VAL_R {
        SELF_ADDR_TX_RPA_VAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
    #[inline(always)]
    pub fn self_addr_init_rpa_sel(&self) -> SELF_ADDR_INIT_RPA_SEL_R {
        SELF_ADDR_INIT_RPA_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
    #[inline(always)]
    pub fn self_addr_type_tx(&self) -> SELF_ADDR_TYPE_TX_R {
        SELF_ADDR_TYPE_TX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates if the entry is already in connection with our device"]
    #[inline(always)]
    pub fn entry_connected(&self) -> ENTRY_CONNECTED_R {
        ENTRY_CONNECTED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the index is valid"]
    #[inline(always)]
    pub fn valid_entry(&mut self) -> VALID_ENTRY_W {
        VALID_ENTRY_W { w: self }
    }
    #[doc = "Bit 1 - Indicates if the listed peer device has shared its IRK. 0 - Identity address in a received packet is accepted. If a valid peer device RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the peer device RPA, if available in the list, in a received packet is accepted. An Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn peer_addr_irk_set(&mut self) -> PEER_ADDR_IRK_SET_W {
        PEER_ADDR_IRK_SET_W { w: self }
    }
    #[doc = "Bit 2 - Indicates if the local IRK has been shared with the listed peer device 0 - Self Identity address in a received packet is accepted. If a valid self RPA is available in the list, then the RPA in a received packet is accepted. 1 - Only the self device RPA, if available in the list, in a received packet is accepted. A Self Identity address in the received packet is reported as a privacy mismatch."]
    #[inline(always)]
    pub fn self_addr_irk_set_rx(&mut self) -> SELF_ADDR_IRK_SET_RX_W {
        SELF_ADDR_IRK_SET_RX_W { w: self }
    }
    #[doc = "Bit 3 - Indicates if the listed peer device is in the whitelist"]
    #[inline(always)]
    pub fn whitelisted_peer(&mut self) -> WHITELISTED_PEER_W {
        WHITELISTED_PEER_W { w: self }
    }
    #[doc = "Bit 4 - Indicates the address type of the listed peer device"]
    #[inline(always)]
    pub fn peer_addr_type(&mut self) -> PEER_ADDR_TYPE_W {
        PEER_ADDR_TYPE_W { w: self }
    }
    #[doc = "Bit 5 - Indicates that the peer device RPA in the list is valid"]
    #[inline(always)]
    pub fn peer_addr_rpa_val(&mut self) -> PEER_ADDR_RPA_VAL_W {
        PEER_ADDR_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 6 - Indicates that the received self RPA in the list is valid"]
    #[inline(always)]
    pub fn self_addr_rxd_rpa_val(&mut self) -> SELF_ADDR_RXD_RPA_VAL_W {
        SELF_ADDR_RXD_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 7 - Indicates that the self RPA in the list to be transmitted is valid"]
    #[inline(always)]
    pub fn self_addr_tx_rpa_val(&mut self) -> SELF_ADDR_TX_RPA_VAL_W {
        SELF_ADDR_TX_RPA_VAL_W { w: self }
    }
    #[doc = "Bit 8 - When Initiator whitelist is disabled, this bit indicates the specific device to from which ADV packets will be accepted."]
    #[inline(always)]
    pub fn self_addr_init_rpa_sel(&mut self) -> SELF_ADDR_INIT_RPA_SEL_W {
        SELF_ADDR_INIT_RPA_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Indicates the TX addr type to be used for SCANA and INITA 0 - Self Identity address is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets 1 - Self RPA address provided in RSLV_LIST_TX_INIT_RPA field in the resolving list with the associated valid bit in SELF_ADDR_TX_RPA_VAL above is used in SCANA/INITA in SCAN_REQ/CONN_REQ packets"]
    #[inline(always)]
    pub fn self_addr_type_tx(&mut self) -> SELF_ADDR_TYPE_TX_W {
        SELF_ADDR_TYPE_TX_W { w: self }
    }
    #[doc = "Bit 10 - Indicates if the entry is already in connection with our device"]
    #[inline(always)]
    pub fn entry_connected(&mut self) -> ENTRY_CONNECTED_W {
        ENTRY_CONNECTED_W { w: self }
    }
}
