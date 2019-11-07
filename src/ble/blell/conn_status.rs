#[doc = "Reader of register CONN_STATUS"]
pub type R = crate::R<u32, super::CONN_STATUS>;
#[doc = "Reader of field `RECEIVE_PACKET_COUNT`"]
pub type RECEIVE_PACKET_COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 12:15 - This field stores the count for the number of receive packets in the receive FIFO that are still not ready by firmware. The counter value is incremented by hardware for every good packet it stores in the FIFO. After firmware reads a packet, it decrements the counter by issuing the PACKET_RECEIVED command from the commander."]
    #[inline(always)]
    pub fn receive_packet_count(&self) -> RECEIVE_PACKET_COUNT_R {
        RECEIVE_PACKET_COUNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
