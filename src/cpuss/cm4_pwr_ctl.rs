#[doc = "Reader of register CM4_PWR_CTL"]
pub type R = crate::R<u32, super::CM4_PWR_CTL>;
#[doc = "Writer for register CM4_PWR_CTL"]
pub type W = crate::W<u32, super::CM4_PWR_CTL>;
#[doc = "Register CM4_PWR_CTL `reset()`'s with value 0xfa05_0001"]
impl crate::ResetValue for super::CM4_PWR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0001
    }
}
#[doc = "Set Power mode for CM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: Switch CM4 off\nPower off, clock off, isolate, reset and no retain."]
    OFF = 0,
    #[doc = "1: Reset CM4\nClock off, no isolated, no retain and reset.\n\nNote: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    RESET = 1,
    #[doc = "2: Put CM4 in Retained mode\nThis can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached.\nPower off, clock off, isolate, no reset and retain."]
    RETAINED = 2,
    #[doc = "3: Switch CM4 on.\nPower on, clock on, no isolate, no reset and no retain."]
    ENABLED = 3,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWR_MODE`"]
pub type PWR_MODE_R = crate::R<u8, PWR_MODE_A>;
impl PWR_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::OFF,
            1 => PWR_MODE_A::RESET,
            2 => PWR_MODE_A::RETAINED,
            3 => PWR_MODE_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PWR_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `RETAINED`"]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PWR_MODE_A::RETAINED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWR_MODE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWR_MODE`"]
pub struct PWR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PWR_MODE_A::OFF)
    }
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RESET)
    }
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut W {
        self.variant(PWR_MODE_A::RETAINED)
    }
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWR_MODE_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `VECTKEYSTAT`"]
pub type VECTKEYSTAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Set Power mode for CM4"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VECTKEYSTAT_R {
        VECTKEYSTAT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set Power mode for CM4"]
    #[inline(always)]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W {
        PWR_MODE_W { w: self }
    }
}
