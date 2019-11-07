#[doc = "Reader of register LL_DBG_7"]
pub type R = crate::R<u32, super::LL_DBG_7>;
#[doc = "Reader of field `ADV_RX_WR_PTR`"]
pub type ADV_RX_WR_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADV_RX_RD_PTR`"]
pub type ADV_RX_RD_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Advertiser Receive FIFO write pointer"]
    #[inline(always)]
    pub fn adv_rx_wr_ptr(&self) -> ADV_RX_WR_PTR_R {
        ADV_RX_WR_PTR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Advertiser Receive FIFO read pointer"]
    #[inline(always)]
    pub fn adv_rx_rd_ptr(&self) -> ADV_RX_RD_PTR_R {
        ADV_RX_RD_PTR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
