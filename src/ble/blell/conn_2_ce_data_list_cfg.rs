#[doc = "Register `CONN_2_CE_DATA_LIST_CFG` reader"]
pub struct R(crate::R<CONN_2_CE_DATA_LIST_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_2_CE_DATA_LIST_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_2_CE_DATA_LIST_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_2_CE_DATA_LIST_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_2_CE_DATA_LIST_CFG` writer"]
pub struct W(crate::W<CONN_2_CE_DATA_LIST_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_2_CE_DATA_LIST_CFG_SPEC>;
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
impl From<crate::W<CONN_2_CE_DATA_LIST_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_2_CE_DATA_LIST_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_LIST_INDEX_LAST_ACK_INDEX_C1` reader - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_LIST_INDEX_LAST_ACK_INDEX_C1` writer - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
pub type DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_LIST_HEAD_UP_C1` reader - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
pub type DATA_LIST_HEAD_UP_C1_R = crate::BitReader<bool>;
#[doc = "Field `DATA_LIST_HEAD_UP_C1` writer - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
pub type DATA_LIST_HEAD_UP_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `SLV_MD_CONFIG_C1` reader - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
pub type SLV_MD_CONFIG_C1_R = crate::BitReader<bool>;
#[doc = "Field `SLV_MD_CONFIG_C1` writer - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
pub type SLV_MD_CONFIG_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `MD_C1` reader - MD bit set to '1' indicates device has more data to be sent."]
pub type MD_C1_R = crate::BitReader<bool>;
#[doc = "Field `MD_C1` writer - MD bit set to '1' indicates device has more data to be sent."]
pub type MD_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `MD_BIT_CLEAR_C1` reader - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub type MD_BIT_CLEAR_C1_R = crate::BitReader<bool>;
#[doc = "Field `MD_BIT_CLEAR_C1` writer - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub type MD_BIT_CLEAR_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `PAUSE_DATA_C1` reader - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
pub type PAUSE_DATA_C1_R = crate::BitReader<bool>;
#[doc = "Field `PAUSE_DATA_C1` writer - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
pub type PAUSE_DATA_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `KILL_CONN` reader - Kills the connection immediately when the connection event is active"]
pub type KILL_CONN_R = crate::BitReader<bool>;
#[doc = "Field `KILL_CONN` writer - Kills the connection immediately when the connection event is active"]
pub type KILL_CONN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `KILL_CONN_AFTER_TX` reader - Kills the connection when the connection event is active and a TX is completed"]
pub type KILL_CONN_AFTER_TX_R = crate::BitReader<bool>;
#[doc = "Field `KILL_CONN_AFTER_TX` writer - Kills the connection when the connection event is active and a TX is completed"]
pub type KILL_CONN_AFTER_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `EMPTYPDU_SENT` reader - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
pub type EMPTYPDU_SENT_R = crate::BitReader<bool>;
#[doc = "Field `EMPTYPDU_SENT` writer - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
pub type EMPTYPDU_SENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_2_CE_DATA_LIST_CFG_SPEC, bool, O>;
#[doc = "Field `CURRENT_PDU_INDEX_C1` reader - The index of the transmit packet buffer that is currently in transmission/waiting for transmission."]
pub type CURRENT_PDU_INDEX_C1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Data list index for start/resume. This field must be valid along with data_list_head_up and indicate the transmit packet buffer index at which the data is loaded. The default number of buffers in the IP is 5,but may be customized for a customer. The buffers are in-dexed 0 to 4. Hardware will start the next data transmission from the index indicated by this field."]
    #[inline(always)]
    pub fn data_list_index_last_ack_index_c1(&self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R {
        DATA_LIST_INDEX_LAST_ACK_INDEX_C1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
    #[inline(always)]
    pub fn data_list_head_up_c1(&self) -> DATA_LIST_HEAD_UP_C1_R {
        DATA_LIST_HEAD_UP_C1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
    #[inline(always)]
    pub fn slv_md_config_c1(&self) -> SLV_MD_CONFIG_C1_R {
        SLV_MD_CONFIG_C1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md_c1(&self) -> MD_C1_R {
        MD_C1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear_c1(&self) -> MD_BIT_CLEAR_C1_R {
        MD_BIT_CLEAR_C1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data_c1(&self) -> PAUSE_DATA_C1_R {
        PAUSE_DATA_C1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Kills the connection immediately when the connection event is active"]
    #[inline(always)]
    pub fn kill_conn(&self) -> KILL_CONN_R {
        KILL_CONN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Kills the connection when the connection event is active and a TX is completed"]
    #[inline(always)]
    pub fn kill_conn_after_tx(&self) -> KILL_CONN_AFTER_TX_R {
        KILL_CONN_AFTER_TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
    #[inline(always)]
    pub fn emptypdu_sent(&self) -> EMPTYPDU_SENT_R {
        EMPTYPDU_SENT_R::new(((self.bits >> 11) & 1) != 0)
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
    pub fn data_list_index_last_ack_index_c1(&mut self) -> DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W<0> {
        DATA_LIST_INDEX_LAST_ACK_INDEX_C1_W::new(self)
    }
    #[doc = "Bit 4 - Update the first packet buffer index ready for transmis-sion to start/resume data transfer after a pause. The bit must be set every time the firmware needs to indicate the start/resume."]
    #[inline(always)]
    pub fn data_list_head_up_c1(&mut self) -> DATA_LIST_HEAD_UP_C1_W<4> {
        DATA_LIST_HEAD_UP_C1_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set to configure the MD bit control when the design is in slave mode. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has valid only when MD_BIT_CLEAR bit is not set"]
    #[inline(always)]
    pub fn slv_md_config_c1(&mut self) -> SLV_MD_CONFIG_C1_W<5> {
        SLV_MD_CONFIG_C1_W::new(self)
    }
    #[doc = "Bit 6 - MD bit set to '1' indicates device has more data to be sent."]
    #[inline(always)]
    pub fn md_c1(&mut self) -> MD_C1_W<6> {
        MD_C1_W::new(self)
    }
    #[doc = "Bit 7 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and software logic combined' 1 - MD bit is exclusively controlled by software, based on status of bit \\[6\\]. 0 - MD Bit in the transmitted PDU is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the MD in bit \\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear_c1(&mut self) -> MD_BIT_CLEAR_C1_W<7> {
        MD_BIT_CLEAR_C1_W::new(self)
    }
    #[doc = "Bit 8 - Pause data. 1 - pause data, 0 - do not pause. The current_pdu_index in hardware does not move to next in-dex until pause_data is cleared. But if the SENT bit is set for the current_pdu_index as which pause is set, data will be sent out"]
    #[inline(always)]
    pub fn pause_data_c1(&mut self) -> PAUSE_DATA_C1_W<8> {
        PAUSE_DATA_C1_W::new(self)
    }
    #[doc = "Bit 9 - Kills the connection immediately when the connection event is active"]
    #[inline(always)]
    pub fn kill_conn(&mut self) -> KILL_CONN_W<9> {
        KILL_CONN_W::new(self)
    }
    #[doc = "Bit 10 - Kills the connection when the connection event is active and a TX is completed"]
    #[inline(always)]
    pub fn kill_conn_after_tx(&mut self) -> KILL_CONN_AFTER_TX_W<10> {
        KILL_CONN_AFTER_TX_W::new(self)
    }
    #[doc = "Bit 11 - This bit indicates if EMPTYPDU has been sent. IF ACK is received this bit will be cleared by HW"]
    #[inline(always)]
    pub fn emptypdu_sent(&mut self) -> EMPTYPDU_SENT_W<11> {
        EMPTYPDU_SENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection specific pause resume for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_ce_data_list_cfg](index.html) module"]
pub struct CONN_2_CE_DATA_LIST_CFG_SPEC;
impl crate::RegisterSpec for CONN_2_CE_DATA_LIST_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_2_ce_data_list_cfg::R](R) reader structure"]
impl crate::Readable for CONN_2_CE_DATA_LIST_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_2_ce_data_list_cfg::W](W) writer structure"]
impl crate::Writable for CONN_2_CE_DATA_LIST_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_2_CE_DATA_LIST_CFG to value 0"]
impl crate::Resettable for CONN_2_CE_DATA_LIST_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
