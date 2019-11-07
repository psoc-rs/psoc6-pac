#[doc = "Reader of register TXRX_HOP"]
pub type R = crate::R<u32, super::TXRX_HOP>;
#[doc = "Reader of field `HOP_CH_TX`"]
pub type HOP_CH_TX_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOP_CH_RX`"]
pub type HOP_CH_RX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Transmit channel index. Channel index on which previous packet is transmitted."]
    #[inline(always)]
    pub fn hop_ch_tx(&self) -> HOP_CH_TX_R {
        HOP_CH_TX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Receive channel index. Channel index on which previous packet is received."]
    #[inline(always)]
    pub fn hop_ch_rx(&self) -> HOP_CH_RX_R {
        HOP_CH_RX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
