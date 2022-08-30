#[doc = "Register `TX_RX_SYNTH_DELAY` reader"]
pub struct R(crate::R<TX_RX_SYNTH_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RX_SYNTH_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RX_SYNTH_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RX_SYNTH_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_RX_SYNTH_DELAY` writer"]
pub struct W(crate::W<TX_RX_SYNTH_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_RX_SYNTH_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TX_RX_SYNTH_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_RX_SYNTH_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN_DELAY` reader - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
pub type RX_EN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_EN_DELAY` writer - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
pub type RX_EN_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_RX_SYNTH_DELAY_SPEC, u8, u8, 8, O>;
#[doc = "Field `TX_EN_DELAY` reader - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
pub type TX_EN_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_EN_DELAY` writer - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
pub type TX_EN_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_RX_SYNTH_DELAY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&self) -> RX_EN_DELAY_R {
        RX_EN_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&self) -> TX_EN_DELAY_R {
        TX_EN_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The delay used to assert rif_rx_en, Rx_tRamp micro-seconds, ahead of first bit of the expected rx_data, which can be used to turn on the Radio receiver. The value to be programmed to the Rx_en_delay \\[7:0\\]
= rx_on_delay - Rx_tRamp rx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[7:0\\]) Rx_tRamp = Radio receiver rampup time"]
    #[inline(always)]
    pub fn rx_en_delay(&mut self) -> RX_EN_DELAY_W<0> {
        RX_EN_DELAY_W::new(self)
    }
    #[doc = "Bits 8:15 - The delay used to assert rif_tx_en exactly Tx_tRamp micro-seconds ahead of the first bit of the tx_data, which can be used to turn on the Radio transmitter. The value to be programmed to the Tx_en_delay \\[7:0\\]
= tx_on_delay - Tx_tRamp tx_on_delay\\[7:0\\]
= TX_RX_ON_DELAY\\[15:8\\]) Tx_tRamp = Radio transmitter ramp_up"]
    #[inline(always)]
    pub fn tx_en_delay(&mut self) -> TX_EN_DELAY_W<8> {
        TX_EN_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit/Receive enable delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_synth_delay](index.html) module"]
pub struct TX_RX_SYNTH_DELAY_SPEC;
impl crate::RegisterSpec for TX_RX_SYNTH_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rx_synth_delay::R](R) reader structure"]
impl crate::Readable for TX_RX_SYNTH_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_rx_synth_delay::W](W) writer structure"]
impl crate::Writable for TX_RX_SYNTH_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_RX_SYNTH_DELAY to value 0"]
impl crate::Resettable for TX_RX_SYNTH_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
