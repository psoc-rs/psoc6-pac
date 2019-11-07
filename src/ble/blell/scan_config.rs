#[doc = "Reader of register SCAN_CONFIG"]
pub type R = crate::R<u32, super::SCAN_CONFIG>;
#[doc = "Writer for register SCAN_CONFIG"]
pub type W = crate::W<u32, super::SCAN_CONFIG>;
#[doc = "Register SCAN_CONFIG `reset()`'s with value 0xe01f"]
impl crate::ResetValue for super::SCAN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe01f
    }
}
#[doc = "Reader of field `SCN_STRT_EN`"]
pub type SCN_STRT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_STRT_EN`"]
pub struct SCN_STRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_STRT_EN_W<'a> {
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
#[doc = "Reader of field `SCN_CLOSE_EN`"]
pub type SCN_CLOSE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_CLOSE_EN`"]
pub struct SCN_CLOSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_CLOSE_EN_W<'a> {
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
#[doc = "Reader of field `SCN_TX_EN`"]
pub type SCN_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_TX_EN`"]
pub struct SCN_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_TX_EN_W<'a> {
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
#[doc = "Reader of field `ADV_RX_EN`"]
pub type ADV_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RX_EN`"]
pub struct ADV_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RX_EN_W<'a> {
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
#[doc = "Reader of field `SCN_RSP_RX_EN`"]
pub type SCN_RSP_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_RSP_RX_EN`"]
pub struct SCN_RSP_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_RSP_RX_EN_W<'a> {
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
#[doc = "Reader of field `SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN`"]
pub type SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN`"]
pub struct SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W<'a> {
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
#[doc = "Reader of field `SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN`"]
pub type SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN`"]
pub struct SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W<'a> {
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
#[doc = "Reader of field `SCANA_TX_ADDR_NOT_SET_INTR_EN`"]
pub type SCANA_TX_ADDR_NOT_SET_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANA_TX_ADDR_NOT_SET_INTR_EN`"]
pub struct SCANA_TX_ADDR_NOT_SET_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANA_TX_ADDR_NOT_SET_INTR_EN_W<'a> {
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
#[doc = "Reader of field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN`"]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN`"]
pub struct RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a> {
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
#[doc = "Reader of field `BACKOFF_ENABLE`"]
pub type BACKOFF_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BACKOFF_ENABLE`"]
pub struct BACKOFF_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKOFF_ENABLE_W<'a> {
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
#[doc = "Reader of field `SCAN_CHANNEL_MAP`"]
pub type SCAN_CHANNEL_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCAN_CHANNEL_MAP`"]
pub struct SCAN_CHANNEL_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_CHANNEL_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable scan event start interrupt."]
    #[inline(always)]
    pub fn scn_strt_en(&self) -> SCN_STRT_EN_R {
        SCN_STRT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable scan event close interrupt."]
    #[inline(always)]
    pub fn scn_close_en(&self) -> SCN_CLOSE_EN_R {
        SCN_CLOSE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable scan request packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_tx_en(&self) -> SCN_TX_EN_R {
        SCN_TX_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable adv packet received interrupt ."]
    #[inline(always)]
    pub fn adv_rx_en(&self) -> ADV_RX_EN_R {
        ADV_RX_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable scan_rsp packet received interrupt ."]
    #[inline(always)]
    pub fn scn_rsp_rx_en(&self) -> SCN_RSP_RX_EN_R {
        SCN_RSP_RX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_peer_rpa_unmch_en(&self) -> SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R {
        SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_self_rpa_unmch_en(&self) -> SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R {
        SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr_en(&self) -> SCANA_TX_ADDR_NOT_SET_INTR_EN_R {
        SCANA_TX_ADDR_NOT_SET_INTR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_scn(&self) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
    #[inline(always)]
    pub fn backoff_enable(&self) -> BACKOFF_ENABLE_R {
        BACKOFF_ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn scan_channel_map(&self) -> SCAN_CHANNEL_MAP_R {
        SCAN_CHANNEL_MAP_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable scan event start interrupt."]
    #[inline(always)]
    pub fn scn_strt_en(&mut self) -> SCN_STRT_EN_W {
        SCN_STRT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable scan event close interrupt."]
    #[inline(always)]
    pub fn scn_close_en(&mut self) -> SCN_CLOSE_EN_W {
        SCN_CLOSE_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable scan request packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_tx_en(&mut self) -> SCN_TX_EN_W {
        SCN_TX_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enable adv packet received interrupt ."]
    #[inline(always)]
    pub fn adv_rx_en(&mut self) -> ADV_RX_EN_W {
        ADV_RX_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable scan_rsp packet received interrupt ."]
    #[inline(always)]
    pub fn scn_rsp_rx_en(&mut self) -> SCN_RSP_RX_EN_W {
        SCN_RSP_RX_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_peer_rpa_unmch_en(&mut self) -> SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W {
        SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_self_rpa_unmch_en(&mut self) -> SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W {
        SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr_en(&mut self) -> SCANA_TX_ADDR_NOT_SET_INTR_EN_W {
        SCANA_TX_ADDR_NOT_SET_INTR_EN_W { w: self }
    }
    #[doc = "Bit 8 - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_scn(
        &mut self,
    ) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W { w: self }
    }
    #[doc = "Bit 11 - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
    #[inline(always)]
    pub fn backoff_enable(&mut self) -> BACKOFF_ENABLE_W {
        BACKOFF_ENABLE_W { w: self }
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn scan_channel_map(&mut self) -> SCAN_CHANNEL_MAP_W {
        SCAN_CHANNEL_MAP_W { w: self }
    }
}
