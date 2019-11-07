#[doc = "Reader of register CONN_RX_PKT_CNTR[%s]"]
pub type R = crate::R<u32, super::CONN_RX_PKT_CNTR>;
#[doc = "Reader of field `RX_PKT_CNT`"]
pub type RX_PKT_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Number of packets received for the connection. Incremented when the packet is received during the connection event and decremented when firmware has processed the packet. The register field FW_PKT_RCV_CONN_INDEX should be programmed before firmware issues the packet received command"]
    #[inline(always)]
    pub fn rx_pkt_cnt(&self) -> RX_PKT_CNT_R {
        RX_PKT_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
