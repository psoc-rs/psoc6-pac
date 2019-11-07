#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_START`"]
pub type TX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_START`"]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TX_PAUSE`"]
pub type TX_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_PAUSE`"]
pub struct TX_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PAUSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RX_START`"]
pub type RX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_START`"]
pub struct RX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&self) -> TX_PAUSE_R {
        TX_PAUSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&mut self) -> TX_PAUSE_W {
        TX_PAUSE_W { w: self }
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W { w: self }
    }
}
