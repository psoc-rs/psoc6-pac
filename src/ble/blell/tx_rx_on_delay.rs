#[doc = "Reader of register TX_RX_ON_DELAY"]
pub type R = crate::R<u32, super::TX_RX_ON_DELAY>;
#[doc = "Writer for register TX_RX_ON_DELAY"]
pub type W = crate::W<u32, super::TX_RX_ON_DELAY>;
#[doc = "Register TX_RX_ON_DELAY `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_RX_ON_DELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXON_DELAY`"]
pub type RXON_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXON_DELAY`"]
pub struct RXON_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXON_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TXON_DELAY`"]
pub type TXON_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXON_DELAY`"]
pub struct TXON_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXON_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
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
    pub fn rxon_delay(&mut self) -> RXON_DELAY_W {
        RXON_DELAY_W { w: self }
    }
    #[doc = "Bits 8:15 - Transmit delay - Delay from start of transmit to transmission of first bit on air. It is used to control the T_IFS. The delay is in resolution of 1 microsecond."]
    #[inline(always)]
    pub fn txon_delay(&mut self) -> TXON_DELAY_W {
        TXON_DELAY_W { w: self }
    }
}
