#[doc = "Register `CONN_RX_PKT_CNTR[%s]` reader"]
pub struct R(crate::R<CONN_RX_PKT_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_RX_PKT_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_RX_PKT_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_RX_PKT_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_PKT_CNT` reader - Number of packets received for the connection. Incremented when the packet is received during the connection event and decremented when firmware has processed the packet. The register field FW_PKT_RCV_CONN_INDEX should be programmed before firmware issues the packet received command"]
pub type RX_PKT_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Number of packets received for the connection. Incremented when the packet is received during the connection event and decremented when firmware has processed the packet. The register field FW_PKT_RCV_CONN_INDEX should be programmed before firmware issues the packet received command"]
    #[inline(always)]
    pub fn rx_pkt_cnt(&self) -> RX_PKT_CNT_R {
        RX_PKT_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Packet Counter for Individual connection index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_rx_pkt_cntr](index.html) module"]
pub struct CONN_RX_PKT_CNTR_SPEC;
impl crate::RegisterSpec for CONN_RX_PKT_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_rx_pkt_cntr::R](R) reader structure"]
impl crate::Readable for CONN_RX_PKT_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONN_RX_PKT_CNTR[%s]
to value 0"]
impl crate::Resettable for CONN_RX_PKT_CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
