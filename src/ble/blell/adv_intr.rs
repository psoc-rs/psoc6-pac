#[doc = "Reader of register ADV_INTR"]
pub type R = crate::R<u32, super::ADV_INTR>;
#[doc = "Writer for register ADV_INTR"]
pub type W = crate::W<u32, super::ADV_INTR>;
#[doc = "Register ADV_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADV_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_STRT_INTR`"]
pub type ADV_STRT_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_STRT_INTR`"]
pub struct ADV_STRT_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_STRT_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_CLOSE_INTR`"]
pub type ADV_CLOSE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_CLOSE_INTR`"]
pub struct ADV_CLOSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CLOSE_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_TX_INTR`"]
pub type ADV_TX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_TX_INTR`"]
pub struct ADV_TX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TX_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_RSP_TX_INTR`"]
pub type SCAN_RSP_TX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_RSP_TX_INTR`"]
pub struct SCAN_RSP_TX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RSP_TX_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_REQ_RX_INTR`"]
pub type SCAN_REQ_RX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_REQ_RX_INTR`"]
pub struct SCAN_REQ_RX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_REQ_RX_INTR_W<'a> {
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
#[doc = "Reader of field `CONN_REQ_RX_INTR`"]
pub type CONN_REQ_RX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_REQ_RX_INTR`"]
pub struct CONN_REQ_RX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_RX_INTR_W<'a> {
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
#[doc = "Reader of field `SLV_CONNECTED`"]
pub type SLV_CONNECTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CONNECTED`"]
pub struct SLV_CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CONNECTED_W<'a> {
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
#[doc = "Reader of field `ADV_TIMEOUT`"]
pub type ADV_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_TIMEOUT`"]
pub struct ADV_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TIMEOUT_W<'a> {
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
#[doc = "Reader of field `ADV_ON`"]
pub type ADV_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLV_CONN_PEER_RPA_UNMCH_INTR`"]
pub type SLV_CONN_PEER_RPA_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CONN_PEER_RPA_UNMCH_INTR`"]
pub struct SLV_CONN_PEER_RPA_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CONN_PEER_RPA_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_REQ_RX_PEER_RPA_UNMCH_INTR`"]
pub type SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_REQ_RX_PEER_RPA_UNMCH_INTR`"]
pub struct SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `INIT_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_strt_intr(&self) -> ADV_STRT_INTR_R {
        ADV_STRT_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_close_intr(&self) -> ADV_CLOSE_INTR_R {
        ADV_CLOSE_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_tx_intr(&self) -> ADV_TX_INTR_R {
        ADV_TX_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_rsp_tx_intr(&self) -> SCAN_RSP_TX_INTR_R {
        SCAN_RSP_TX_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_req_rx_intr(&self) -> SCAN_REQ_RX_INTR_R {
        SCAN_REQ_RX_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn conn_req_rx_intr(&self) -> CONN_REQ_RX_INTR_R {
        CONN_REQ_RX_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
    #[inline(always)]
    pub fn slv_connected(&self) -> SLV_CONNECTED_R {
        SLV_CONNECTED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_timeout(&self) -> ADV_TIMEOUT_R {
        ADV_TIMEOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Advertiser procedure is ON in hardware. Indicates that advertiser procedure is ON in hardware. 1 - ON 0 - OFF"]
    #[inline(always)]
    pub fn adv_on(&self) -> ADV_ON_R {
        ADV_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_unmch_intr(&self) -> SLV_CONN_PEER_RPA_UNMCH_INTR_R {
        SLV_CONN_PEER_RPA_UNMCH_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_req_rx_peer_rpa_unmch_intr(&self) -> SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R {
        SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn init_addr_match_priv_mismatch_intr(&self) -> INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_addr_match_priv_mismatch_intr(&self) -> SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_strt_intr(&mut self) -> ADV_STRT_INTR_W {
        ADV_STRT_INTR_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_close_intr(&mut self) -> ADV_CLOSE_INTR_W {
        ADV_CLOSE_INTR_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_tx_intr(&mut self) -> ADV_TX_INTR_W {
        ADV_TX_INTR_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_rsp_tx_intr(&mut self) -> SCAN_RSP_TX_INTR_W {
        SCAN_RSP_TX_INTR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_req_rx_intr(&mut self) -> SCAN_REQ_RX_INTR_W {
        SCAN_REQ_RX_INTR_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn conn_req_rx_intr(&mut self) -> CONN_REQ_RX_INTR_W {
        CONN_REQ_RX_INTR_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
    #[inline(always)]
    pub fn slv_connected(&mut self) -> SLV_CONNECTED_W {
        SLV_CONNECTED_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_timeout(&mut self) -> ADV_TIMEOUT_W {
        ADV_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_unmch_intr(&mut self) -> SLV_CONN_PEER_RPA_UNMCH_INTR_W {
        SLV_CONN_PEER_RPA_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 10 - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_req_rx_peer_rpa_unmch_intr(&mut self) -> SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W {
        SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 11 - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn init_addr_match_priv_mismatch_intr(&mut self) -> INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
    #[doc = "Bit 12 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_addr_match_priv_mismatch_intr(&mut self) -> SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
}
