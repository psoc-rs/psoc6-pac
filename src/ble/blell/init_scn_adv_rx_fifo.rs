#[doc = "Register `INIT_SCN_ADV_RX_FIFO` reader"]
pub struct R(crate::R<INIT_SCN_ADV_RX_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_SCN_ADV_RX_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_SCN_ADV_RX_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_SCN_ADV_RX_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADV_SCAN_RSP_RX_DATA` reader - IO mapped FIFO of depth 64, to store ADV and SCAN_RSP header and payload received by the scanner. The RSSI value at the time of reception of this packet is also stored. Firmware reads from the same address to read out consecutive words of data. Note: The 16 bit header is first loaded to the advertise channel data receive FIFO followed by the payload data and then 16 bit RSSI."]
pub type ADV_SCAN_RSP_RX_DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 64, to store ADV and SCAN_RSP header and payload received by the scanner. The RSSI value at the time of reception of this packet is also stored. Firmware reads from the same address to read out consecutive words of data. Note: The 16 bit header is first loaded to the advertise channel data receive FIFO followed by the payload data and then 16 bit RSSI."]
    #[inline(always)]
    pub fn adv_scan_rsp_rx_data(&self) -> ADV_SCAN_RSP_RX_DATA_R {
        ADV_SCAN_RSP_RX_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "advertising scan response data receive data FIFO. Access ADVRX_FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_scn_adv_rx_fifo](index.html) module"]
pub struct INIT_SCN_ADV_RX_FIFO_SPEC;
impl crate::RegisterSpec for INIT_SCN_ADV_RX_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_scn_adv_rx_fifo::R](R) reader structure"]
impl crate::Readable for INIT_SCN_ADV_RX_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INIT_SCN_ADV_RX_FIFO to value 0"]
impl crate::Resettable for INIT_SCN_ADV_RX_FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
