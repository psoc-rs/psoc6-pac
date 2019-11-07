#[doc = "Reader of register CE_CNFG_STS_REGISTER"]
pub type R = crate::R<u32, super::CE_CNFG_STS_REGISTER>;
#[doc = "Writer for register CE_CNFG_STS_REGISTER"]
pub type W = crate::W<u32, super::CE_CNFG_STS_REGISTER>;
#[doc = "Register CE_CNFG_STS_REGISTER `reset()`'s with value 0"]
impl crate::ResetValue for super::CE_CNFG_STS_REGISTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_LIST_INDEX_LAST_ACK_INDEX`"]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_LIST_INDEX_LAST_ACK_INDEX`"]
pub struct DATA_LIST_INDEX_LAST_ACK_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LIST_INDEX_LAST_ACK_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DATA_LIST_HEAD_UP`"]
pub type DATA_LIST_HEAD_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_LIST_HEAD_UP`"]
pub struct DATA_LIST_HEAD_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LIST_HEAD_UP_W<'a> {
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
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
#[doc = "Reader of field `MD`"]
pub type MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD`"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
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
#[doc = "Reader of field `MAP_INDEX__CURR_INDEX`"]
pub type MAP_INDEX__CURR_INDEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAP_INDEX__CURR_INDEX`"]
pub struct MAP_INDEX__CURR_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_INDEX__CURR_INDEX_W<'a> {
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
#[doc = "Reader of field `PAUSE_DATA`"]
pub type PAUSE_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSE_DATA`"]
pub struct PAUSE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_DATA_W<'a> {
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
#[doc = "Reader of field `CONN_ACTIVE`"]
pub type CONN_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURRENT_PDU_INDEX`"]
pub type CURRENT_PDU_INDEX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index(&self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_R {
        DATA_LIST_INDEX_LAST_ACK_INDEX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
    #[inline(always)]
    pub fn data_list_head_up(&self) -> DATA_LIST_HEAD_UP_R {
        DATA_LIST_HEAD_UP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is unused"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
    #[inline(always)]
    pub fn map_index__curr_index(&self) -> MAP_INDEX__CURR_INDEX_R {
        MAP_INDEX__CURR_INDEX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data(&self) -> PAUSE_DATA_R {
        PAUSE_DATA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is '1' whenever the connection is active."]
    #[inline(always)]
    pub fn conn_active(&self) -> CONN_ACTIVE_R {
        CONN_ACTIVE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - The index of the transmit packet buffer that is currently in transmission/waiting for transmission."]
    #[inline(always)]
    pub fn current_pdu_index(&self) -> CURRENT_PDU_INDEX_R {
        CURRENT_PDU_INDEX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index(&mut self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_W {
        DATA_LIST_INDEX_LAST_ACK_INDEX_W { w: self }
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
    #[inline(always)]
    pub fn data_list_head_up(&mut self) -> DATA_LIST_HEAD_UP_W {
        DATA_LIST_HEAD_UP_W { w: self }
    }
    #[doc = "Bit 5 - This bit is unused"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Bit 7 - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
    #[inline(always)]
    pub fn map_index__curr_index(&mut self) -> MAP_INDEX__CURR_INDEX_W {
        MAP_INDEX__CURR_INDEX_W { w: self }
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data(&mut self) -> PAUSE_DATA_W {
        PAUSE_DATA_W { w: self }
    }
}
