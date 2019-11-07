#[doc = "Reader of register TX_RX_SYNTH_DELAY"]
pub type R = crate::R<u32, super::TX_RX_SYNTH_DELAY>;
#[doc = "Writer for register TX_RX_SYNTH_DELAY"]
pub type W = crate::W<u32, super::TX_RX_SYNTH_DELAY>;
#[doc = "Register TX_RX_SYNTH_DELAY `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_RX_SYNTH_DELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_EN_DELAY`"]
pub type RX_EN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_EN_DELAY`"]
pub struct RX_EN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TX_EN_DELAY`"]
pub type TX_EN_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_EN_DELAY`"]
pub struct TX_EN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\] = rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&self) -> RX_EN_DELAY_R {
        RX_EN_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\] = tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&self) -> TX_EN_DELAY_R {
        TX_EN_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\] = rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&mut self) -> RX_EN_DELAY_W {
        RX_EN_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\] = tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\] = TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&mut self) -> TX_EN_DELAY_W {
        TX_EN_DELAY_W { w: self }
    }
}
