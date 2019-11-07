#[doc = "Reader of register MCWDT_CTL"]
pub type R = crate::R<u32, super::MCWDT_CTL>;
#[doc = "Writer for register MCWDT_CTL"]
pub type W = crate::W<u32, super::MCWDT_CTL>;
#[doc = "Register MCWDT_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_ENABLE0`"]
pub type WDT_ENABLE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_ENABLE0`"]
pub struct WDT_ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE0_W<'a> {
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
#[doc = "Reader of field `WDT_ENABLED0`"]
pub type WDT_ENABLED0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT_RESET0`"]
pub type WDT_RESET0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_RESET0`"]
pub struct WDT_RESET0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WDT_ENABLE1`"]
pub type WDT_ENABLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_ENABLE1`"]
pub struct WDT_ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE1_W<'a> {
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
#[doc = "Reader of field `WDT_ENABLED1`"]
pub type WDT_ENABLED1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT_RESET1`"]
pub type WDT_RESET1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_RESET1`"]
pub struct WDT_RESET1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `WDT_ENABLE2`"]
pub type WDT_ENABLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_ENABLE2`"]
pub struct WDT_ENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE2_W<'a> {
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
#[doc = "Reader of field `WDT_ENABLED2`"]
pub type WDT_ENABLED2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDT_RESET2`"]
pub type WDT_RESET2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_RESET2`"]
pub struct WDT_RESET2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&self) -> WDT_ENABLE0_R {
        WDT_ENABLE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled0(&self) -> WDT_ENABLED0_R {
        WDT_ENABLED0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&self) -> WDT_RESET0_R {
        WDT_RESET0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&self) -> WDT_ENABLE1_R {
        WDT_ENABLE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled1(&self) -> WDT_ENABLED1_R {
        WDT_ENABLED1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&self) -> WDT_RESET1_R {
        WDT_RESET1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&self) -> WDT_ENABLE2_R {
        WDT_ENABLE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled2(&self) -> WDT_ENABLED2_R {
        WDT_ENABLED2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&self) -> WDT_RESET2_R {
        WDT_RESET2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&mut self) -> WDT_ENABLE0_W {
        WDT_ENABLE0_W { w: self }
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&mut self) -> WDT_RESET0_W {
        WDT_RESET0_W { w: self }
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&mut self) -> WDT_ENABLE1_W {
        WDT_ENABLE1_W { w: self }
    }
    #[doc = "Bit 11 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&mut self) -> WDT_RESET1_W {
        WDT_RESET1_W { w: self }
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&mut self) -> WDT_ENABLE2_W {
        WDT_ENABLE2_W { w: self }
    }
    #[doc = "Bit 19 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&mut self) -> WDT_RESET2_W {
        WDT_RESET2_W { w: self }
    }
}
