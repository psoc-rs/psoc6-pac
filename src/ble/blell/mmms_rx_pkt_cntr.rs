#[doc = "Reader of register MMMS_RX_PKT_CNTR"]
pub type R = crate::R<u32, super::MMMS_RX_PKT_CNTR>;
#[doc = "Reader of field `MMMS_RX_PKT_CNT`"]
pub type MMMS_RX_PKT_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Count of all packets in the RX FIFO in MMMS mode"]
    #[inline(always)]
    pub fn mmms_rx_pkt_cnt(&self) -> MMMS_RX_PKT_CNT_R {
        MMMS_RX_PKT_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
