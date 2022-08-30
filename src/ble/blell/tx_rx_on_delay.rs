#[doc = "Register `TX_RX_ON_DELAY` reader"]
pub struct R(crate::R<TX_RX_ON_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RX_ON_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RX_ON_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RX_ON_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_RX_ON_DELAY` writer"]
pub struct W(crate::W<TX_RX_ON_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_RX_ON_DELAY_SPEC>;
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
impl From<crate::W<TX_RX_ON_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_RX_ON_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXON_DELAY` reader - Receive delay - Delay from start of receive to expected first bit of receive packet at the controller. Used to control the turn on time of radio to optimize on power. The delay is in resolution of 1 microsecond."]
pub type RXON_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXON_DELAY` writer - Receive delay - Delay from start of receive to expected first bit of receive packet at the controller. Used to control the turn on time of radio to optimize on power. The delay is in resolution of 1 microsecond."]
pub type RXON_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_RX_ON_DELAY_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXON_DELAY` reader - Transmit delay - Delay from start of transmit to transmission of first bit on air. It is used to control the T_IFS. The delay is in resolution of 1 microsecond."]
pub type TXON_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXON_DELAY` writer - Transmit delay - Delay from start of transmit to transmission of first bit on air. It is used to control the T_IFS. The delay is in resolution of 1 microsecond."]
pub type TXON_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_RX_ON_DELAY_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive delay - Delay from start of receive to expected first bit of receive packet at the controller. Used to control the turn on time of radio to optimize on power. The delay is in resolution of 1 microsecond."]
    #[inline(always)]
    pub fn rxon_delay(&self) -> RXON_DELAY_R {
        RXON_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit delay - Delay from start of transmit to transmission of first bit on air. It is used to control the T_IFS. The delay is in resolution of 1 microsecond."]
    #[inline(always)]
    pub fn txon_delay(&self) -> TXON_DELAY_R {
        TXON_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive delay - Delay from start of receive to expected first bit of receive packet at the controller. Used to control the turn on time of radio to optimize on power. The delay is in resolution of 1 microsecond."]
    #[inline(always)]
    pub fn rxon_delay(&mut self) -> RXON_DELAY_W<0> {
        RXON_DELAY_W::new(self)
    }
    #[doc = "Bits 8:15 - Transmit delay - Delay from start of transmit to transmission of first bit on air. It is used to control the T_IFS. The delay is in resolution of 1 microsecond."]
    #[inline(always)]
    pub fn txon_delay(&mut self) -> TXON_DELAY_W<8> {
        TXON_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit/Receive data delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_on_delay](index.html) module"]
pub struct TX_RX_ON_DELAY_SPEC;
impl crate::RegisterSpec for TX_RX_ON_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rx_on_delay::R](R) reader structure"]
impl crate::Readable for TX_RX_ON_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_rx_on_delay::W](W) writer structure"]
impl crate::Writable for TX_RX_ON_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_RX_ON_DELAY to value 0"]
impl crate::Resettable for TX_RX_ON_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
