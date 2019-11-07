#[doc = "Reader of register LL_DBG_1"]
pub type R = crate::R<u32, super::LL_DBG_1>;
#[doc = "Reader of field `CONN_RX_WR_PTR`"]
pub type CONN_RX_WR_PTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Connection receive FIFO write pointer"]
    #[inline(always)]
    pub fn conn_rx_wr_ptr(&self) -> CONN_RX_WR_PTR_R {
        CONN_RX_WR_PTR_R::new((self.bits & 0x03ff) as u16)
    }
}
