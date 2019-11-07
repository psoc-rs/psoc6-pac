#[doc = "Reader of register CONN_1_CE_DATA_LIST_CFG"]
pub type R = crate::R<u32, super::CONN_1_CE_DATA_LIST_CFG>;
#[doc = "Writer for register CONN_1_CE_DATA_LIST_CFG"]
pub type W = crate::W<u32, super::CONN_1_CE_DATA_LIST_CFG>;
#[doc = "Register CONN_1_CE_DATA_LIST_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_1_CE_DATA_LIST_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_LIST_INDEX_LAST_ACK_INDEX_C1`"]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_LIST_INDEX_LAST_ACK_INDEX_C1`"]
pub struct DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DATA_LIST_HEAD_UP_C1`"]
pub type DATA_LIST_HEAD_UP_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_LIST_HEAD_UP_C1`"]
pub struct DATA_LIST_HEAD_UP_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LIST_HEAD_UP_C1_W<'a> {
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
#[doc = "Reader of field `SLV_MD_CONFIG_C1`"]
pub type SLV_MD_CONFIG_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_MD_CONFIG_C1`"]
pub struct SLV_MD_CONFIG_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_MD_CONFIG_C1_W<'a> {
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
#[doc = "Reader of field `MD_C1`"]
pub type MD_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD_C1`"]
pub struct MD_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_C1_W<'a> {
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
#[doc = "Reader of field `MD_BIT_CLEAR_C1`"]
pub type MD_BIT_CLEAR_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD_BIT_CLEAR_C1`"]
pub struct MD_BIT_CLEAR_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_BIT_CLEAR_C1_W<'a> {
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
#[doc = "Reader of field `PAUSE_DATA_C1`"]
pub type PAUSE_DATA_C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE_DATA_C1`"]
pub struct PAUSE_DATA_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_DATA_C1_W<'a> {
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
#[doc = "Reader of field `KILL_CONN`"]
pub type KILL_CONN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KILL_CONN`"]
pub struct KILL_CONN_W<'a> {
    w: &'a mut W,
}
impl<'a> KILL_CONN_W<'a> {
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
#[doc = "Reader of field `KILL_CONN_AFTER_TX`"]
pub type KILL_CONN_AFTER_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KILL_CONN_AFTER_TX`"]
pub struct KILL_CONN_AFTER_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> KILL_CONN_AFTER_TX_W<'a> {
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
#[doc = "Reader of field `EMPTYPDU_SENT`"]
pub type EMPTYPDU_SENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYPDU_SENT`"]
pub struct EMPTYPDU_SENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYPDU_SENT_W<'a> {
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
#[doc = "Reader of field `CURRENT_PDU_INDEX_C1`"]
pub type CURRENT_PDU_INDEX_C1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index_c1(&self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R {
        DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
    #[inline(always)]
    pub fn data_list_head_up_c1(&self) -> DATA_LIST_HEAD_UP_C1_R {
        DATA_LIST_HEAD_UP_C1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
    #[inline(always)]
    pub fn slv_md_config_c1(&self) -> SLV_MD_CONFIG_C1_R {
        SLV_MD_CONFIG_C1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md_c1(&self) -> MD_C1_R {
        MD_C1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\] and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear_c1(&self) -> MD_BIT_CLEAR_C1_R {
        MD_BIT_CLEAR_C1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data_c1(&self) -> PAUSE_DATA_C1_R {
        PAUSE_DATA_C1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Kills the connection immediately when the connection event is active"]
    #[inline(always)]
    pub fn kill_conn(&self) -> KILL_CONN_R {
        KILL_CONN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Kills the connection when the connection event is active and a TX is completed"]
    #[inline(always)]
    pub fn kill_conn_after_tx(&self) -> KILL_CONN_AFTER_TX_R {
        KILL_CONN_AFTER_TX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
    #[inline(always)]
    pub fn emptypdu_sent(&self) -> EMPTYPDU_SENT_R {
        EMPTYPDU_SENT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - The index of the transmit packet buffer that is currently in transmission/waiting for transmission."]
    #[inline(always)]
    pub fn current_pdu_index_c1(&self) -> CURRENT_PDU_INDEX_C1_R {
        CURRENT_PDU_INDEX_C1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index_c1(&mut self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W {
        DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W { w: self }
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
    #[inline(always)]
    pub fn data_list_head_up_c1(&mut self) -> DATA_LIST_HEAD_UP_C1_W {
        DATA_LIST_HEAD_UP_C1_W { w: self }
    }
    #[doc = "Bit 5 - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
    #[inline(always)]
    pub fn slv_md_config_c1(&mut self) -> SLV_MD_CONFIG_C1_W {
        SLV_MD_CONFIG_C1_W { w: self }
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md_c1(&mut self) -> MD_C1_W {
        MD_C1_W { w: self }
    }
    #[doc = "Bit 7 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\] and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear_c1(&mut self) -> MD_BIT_CLEAR_C1_W {
        MD_BIT_CLEAR_C1_W { w: self }
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data_c1(&mut self) -> PAUSE_DATA_C1_W {
        PAUSE_DATA_C1_W { w: self }
    }
    #[doc = "Bit 9 - Kills the connection immediately when the connection event is active"]
    #[inline(always)]
    pub fn kill_conn(&mut self) -> KILL_CONN_W {
        KILL_CONN_W { w: self }
    }
    #[doc = "Bit 10 - Kills the connection when the connection event is active and a TX is completed"]
    #[inline(always)]
    pub fn kill_conn_after_tx(&mut self) -> KILL_CONN_AFTER_TX_W {
        KILL_CONN_AFTER_TX_W { w: self }
    }
    #[doc = "Bit 11 - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
    #[inline(always)]
    pub fn emptypdu_sent(&mut self) -> EMPTYPDU_SENT_W {
        EMPTYPDU_SENT_W { w: self }
    }
}
