#[doc = "Reader of register CONN_CONFIG_EXT"]
pub type R = crate::R<u32, super::CONN_CONFIG_EXT>;
#[doc = "Writer for register CONN_CONFIG_EXT"]
pub type W = crate::W<u32, super::CONN_CONFIG_EXT>;
#[doc = "Register CONN_CONFIG_EXT `reset()`'s with value 0xa000"]
impl crate::ResetValue for super::CONN_CONFIG_EXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa000
    }
}
#[doc = "Reader of field `CONN_REQ_2SLOT_EARLY`"]
pub type CONN_REQ_2SLOT_EARLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_REQ_2SLOT_EARLY`"]
pub struct CONN_REQ_2SLOT_EARLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_2SLOT_EARLY_W<'a> {
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
#[doc = "Reader of field `CONN_REQ_3SLOT_EARLY`"]
pub type CONN_REQ_3SLOT_EARLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_REQ_3SLOT_EARLY`"]
pub struct CONN_REQ_3SLOT_EARLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_3SLOT_EARLY_W<'a> {
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
#[doc = "Reader of field `FW_PKT_RCV_CONN_INDEX`"]
pub type FW_PKT_RCV_CONN_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FW_PKT_RCV_CONN_INDEX`"]
pub struct FW_PKT_RCV_CONN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_PKT_RCV_CONN_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `MMMS_RX_PKT_LIMIT`"]
pub type MMMS_RX_PKT_LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MMMS_RX_PKT_LIMIT`"]
pub struct MMMS_RX_PKT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MMMS_RX_PKT_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DEBUG_CE_EXPIRE`"]
pub type DEBUG_CE_EXPIRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUG_CE_EXPIRE`"]
pub struct DEBUG_CE_EXPIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_CE_EXPIRE_W<'a> {
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
#[doc = "Reader of field `MT_PDU_CE_EXPIRE`"]
pub type MT_PDU_CE_EXPIRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MT_PDU_CE_EXPIRE`"]
pub struct MT_PDU_CE_EXPIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MT_PDU_CE_EXPIRE_W<'a> {
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
    #[doc = "Bit 0 - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_2slot_early(&self) -> CONN_REQ_2SLOT_EARLY_R {
        CONN_REQ_2SLOT_EARLY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_3slot_early(&self) -> CONN_REQ_3SLOT_EARLY_R {
        CONN_REQ_3SLOT_EARLY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
    #[inline(always)]
    pub fn fw_pkt_rcv_conn_index(&self) -> FW_PKT_RCV_CONN_INDEX_R {
        FW_PKT_RCV_CONN_INDEX_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
    #[inline(always)]
    pub fn mmms_rx_pkt_limit(&self) -> MMMS_RX_PKT_LIMIT_R {
        MMMS_RX_PKT_LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - MMMS CE expire control bit"]
    #[inline(always)]
    pub fn debug_ce_expire(&self) -> DEBUG_CE_EXPIRE_R {
        DEBUG_CE_EXPIRE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMMS empty PDU CE expire handling control bit"]
    #[inline(always)]
    pub fn mt_pdu_ce_expire(&self) -> MT_PDU_CE_EXPIRE_R {
        MT_PDU_CE_EXPIRE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable extension of the Conn Request to arbiter to 2 slot early. When enabled the request length is 3 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY bit. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_2slot_early(&mut self) -> CONN_REQ_2SLOT_EARLY_W {
        CONN_REQ_2SLOT_EARLY_W { w: self }
    }
    #[doc = "Bit 1 - This bit is used to enable extension of the Conn Request to arbiter to 3 slot early. When enabled the request length is 4 slots, irrespective of the status of CONN_REQ_1SLOT_EARLY & CONN_REQ_2SLOT_EARLY bits. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_3slot_early(&mut self) -> CONN_REQ_3SLOT_EARLY_W {
        CONN_REQ_3SLOT_EARLY_W { w: self }
    }
    #[doc = "Bits 2:6 - Connection Index for which the FW generates Packet Received Command. In MMMS mode, FW should write this field before giving PKT_RECEIVE_COMMAND to HW."]
    #[inline(always)]
    pub fn fw_pkt_rcv_conn_index(&mut self) -> FW_PKT_RCV_CONN_INDEX_W {
        FW_PKT_RCV_CONN_INDEX_W { w: self }
    }
    #[doc = "Bits 8:13 - Receive Packet Limit for MMMS mode. This is the RX_FIFO Limit and applies to all connections together"]
    #[inline(always)]
    pub fn mmms_rx_pkt_limit(&mut self) -> MMMS_RX_PKT_LIMIT_W {
        MMMS_RX_PKT_LIMIT_W { w: self }
    }
    #[doc = "Bit 14 - MMMS CE expire control bit"]
    #[inline(always)]
    pub fn debug_ce_expire(&mut self) -> DEBUG_CE_EXPIRE_W {
        DEBUG_CE_EXPIRE_W { w: self }
    }
    #[doc = "Bit 15 - MMMS empty PDU CE expire handling control bit"]
    #[inline(always)]
    pub fn mt_pdu_ce_expire(&mut self) -> MT_PDU_CE_EXPIRE_W {
        MT_PDU_CE_EXPIRE_W { w: self }
    }
}
