#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_ENABLED`"]
pub type TX_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ENABLED`"]
pub struct TX_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RX_ENABLED`"]
pub type RX_ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ENABLED`"]
pub struct RX_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&self) -> TX_ENABLED_R {
        TX_ENABLED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&self) -> RX_ENABLED_R {
        RX_ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&mut self) -> TX_ENABLED_W {
        TX_ENABLED_W { w: self }
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&mut self) -> RX_ENABLED_W {
        RX_ENABLED_W { w: self }
    }
}
