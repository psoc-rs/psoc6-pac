#[doc = "Reader of register SCAN_INTR"]
pub type R = crate::R<u32, super::SCAN_INTR>;
#[doc = "Writer for register SCAN_INTR"]
pub type W = crate::W<u32, super::SCAN_INTR>;
#[doc = "Register SCAN_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCAN_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCAN_STRT_INTR`"]
pub type SCAN_STRT_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_STRT_INTR`"]
pub struct SCAN_STRT_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_STRT_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_CLOSE_INTR`"]
pub type SCAN_CLOSE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_CLOSE_INTR`"]
pub struct SCAN_CLOSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_CLOSE_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_TX_INTR`"]
pub type SCAN_TX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_TX_INTR`"]
pub struct SCAN_TX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_TX_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_RX_INTR`"]
pub type ADV_RX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_INTR`"]
pub struct ADV_RX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_RSP_RX_INTR`"]
pub type SCAN_RSP_RX_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_RSP_RX_INTR`"]
pub struct SCAN_RSP_RX_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_RSP_RX_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_RX_PEER_RPA_UNMCH_INTR`"]
pub type ADV_RX_PEER_RPA_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_PEER_RPA_UNMCH_INTR`"]
pub struct ADV_RX_PEER_RPA_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_PEER_RPA_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `ADV_RX_SELF_RPA_UNMCH_INTR`"]
pub type ADV_RX_SELF_RPA_UNMCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_SELF_RPA_UNMCH_INTR`"]
pub struct ADV_RX_SELF_RPA_UNMCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_SELF_RPA_UNMCH_INTR_W<'a> {
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
#[doc = "Reader of field `SCANA_TX_ADDR_NOT_SET_INTR`"]
pub type SCANA_TX_ADDR_NOT_SET_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANA_TX_ADDR_NOT_SET_INTR`"]
pub struct SCANA_TX_ADDR_NOT_SET_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANA_TX_ADDR_NOT_SET_INTR_W<'a> {
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
#[doc = "Reader of field `SCAN_ON`"]
pub type SCAN_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEER_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEER_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
#[doc = "Reader of field `SELF_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub type SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELF_ADDR_MATCH_PRIV_MISMATCH_INTR`"]
pub struct SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a> {
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
    #[doc = "Bit 0 - If this bit is set it indicates scan window is opened. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_strt_intr(&self) -> SCAN_STRT_INTR_R {
        SCAN_STRT_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates scan window is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_close_intr(&self) -> SCAN_CLOSE_INTR_R {
        SCAN_CLOSE_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates scan request packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_tx_intr(&self) -> SCAN_TX_INTR_R {
        SCAN_TX_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set it indicates ADV packet received. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets. Note: Any ADV RX interrupt received after issuing SCAN_STOP command must be ignored and the ADVCH FIFO flushed."]
    #[inline(always)]
    pub fn adv_rx_intr(&self) -> ADV_RX_INTR_R {
        ADV_RX_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates SCAN_RSP packet is received. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. Write to the register with this bit set to 1, clears the interrupt source. NOTE: This interrupt is generated while active scanning upon receiving scan response packet."]
    #[inline(always)]
    pub fn scan_rsp_rx_intr(&self) -> SCAN_RSP_RX_INTR_R {
        SCAN_RSP_RX_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV packet received but the peer device Address is not match yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_rpa_unmch_intr(&self) -> ADV_RX_PEER_RPA_UNMCH_INTR_R {
        ADV_RX_PEER_RPA_UNMCH_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv_direct packets."]
    #[inline(always)]
    pub fn adv_rx_self_rpa_unmch_intr(&self) -> ADV_RX_SELF_RPA_UNMCH_INTR_R {
        ADV_RX_SELF_RPA_UNMCH_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid ScanA RPA to be transmitted in SCAN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr(&self) -> SCANA_TX_ADDR_NOT_SET_INTR_R {
        SCANA_TX_ADDR_NOT_SET_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Scan procedure status. 1 - scan procedure is active. 0 - scan procedure is not active."]
    #[inline(always)]
    pub fn scan_on(&self) -> SCAN_ON_R {
        SCAN_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn peer_addr_match_priv_mismatch_intr(&self) -> PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If this bit is set it indicates that the self Identity address is received from an initiator and matches, but self IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn self_addr_match_priv_mismatch_intr(&self) -> SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates scan window is opened. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_strt_intr(&mut self) -> SCAN_STRT_INTR_W {
        SCAN_STRT_INTR_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set it indicates scan window is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_close_intr(&mut self) -> SCAN_CLOSE_INTR_W {
        SCAN_CLOSE_INTR_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates scan request packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_tx_intr(&mut self) -> SCAN_TX_INTR_W {
        SCAN_TX_INTR_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set it indicates ADV packet received. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets. Note: Any ADV RX interrupt received after issuing SCAN_STOP command must be ignored and the ADVCH FIFO flushed."]
    #[inline(always)]
    pub fn adv_rx_intr(&mut self) -> ADV_RX_INTR_W {
        ADV_RX_INTR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set it indicates SCAN_RSP packet is received. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. Write to the register with this bit set to 1, clears the interrupt source. NOTE: This interrupt is generated while active scanning upon receiving scan response packet."]
    #[inline(always)]
    pub fn scan_rsp_rx_intr(&mut self) -> SCAN_RSP_RX_INTR_W {
        SCAN_RSP_RX_INTR_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set it indicates ADV packet received but the peer device Address is not match yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv packets."]
    #[inline(always)]
    pub fn adv_rx_peer_rpa_unmch_intr(&mut self) -> ADV_RX_PEER_RPA_UNMCH_INTR_W {
        ADV_RX_PEER_RPA_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set it indicates ADV_DIRECT packet received but the self device Resolvable Private Address is not resolved yet. Firmware can read the content of the packet from the INIT_SCN_ADV_RX_FIFO. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set. Write to the register with this bit set to 1, clears the interrupt source. This interrupt is generated while active/passive scanning upon receiving adv_direct packets."]
    #[inline(always)]
    pub fn adv_rx_self_rpa_unmch_intr(&mut self) -> ADV_RX_SELF_RPA_UNMCH_INTR_W {
        ADV_RX_SELF_RPA_UNMCH_INTR_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set it indicates that a valid ScanA RPA to be transmitted in SCAN_REQ packet in response to an ADV packet is not present in the resolving list Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr(&mut self) -> SCANA_TX_ADDR_NOT_SET_INTR_W {
        SCANA_TX_ADDR_NOT_SET_INTR_W { w: self }
    }
    #[doc = "Bit 9 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn peer_addr_match_priv_mismatch_intr(&mut self) -> PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        PEER_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
    #[doc = "Bit 10 - If this bit is set it indicates that the self Identity address is received from an initiator and matches, but self IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn self_addr_match_priv_mismatch_intr(&mut self) -> SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W {
        SELF_ADDR_MATCH_PRIV_MISMATCH_INTR_W { w: self }
    }
}
