#[doc = "Reader of register CONN_CONFIG"]
pub type R = crate::R<u32, super::CONN_CONFIG>;
#[doc = "Writer for register CONN_CONFIG"]
pub type W = crate::W<u32, super::CONN_CONFIG>;
#[doc = "Register CONN_CONFIG `reset()`'s with value 0xe11f"]
impl crate::ResetValue for super::CONN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe11f
    }
}
#[doc = "Reader of field `RX_PKT_LIMIT`"]
pub type RX_PKT_LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_PKT_LIMIT`"]
pub struct RX_PKT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PKT_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RX_INTR_THRESHOLD`"]
pub type RX_INTR_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_INTR_THRESHOLD`"]
pub struct RX_INTR_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INTR_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MD_BIT_CLEAR`"]
pub type MD_BIT_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD_BIT_CLEAR`"]
pub struct MD_BIT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_BIT_CLEAR_W<'a> {
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
#[doc = "Reader of field `DSM_SLOT_VARIANCE`"]
pub type DSM_SLOT_VARIANCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_SLOT_VARIANCE`"]
pub struct DSM_SLOT_VARIANCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_SLOT_VARIANCE_W<'a> {
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
#[doc = "Reader of field `SLV_MD_CONFIG`"]
pub type SLV_MD_CONFIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_MD_CONFIG`"]
pub struct SLV_MD_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_MD_CONFIG_W<'a> {
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
#[doc = "Reader of field `EXTEND_CU_TX_WIN`"]
pub type EXTEND_CU_TX_WIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTEND_CU_TX_WIN`"]
pub struct EXTEND_CU_TX_WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEND_CU_TX_WIN_W<'a> {
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
#[doc = "Reader of field `MASK_SUTO_AT_UPDT`"]
pub type MASK_SUTO_AT_UPDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK_SUTO_AT_UPDT`"]
pub struct MASK_SUTO_AT_UPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_SUTO_AT_UPDT_W<'a> {
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
#[doc = "Reader of field `CONN_REQ_1SLOT_EARLY`"]
pub type CONN_REQ_1SLOT_EARLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_REQ_1SLOT_EARLY`"]
pub struct CONN_REQ_1SLOT_EARLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_REQ_1SLOT_EARLY_W<'a> {
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
    #[doc = "Bits 0:3 - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
    #[inline(always)]
    pub fn rx_pkt_limit(&self) -> RX_PKT_LIMIT_R {
        RX_PKT_LIMIT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
    #[inline(always)]
    pub fn rx_intr_threshold(&self) -> RX_INTR_THRESHOLD_R {
        RX_INTR_THRESHOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\] - md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\] and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&self) -> MD_BIT_CLEAR_R {
        MD_BIT_CLEAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&self) -> DSM_SLOT_VARIANCE_R {
        DSM_SLOT_VARIANCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&self) -> SLV_MD_CONFIG_R {
        SLV_MD_CONFIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&self) -> EXTEND_CU_TX_WIN_R {
        EXTEND_CU_TX_WIN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&self) -> MASK_SUTO_AT_UPDT_R {
        MASK_SUTO_AT_UPDT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&self) -> CONN_REQ_1SLOT_EARLY_R {
        CONN_REQ_1SLOT_EARLY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
    #[inline(always)]
    pub fn rx_pkt_limit(&mut self) -> RX_PKT_LIMIT_W {
        RX_PKT_LIMIT_W { w: self }
    }
    #[doc = "Bits 4:7 - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
    #[inline(always)]
    pub fn rx_intr_threshold(&mut self) -> RX_INTR_THRESHOLD_W {
        RX_INTR_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\] - md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\] and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&mut self) -> MD_BIT_CLEAR_W {
        MD_BIT_CLEAR_W { w: self }
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&mut self) -> DSM_SLOT_VARIANCE_W {
        DSM_SLOT_VARIANCE_W { w: self }
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&mut self) -> SLV_MD_CONFIG_W {
        SLV_MD_CONFIG_W { w: self }
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&mut self) -> EXTEND_CU_TX_WIN_W {
        EXTEND_CU_TX_WIN_W { w: self }
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&mut self) -> MASK_SUTO_AT_UPDT_W {
        MASK_SUTO_AT_UPDT_W { w: self }
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&mut self) -> CONN_REQ_1SLOT_EARLY_W {
        CONN_REQ_1SLOT_EARLY_W { w: self }
    }
}
