#[doc = "Reader of register LL_DBG_3"]
pub type R = crate::R<u32, super::LL_DBG_3>;
#[doc = "Reader of field `CONN_RX_WR_PTR_STORE`"]
pub type CONN_RX_WR_PTR_STORE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Connection receive FIFO stored write pointer for pointer restore"]
    #[inline(always)]
    pub fn conn_rx_wr_ptr_store(&self) -> CONN_RX_WR_PTR_STORE_R {
        CONN_RX_WR_PTR_STORE_R::new((self.bits & 0x03ff) as u16)
    }
}
