#[doc = "Reader of register MCWDT_LOCK"]
pub type R = crate::R<u32, super::MCWDT_LOCK>;
#[doc = "Writer for register MCWDT_LOCK"]
pub type W = crate::W<u32, super::MCWDT_LOCK>;
#[doc = "Register MCWDT_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCWDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<MCWDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MCWDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCWDT_LOCK`"]
pub type MCWDT_LOCK_R = crate::R<u8, MCWDT_LOCK_A>;
impl MCWDT_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCWDT_LOCK_A {
        match self.bits {
            0 => MCWDT_LOCK_A::NO_CHG,
            1 => MCWDT_LOCK_A::CLR0,
            2 => MCWDT_LOCK_A::CLR1,
            3 => MCWDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHG`"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == MCWDT_LOCK_A::NO_CHG
    }
    #[doc = "Checks if the value of the field is `CLR0`"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR0
    }
    #[doc = "Checks if the value of the field is `CLR1`"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == MCWDT_LOCK_A::CLR1
    }
    #[doc = "Checks if the value of the field is `SET01`"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == MCWDT_LOCK_A::SET01
    }
}
#[doc = "Write proxy for field `MCWDT_LOCK`"]
pub struct MCWDT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCWDT_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCWDT_LOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut W {
        self.variant(MCWDT_LOCK_A::SET01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&self) -> MCWDT_LOCK_R {
        MCWDT_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Prohibits writing control and configuration registers related to this MCWDT when not equal 0 (as specified in the other register descriptions). Requires at least two different writes to unlock. Note that this field is 2 bits to force multiple writes only. Each MCWDT has a separate local lock. LFCLK settings are locked by the global WDT_LOCK register, and this register has no effect on that."]
    #[inline(always)]
    pub fn mcwdt_lock(&mut self) -> MCWDT_LOCK_W {
        MCWDT_LOCK_W { w: self }
    }
}
