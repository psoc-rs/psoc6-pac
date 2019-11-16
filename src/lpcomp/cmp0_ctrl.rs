#[doc = "Reader of register CMP0_CTRL"]
pub type R = crate::R<u32, super::CMP0_CTRL>;
#[doc = "Writer for register CMP0_CTRL"]
pub type W = crate::W<u32, super::CMP0_CTRL>;
#[doc = "Register CMP0_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP0_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 1,
    #[doc = "2: Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 2,
    #[doc = "3: Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE0`"]
pub type MODE0_R = crate::R<u8, MODE0_A>;
impl MODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::OFF,
            1 => MODE0_A::ULP,
            2 => MODE0_A::LP,
            3 => MODE0_A::NORMAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MODE0_A::OFF
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE0_A::ULP
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == MODE0_A::LP
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE0_A::NORMAL
    }
}
#[doc = "Write proxy for field `MODE0`"]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE0_A::OFF)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE0_A::ULP)
    }
    #[doc = "Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(MODE0_A::LP)
    }
    #[doc = "Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE0_A::NORMAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `HYST0`"]
pub type HYST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYST0`"]
pub struct HYST0_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE0_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTTYPE0`"]
pub type INTTYPE0_R = crate::R<u8, INTTYPE0_A>;
impl INTTYPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE0_A {
        match self.bits {
            0 => INTTYPE0_A::DISABLE,
            1 => INTTYPE0_A::RISING,
            2 => INTTYPE0_A::FALLING,
            3 => INTTYPE0_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTTYPE0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTTYPE0_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTTYPE0_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTTYPE0_A::BOTH
    }
}
#[doc = "Write proxy for field `INTTYPE0`"]
pub struct INTTYPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTTYPE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTTYPE0_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTTYPE0_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTTYPE0_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTTYPE0_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DSI_BYPASS0`"]
pub type DSI_BYPASS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_BYPASS0`"]
pub struct DSI_BYPASS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_BYPASS0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DSI_LEVEL0`"]
pub type DSI_LEVEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_LEVEL0`"]
pub struct DSI_LEVEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_LEVEL0_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst0(&self) -> HYST0_R {
        HYST0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype0(&self) -> INTTYPE0_R {
        INTTYPE0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass0(&self) -> DSI_BYPASS0_R {
        DSI_BYPASS0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level0(&self) -> DSI_LEVEL0_R {
        DSI_LEVEL0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst0(&mut self) -> HYST0_W {
        HYST0_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype0(&mut self) -> INTTYPE0_W {
        INTTYPE0_W { w: self }
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass0(&mut self) -> DSI_BYPASS0_W {
        DSI_BYPASS0_W { w: self }
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level0(&mut self) -> DSI_LEVEL0_W {
        DSI_LEVEL0_W { w: self }
    }
}
