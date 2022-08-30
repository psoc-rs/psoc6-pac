#[doc = "Register `TXRX_HOP` reader"]
pub struct R(crate::R<TXRX_HOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXRX_HOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXRX_HOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXRX_HOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOP_CH_TX` reader - Transmit channel index. Channel index on which previous packet is transmitted."]
pub type HOP_CH_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOP_CH_RX` reader - Receive channel index. Channel index on which previous packet is received."]
pub type HOP_CH_RX_R = crate::FieldReader<u8, u8>;
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
#[doc = "Channel Address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrx_hop](index.html) module"]
pub struct TXRX_HOP_SPEC;
impl crate::RegisterSpec for TXRX_HOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txrx_hop::R](R) reader structure"]
impl crate::Readable for TXRX_HOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXRX_HOP to value 0"]
impl crate::Resettable for TXRX_HOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
