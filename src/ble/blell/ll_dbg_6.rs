#[doc = "Reader of register LL_DBG_6"]
pub type R = crate::R<u32, super::LL_DBG_6>;
#[doc = "Reader of field `ADV_TX_WR_PTR`"]
pub type ADV_TX_WR_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCAN_RSP_TX_WR_PTR`"]
pub type SCAN_RSP_TX_WR_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADV_TX_RD_PTR`"]
pub type ADV_TX_RD_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Advertiser Transmit FIFO write pointer"]
    #[inline(always)]
    pub fn adv_tx_wr_ptr(&self) -> ADV_TX_WR_PTR_R {
        ADV_TX_WR_PTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Scan Response Transmit FIFO write pointer"]
    #[inline(always)]
    pub fn scan_rsp_tx_wr_ptr(&self) -> SCAN_RSP_TX_WR_PTR_R {
        SCAN_RSP_TX_WR_PTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Advertiser/ Scan Response FIFO read pointer"]
    #[inline(always)]
    pub fn adv_tx_rd_ptr(&self) -> ADV_TX_RD_PTR_R {
        ADV_TX_RD_PTR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
