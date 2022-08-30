#[doc = "Register `MMMS_RX_PKT_CNTR` reader"]
pub struct R(crate::R<MMMS_RX_PKT_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_RX_PKT_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_RX_PKT_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_RX_PKT_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MMMS_RX_PKT_CNT` reader - Count of all packets in the RX FIFO in MMMS mode"]
pub type MMMS_RX_PKT_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Count of all packets in the RX FIFO in MMMS mode"]
    #[inline(always)]
    pub fn mmms_rx_pkt_cnt(&self) -> MMMS_RX_PKT_CNT_R {
        MMMS_RX_PKT_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Packet Counter of packets in RX FIFO in MMMS mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_rx_pkt_cntr](index.html) module"]
pub struct MMMS_RX_PKT_CNTR_SPEC;
impl crate::RegisterSpec for MMMS_RX_PKT_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_rx_pkt_cntr::R](R) reader structure"]
impl crate::Readable for MMMS_RX_PKT_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMMS_RX_PKT_CNTR to value 0"]
impl crate::Resettable for MMMS_RX_PKT_CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
