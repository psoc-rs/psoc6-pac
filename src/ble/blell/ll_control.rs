#[doc = "Reader of register LL_CONTROL"]
pub type R = crate::R<u32, super::LL_CONTROL>;
#[doc = "Writer for register LL_CONTROL"]
pub type W = crate::W<u32, super::LL_CONTROL>;
#[doc = "Register LL_CONTROL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::LL_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `PRIV_1_2`"]
pub type PRIV_1_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV_1_2`"]
pub struct PRIV_1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_1_2_W<'a> {
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
#[doc = "Reader of field `DLE`"]
pub type DLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLE`"]
pub struct DLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLE_W<'a> {
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
#[doc = "Reader of field `WL_READ_AS_MEM`"]
pub type WL_READ_AS_MEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WL_READ_AS_MEM`"]
pub struct WL_READ_AS_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_READ_AS_MEM_W<'a> {
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
#[doc = "Reader of field `ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL`"]
pub type ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL`"]
pub struct ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W<'a> {
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
#[doc = "Reader of field `HW_RSLV_LIST_FULL`"]
pub type HW_RSLV_LIST_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_RSLV_LIST_FULL`"]
pub struct HW_RSLV_LIST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_RSLV_LIST_FULL_W<'a> {
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
#[doc = "Reader of field `RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV`"]
pub type RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV`"]
pub struct RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a> {
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
#[doc = "Reader of field `RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV`"]
pub type RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV`"]
pub struct RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W<'a> {
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
#[doc = "Reader of field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN`"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN`"]
pub struct RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a> {
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
#[doc = "Reader of field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI`"]
pub type RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI`"]
pub struct RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a> {
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
#[doc = "Reader of field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI`"]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI`"]
pub struct RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W<'a> {
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
#[doc = "Reader of field `PRIV_1_2_ADV`"]
pub type PRIV_1_2_ADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV_1_2_ADV`"]
pub struct PRIV_1_2_ADV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_1_2_ADV_W<'a> {
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
#[doc = "Reader of field `PRIV_1_2_SCAN`"]
pub type PRIV_1_2_SCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV_1_2_SCAN`"]
pub struct PRIV_1_2_SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_1_2_SCAN_W<'a> {
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
#[doc = "Reader of field `PRIV_1_2_INIT`"]
pub type PRIV_1_2_INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV_1_2_INIT`"]
pub struct PRIV_1_2_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_1_2_INIT_W<'a> {
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
#[doc = "Reader of field `EN_CONN_RX_EN_MOD`"]
pub type EN_CONN_RX_EN_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_CONN_RX_EN_MOD`"]
pub struct EN_CONN_RX_EN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_CONN_RX_EN_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLV_CONN_PEER_RPA_NOT_RSLVD`"]
pub type SLV_CONN_PEER_RPA_NOT_RSLVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CONN_PEER_RPA_NOT_RSLVD`"]
pub struct SLV_CONN_PEER_RPA_NOT_RSLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CONN_PEER_RPA_NOT_RSLVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `ADVCH_FIFO_FLUSH`"]
pub struct ADVCH_FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVCH_FIFO_FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables Privacy 1.2 Feature."]
    #[inline(always)]
    pub fn priv_1_2(&self) -> PRIV_1_2_R {
        PRIV_1_2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
    #[inline(always)]
    pub fn dle(&self) -> DLE_R {
        DLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
    #[inline(always)]
    pub fn wl_read_as_mem(&self) -> WL_READ_AS_MEM_R {
        WL_READ_AS_MEM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
    #[inline(always)]
    pub fn advch_fifo_priv_1_2_flush_ctrl(&self) -> ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R {
        ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
    #[inline(always)]
    pub fn hw_rslv_list_full(&self) -> HW_RSLV_LIST_FULL_R {
        HW_RSLV_LIST_FULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_init_addr_match_priv_mismatch_adv(&self) -> RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R {
        RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_scan_addr_match_priv_mismatch_adv(&self) -> RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R {
        RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_scn(&self) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_ini(&self) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_ini(&self) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables Privacy 1.2 for ADV engine"]
    #[inline(always)]
    pub fn priv_1_2_adv(&self) -> PRIV_1_2_ADV_R {
        PRIV_1_2_ADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables Privacy 1.2 for SCAN engine"]
    #[inline(always)]
    pub fn priv_1_2_scan(&self) -> PRIV_1_2_SCAN_R {
        PRIV_1_2_SCAN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables Privacy 1.2 for INIT engine"]
    #[inline(always)]
    pub fn priv_1_2_init(&self) -> PRIV_1_2_INIT_R {
        PRIV_1_2_INIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
    #[inline(always)]
    pub fn en_conn_rx_en_mod(&self) -> EN_CONN_RX_EN_MOD_R {
        EN_CONN_RX_EN_MOD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_not_rslvd(&self) -> SLV_CONN_PEER_RPA_NOT_RSLVD_R {
        SLV_CONN_PEER_RPA_NOT_RSLVD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables Privacy 1.2 Feature."]
    #[inline(always)]
    pub fn priv_1_2(&mut self) -> PRIV_1_2_W {
        PRIV_1_2_W { w: self }
    }
    #[doc = "Bit 1 - Enables Data Length extension feature in DTM, connection and encryption modules. This bit should always be set to 1'b1. 1'b0 is not supported."]
    #[inline(always)]
    pub fn dle(&mut self) -> DLE_W {
        DLE_W { w: self }
    }
    #[doc = "Bit 2 - The Whilelist read logic is controlled using this bit. 0 - The reads to the whitelist address range is treated as FIFO reads and the pointers are reset by issueing the RESET_READ_PTR command. 1 - The reads to the whitelist address range is treated an memory reads. Any whilelist entry can be read."]
    #[inline(always)]
    pub fn wl_read_as_mem(&mut self) -> WL_READ_AS_MEM_W {
        WL_READ_AS_MEM_W { w: self }
    }
    #[doc = "Bit 3 - Controls the ADVCH FIFO flushing when PRIV_1_2 is enabled. 0 - Flushes all ADV & INIT packets, as in non privacy 1.2 mode, except those with unresolved peer or self RPA. 1 - Does not flush any CRC good packets"]
    #[inline(always)]
    pub fn advch_fifo_priv_1_2_flush_ctrl(&mut self) -> ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W {
        ADVCH_FIFO_PRIV_1_2_FLUSH_CTRL_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that the resolving list in the hardware is full and the list is extended in the FW. This will affect the behavior of address resolution. 0 - The resolving list in the hardware is not fully filled. When Whitelist is disabled and a peer identity address not in the resolving list is received, the packet is responded to by the hardware. 1 - The resolving list in the hardware is fully filled. All address comparisons must be extended to the Firmware list as well, Any match in the Firmware list should be followed by copying the matching entry into the hardware resolving list."]
    #[inline(always)]
    pub fn hw_rslv_list_full(&mut self) -> HW_RSLV_LIST_FULL_W {
        HW_RSLV_LIST_FULL_W { w: self }
    }
    #[doc = "Bit 5 - This bit controls the ADV engine behavior when an initiator address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_init_addr_match_priv_mismatch_adv(
        &mut self,
    ) -> RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W {
        RPT_INIT_ADDR_MATCH_PRIV_MISMATCH_ADV_W { w: self }
    }
    #[doc = "Bit 6 - This bit controls the ADV engine behavior when a scanner address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_scan_addr_match_priv_mismatch_adv(
        &mut self,
    ) -> RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W {
        RPT_SCAN_ADDR_MATCH_PRIV_MISMATCH_ADV_W { w: self }
    }
    #[doc = "Bit 7 - This bit controls the SCAN engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_scn(
        &mut self,
    ) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_SCN_W { w: self }
    }
    #[doc = "Bit 8 - This bit controls the INIT engine behavior when an peer address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_peer_addr_match_priv_mismatch_ini(
        &mut self,
    ) -> RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W {
        RPT_PEER_ADDR_MATCH_PRIV_MISMATCH_INI_W { w: self }
    }
    #[doc = "Bit 9 - This bit controls the INIT engine behavior when a self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware"]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_ini(
        &mut self,
    ) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_INI_W { w: self }
    }
    #[doc = "Bit 10 - Enables Privacy 1.2 for ADV engine"]
    #[inline(always)]
    pub fn priv_1_2_adv(&mut self) -> PRIV_1_2_ADV_W {
        PRIV_1_2_ADV_W { w: self }
    }
    #[doc = "Bit 11 - Enables Privacy 1.2 for SCAN engine"]
    #[inline(always)]
    pub fn priv_1_2_scan(&mut self) -> PRIV_1_2_SCAN_W {
        PRIV_1_2_SCAN_W { w: self }
    }
    #[doc = "Bit 12 - Enables Privacy 1.2 for INIT engine"]
    #[inline(always)]
    pub fn priv_1_2_init(&mut self) -> PRIV_1_2_INIT_W {
        PRIV_1_2_INIT_W { w: self }
    }
    #[doc = "Bit 13 - This bit controls the Connection RX enable modification mode when SLV_CONN_PEER_RPA_NOT_RSLVD is set. 1'b0 - The Connection RX enable is unmodified 1'b1 - The Connection RX enable is during the Peer INIT RPA unresolved state is modified, until it is resolved."]
    #[inline(always)]
    pub fn en_conn_rx_en_mod(&mut self) -> EN_CONN_RX_EN_MOD_W {
        EN_CONN_RX_EN_MOD_W { w: self }
    }
    #[doc = "Bit 14 - This bit is asserted when SLV_CONN_PEER_RPA_UNMCH_INTR is set. The device does not enter into Connection established state until this bit is cleared after the RPA is resoved by the firmware. If the firmware is not able to resolve the RPA within the supervision timeout, the device aborts the connection establishement and this bit is cleared by the hardware. This bit is valid only if PRIV_1_2 is set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_not_rslvd(&mut self) -> SLV_CONN_PEER_RPA_NOT_RSLVD_W {
        SLV_CONN_PEER_RPA_NOT_RSLVD_W { w: self }
    }
    #[doc = "Bit 15 - When set, flushes the ADVCH FIFO. The bit is auto cleared. Note that this should be used only when the FIFO is not read by the firmware. If firmware has started reading the FIFO, then the FIFO must be emptied exclusively by firmware reads"]
    #[inline(always)]
    pub fn advch_fifo_flush(&mut self) -> ADVCH_FIFO_FLUSH_W {
        ADVCH_FIFO_FLUSH_W { w: self }
    }
}
