#[doc = "Reader of register DTM_RX_PKT_COUNT"]
pub type R = crate::R<u32, super::DTM_RX_PKT_COUNT>;
#[doc = "Reader of field `RX_PACKET_COUNT`"]
pub type RX_PACKET_COUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of packets received in receive test mode."]
    #[inline(always)]
    pub fn rx_packet_count(&self) -> RX_PACKET_COUNT_R {
        RX_PACKET_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
