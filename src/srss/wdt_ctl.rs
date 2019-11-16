#[doc = "Reader of register WDT_CTL"]
pub type R = crate::R<u32, super::WDT_CTL>;
#[doc = "Writer for register WDT_CTL"]
pub type W = crate::W<u32, super::WDT_CTL>;
#[doc = "Register WDT_CTL `reset()`'s with value 0xc000_0001"]
impl crate::ResetValue for super::WDT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0001
    }
}
#[doc = "Reader of field `WDT_EN`"]
pub type WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_EN`"]
pub struct WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EN_W<'a> {
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
#[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<WDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDT_LOCK`"]
pub type WDT_LOCK_R = crate::R<u8, WDT_LOCK_A>;
impl WDT_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_LOCK_A {
        match self.bits {
            0 => WDT_LOCK_A::NO_CHG,
            1 => WDT_LOCK_A::CLR0,
            2 => WDT_LOCK_A::CLR1,
            3 => WDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHG`"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == WDT_LOCK_A::NO_CHG
    }
    #[doc = "Checks if the value of the field is `CLR0`"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == WDT_LOCK_A::CLR0
    }
    #[doc = "Checks if the value of the field is `CLR1`"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == WDT_LOCK_A::CLR1
    }
    #[doc = "Checks if the value of the field is `SET01`"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == WDT_LOCK_A::SET01
    }
}
#[doc = "Write proxy for field `WDT_LOCK`"]
pub struct WDT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_LOCK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::SET01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W {
        WDT_LOCK_W { w: self }
    }
}
