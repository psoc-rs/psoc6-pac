#[doc = "Register `DTM_RX_PKT_COUNT` reader"]
pub struct R(crate::R<DTM_RX_PKT_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTM_RX_PKT_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTM_RX_PKT_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTM_RX_PKT_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_PACKET_COUNT` reader - Number of packets received in receive test mode."]
pub type RX_PACKET_COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of packets received in receive test mode."]
    #[inline(always)]
    pub fn rx_packet_count(&self) -> RX_PACKET_COUNT_R {
        RX_PACKET_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Direct Test Mode receive packet count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtm_rx_pkt_count](index.html) module"]
pub struct DTM_RX_PKT_COUNT_SPEC;
impl crate::RegisterSpec for DTM_RX_PKT_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtm_rx_pkt_count::R](R) reader structure"]
impl crate::Readable for DTM_RX_PKT_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTM_RX_PKT_COUNT to value 0"]
impl crate::Resettable for DTM_RX_PKT_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
