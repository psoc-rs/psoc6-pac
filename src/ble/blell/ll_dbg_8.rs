#[doc = "Reader of register LL_DBG_8"]
pub type R = crate::R<u32, super::LL_DBG_8>;
#[doc = "Reader of field `ADV_RX_WR_PTR_STORE`"]
pub type ADV_RX_WR_PTR_STORE_R = crate::R<u8, u8>;
#[doc = "Reader of field `WLF_PTR`"]
pub type WLF_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Advertiser Receive FIFO stored write pointer for pointer restore"]
    #[inline(always)]
    pub fn adv_rx_wr_ptr_store(&self) -> ADV_RX_WR_PTR_STORE_R {
        ADV_RX_WR_PTR_STORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Whitelist FIFO pointer"]
    #[inline(always)]
    pub fn wlf_ptr(&self) -> WLF_PTR_R {
        WLF_PTR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
