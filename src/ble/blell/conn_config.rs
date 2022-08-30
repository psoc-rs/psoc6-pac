#[doc = "Register `CONN_CONFIG` reader"]
pub struct R(crate::R<CONN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CONFIG` writer"]
pub struct W(crate::W<CONN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CONFIG_SPEC>;
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
impl From<crate::W<CONN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_PKT_LIMIT` reader - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
pub type RX_PKT_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PKT_LIMIT` writer - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
pub type RX_PKT_LIMIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RX_INTR_THRESHOLD` reader - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
pub type RX_INTR_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_INTR_THRESHOLD` writer - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
pub type RX_INTR_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CONFIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `MD_BIT_CLEAR` reader - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub type MD_BIT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `MD_BIT_CLEAR` writer - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
pub type MD_BIT_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
#[doc = "Field `DSM_SLOT_VARIANCE` reader - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
pub type DSM_SLOT_VARIANCE_R = crate::BitReader<bool>;
#[doc = "Field `DSM_SLOT_VARIANCE` writer - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
pub type DSM_SLOT_VARIANCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SLV_MD_CONFIG` reader - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
pub type SLV_MD_CONFIG_R = crate::BitReader<bool>;
#[doc = "Field `SLV_MD_CONFIG` writer - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
pub type SLV_MD_CONFIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
#[doc = "Field `EXTEND_CU_TX_WIN` reader - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
pub type EXTEND_CU_TX_WIN_R = crate::BitReader<bool>;
#[doc = "Field `EXTEND_CU_TX_WIN` writer - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
pub type EXTEND_CU_TX_WIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
#[doc = "Field `MASK_SUTO_AT_UPDT` reader - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
pub type MASK_SUTO_AT_UPDT_R = crate::BitReader<bool>;
#[doc = "Field `MASK_SUTO_AT_UPDT` writer - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
pub type MASK_SUTO_AT_UPDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
#[doc = "Field `CONN_REQ_1SLOT_EARLY` reader - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
pub type CONN_REQ_1SLOT_EARLY_R = crate::BitReader<bool>;
#[doc = "Field `CONN_REQ_1SLOT_EARLY` writer - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
pub type CONN_REQ_1SLOT_EARLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_CONFIG_SPEC, bool, O>;
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
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&self) -> MD_BIT_CLEAR_R {
        MD_BIT_CLEAR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&self) -> DSM_SLOT_VARIANCE_R {
        DSM_SLOT_VARIANCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&self) -> SLV_MD_CONFIG_R {
        SLV_MD_CONFIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&self) -> EXTEND_CU_TX_WIN_R {
        EXTEND_CU_TX_WIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&self) -> MASK_SUTO_AT_UPDT_R {
        MASK_SUTO_AT_UPDT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&self) -> CONN_REQ_1SLOT_EARLY_R {
        CONN_REQ_1SLOT_EARLY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines a limit for the number of Rx packets that can be re-ceived by the LLH. Default maximum value is 0xF.Minimum value shall be '1' or no packet will be stored in the Rx FIFO."]
    #[inline(always)]
    pub fn rx_pkt_limit(&mut self) -> RX_PKT_LIMIT_W<0> {
        RX_PKT_LIMIT_W::new(self)
    }
    #[doc = "Bits 4:7 - This register field allows setting a threshold for the packet received interrupt to the firmware. For example if the value programmed is 0x2 - then HW will generate interrupt only on receiving the second packet. In any case if the received number of packets in a conn event is less than the threshold or there are still packets (less than threshold) pending in the Rx FIFO, HW will generate the interrupt at the ce_close. Min value possible is 1. Max value depends on the Rx FIFO capacity."]
    #[inline(always)]
    pub fn rx_intr_threshold(&mut self) -> RX_INTR_THRESHOLD_W<4> {
        RX_INTR_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 8 - This register field indicates whether the MD (More Data) bit needs to be controlled by 'software' or, 'hardware and soft-ware logic combined'. 1 - MD bit is exclusively controlled by software, ie based on status of CE_CNFG_STS_REGISTER\\[6\\]
- md bit. 0 - MD Bit in the transmitted pdu is controlled by software and hardware logic. MD bit is set in transmitted packet, only if the software has set the md bit in CE_CNFG_STS_REGISTER\\[6\\]
and either of the following conditions is true, a) If there are packets queued for transmission. b) If there is an acknowledgement awaited from the remote side for the packet transmitted."]
    #[inline(always)]
    pub fn md_bit_clear(&mut self) -> MD_BIT_CLEAR_W<8> {
        MD_BIT_CLEAR_W::new(self)
    }
    #[doc = "Bit 11 - This bit configures the DSM slot counting mode. 0 - The DSM slot count variance with respect to actual time is less than 1 slot 1 - The DSM slot count variance with respect to actual time is more than 1 slot &less that 2 slots"]
    #[inline(always)]
    pub fn dsm_slot_variance(&mut self) -> DSM_SLOT_VARIANCE_W<11> {
        DSM_SLOT_VARIANCE_W::new(self)
    }
    #[doc = "Bit 12 - This bit is set to configure the MD bit control when IUT is in slave role. 1 - MD bit will be decided on packet pending status 0 - MD bit will be decided on packet queued in next buffer status This bit has effect only when 'CONN_CONFIG.md_bit_ctr' bit is not set ."]
    #[inline(always)]
    pub fn slv_md_config(&mut self) -> SLV_MD_CONFIG_W<12> {
        SLV_MD_CONFIG_W::new(self)
    }
    #[doc = "Bit 13 - This bit is used to enable/disable extending the additional rx window on slave side during connection update in event of packet miss at the update instant. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn extend_cu_tx_win(&mut self) -> EXTEND_CU_TX_WIN_W<13> {
        EXTEND_CU_TX_WIN_W::new(self)
    }
    #[doc = "Bit 14 - This bit is used to enable/disable masking of internal hardware supervision timeout trigger when switching from old connection parameters to new parameters. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn mask_suto_at_updt(&mut self) -> MASK_SUTO_AT_UPDT_W<14> {
        MASK_SUTO_AT_UPDT_W::new(self)
    }
    #[doc = "Bit 15 - This bit is used to enable extension of the Conn Request to arbiter to 1 slot early. When enabled the request length is 2 slots. 1 - Enable 0 - Disable"]
    #[inline(always)]
    pub fn conn_req_1slot_early(&mut self) -> CONN_REQ_1SLOT_EARLY_W<15> {
        CONN_REQ_1SLOT_EARLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_config](index.html) module"]
pub struct CONN_CONFIG_SPEC;
impl crate::RegisterSpec for CONN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_config::R](R) reader structure"]
impl crate::Readable for CONN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_config::W](W) writer structure"]
impl crate::Writable for CONN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CONFIG to value 0xe11f"]
impl crate::Resettable for CONN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe11f
    }
}
