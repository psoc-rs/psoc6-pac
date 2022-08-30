#[doc = "Register `CE_CNFG_STS_REGISTER` reader"]
pub struct R(crate::R<CE_CNFG_STS_REGISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_CNFG_STS_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_CNFG_STS_REGISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_CNFG_STS_REGISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_CNFG_STS_REGISTER` writer"]
pub struct W(crate::W<CE_CNFG_STS_REGISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_CNFG_STS_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CE_CNFG_STS_REGISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_CNFG_STS_REGISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_LIST_INDEX_LAST_ACK_INDEX` reader - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_LIST_INDEX_LAST_ACK_INDEX` writer - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_LIST_HEAD_UP` reader - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
pub type DATA_LIST_HEAD_UP_R = crate::BitReader<bool>;
#[doc = "Field `DATA_LIST_HEAD_UP` writer - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
pub type DATA_LIST_HEAD_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, bool, O>;
#[doc = "Field `SPARE` reader - This bit is unused"]
pub type SPARE_R = crate::BitReader<bool>;
#[doc = "Field `SPARE` writer - This bit is unused"]
pub type SPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, bool, O>;
#[doc = "Field `MD` reader - MD bit set to '1' indicates device has more data to be sent."]
pub type MD_R = crate::BitReader<bool>;
#[doc = "Field `MD` writer - MD bit set to '1' indicates device has more data to be sent."]
pub type MD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, bool, O>;
#[doc = "Field `MAP_INDEX__CURR_INDEX` reader - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
pub type MAP_INDEX__CURR_INDEX_R = crate::BitReader<bool>;
#[doc = "Field `MAP_INDEX__CURR_INDEX` writer - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
pub type MAP_INDEX__CURR_INDEX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, bool, O>;
#[doc = "Field `PAUSE_DATA` reader - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
pub type PAUSE_DATA_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_DATA` writer - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
pub type PAUSE_DATA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CE_CNFG_STS_REGISTER_SPEC, bool, O>;
#[doc = "Field `CONN_ACTIVE` reader - This bit is '1' whenever the connection is active."]
pub type CONN_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `CURRENT_PDU_INDEX` reader - The index of the transmit packet buffer that is currently in transmission/waiting for transmission."]
pub type CURRENT_PDU_INDEX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index(&self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_R {
        DATA_LIST_INDEX_LAST_ACK_INDEX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
    #[inline(always)]
    pub fn data_list_head_up(&self) -> DATA_LIST_HEAD_UP_R {
        DATA_LIST_HEAD_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is unused"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
    #[inline(always)]
    pub fn map_index__curr_index(&self) -> MAP_INDEX__CURR_INDEX_R {
        MAP_INDEX__CURR_INDEX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data(&self) -> PAUSE_DATA_R {
        PAUSE_DATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit is '1' whenever the connection is active."]
    #[inline(always)]
    pub fn conn_active(&self) -> CONN_ACTIVE_R {
        CONN_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
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
    pub fn data_list_index_last_ack_index(&mut self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_W<0> {
        DATA_LIST_INDEX_LAST_ACK_INDEX_W::new(self)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be toggled every time the firmware needs to indicate the start/resume. This requires a read modify write operation."]
    #[inline(always)]
    pub fn data_list_head_up(&mut self) -> DATA_LIST_HEAD_UP_W<4> {
        DATA_LIST_HEAD_UP_W::new(self)
    }
    #[doc = "Bit 5 - This bit is unused"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W<5> {
        SPARE_W::new(self)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<6> {
        MD_W::new(self)
    }
    #[doc = "Bit 7 - Written by firmware to select the map index to be used by hardware for this connection. 1 - use channel map register set 1. 0 - use channel map register set 0. When firmware reads this field, it returns the current map index being used in hardware."]
    #[inline(always)]
    pub fn map_index__curr_index(&mut self) -> MAP_INDEX__CURR_INDEX_W<7> {
        MAP_INDEX__CURR_INDEX_W::new(self)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data(&mut self) -> PAUSE_DATA_W<8> {
        PAUSE_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "connection configuration & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cnfg_sts_register](index.html) module"]
pub struct CE_CNFG_STS_REGISTER_SPEC;
impl crate::RegisterSpec for CE_CNFG_STS_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_cnfg_sts_register::R](R) reader structure"]
impl crate::Readable for CE_CNFG_STS_REGISTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_cnfg_sts_register::W](W) writer structure"]
impl crate::Writable for CE_CNFG_STS_REGISTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_CNFG_STS_REGISTER to value 0"]
impl crate::Resettable for CE_CNFG_STS_REGISTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
