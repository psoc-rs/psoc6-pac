#[doc = "Reader of register CLK_OUTPUT_SLOW"]
pub type R = crate::R<u32, super::CLK_OUTPUT_SLOW>;
#[doc = "Writer for register CLK_OUTPUT_SLOW"]
pub type W = crate::W<u32, super::CLK_OUTPUT_SLOW>;
#[doc = "Register CLK_OUTPUT_SLOW `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_OUTPUT_SLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select signal for slow clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOW_SEL0_A {
    #[doc = "0: Disabled - output is 0.  For power savings, clocks are blocked before entering any muxes."]
    NC = 0,
    #[doc = "1: Internal Low Speed Oscillator (ILO)"]
    ILO = 1,
    #[doc = "2: Watch-Crystal Oscillator (WCO)"]
    WCO = 2,
    #[doc = "3: Root of the Backup domain clock tree (BAK)"]
    BAK = 3,
    #[doc = "4: Alternate low-frequency clock input to SRSS (ALTLF)"]
    ALTLF = 4,
    #[doc = "5: Root of the low-speed clock tree (LFCLK)"]
    LFCLK = 5,
    #[doc = "6: Internal Main Oscillator (IMO).  This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL).  This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    SLPCTRL = 7,
    #[doc = "8: Precision Internal Low Speed Oscillator (PILO)"]
    PILO = 8,
}
impl From<SLOW_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOW_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOW_SEL0`"]
pub type SLOW_SEL0_R = crate::R<u8, SLOW_SEL0_A>;
impl SLOW_SEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOW_SEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOW_SEL0_A::NC),
            1 => Val(SLOW_SEL0_A::ILO),
            2 => Val(SLOW_SEL0_A::WCO),
            3 => Val(SLOW_SEL0_A::BAK),
            4 => Val(SLOW_SEL0_A::ALTLF),
            5 => Val(SLOW_SEL0_A::LFCLK),
            6 => Val(SLOW_SEL0_A::IMO),
            7 => Val(SLOW_SEL0_A::SLPCTRL),
            8 => Val(SLOW_SEL0_A::PILO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == SLOW_SEL0_A::NC
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == SLOW_SEL0_A::ILO
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == SLOW_SEL0_A::WCO
    }
    #[doc = "Checks if the value of the field is `BAK`"]
    #[inline(always)]
    pub fn is_bak(&self) -> bool {
        *self == SLOW_SEL0_A::BAK
    }
    #[doc = "Checks if the value of the field is `ALTLF`"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == SLOW_SEL0_A::ALTLF
    }
    #[doc = "Checks if the value of the field is `LFCLK`"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == SLOW_SEL0_A::LFCLK
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == SLOW_SEL0_A::IMO
    }
    #[doc = "Checks if the value of the field is `SLPCTRL`"]
    #[inline(always)]
    pub fn is_slpctrl(&self) -> bool {
        *self == SLOW_SEL0_A::SLPCTRL
    }
    #[doc = "Checks if the value of the field is `PILO`"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == SLOW_SEL0_A::PILO
    }
}
#[doc = "Write proxy for field `SLOW_SEL0`"]
pub struct SLOW_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOW_SEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::NC)
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::ILO)
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::WCO)
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn bak(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::BAK)
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::ALTLF)
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::LFCLK)
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::IMO)
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn slpctrl(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::SLPCTRL)
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut W {
        self.variant(SLOW_SEL0_A::PILO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Select signal for slow clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOW_SEL1_A {
    #[doc = "0: Disabled - output is 0.  For power savings, clocks are blocked before entering any muxes."]
    NC = 0,
    #[doc = "1: Internal Low Speed Oscillator (ILO)"]
    ILO = 1,
    #[doc = "2: Watch-Crystal Oscillator (WCO)"]
    WCO = 2,
    #[doc = "3: Root of the Backup domain clock tree (BAK)"]
    BAK = 3,
    #[doc = "4: Alternate low-frequency clock input to SRSS (ALTLF)"]
    ALTLF = 4,
    #[doc = "5: Root of the low-speed clock tree (LFCLK)"]
    LFCLK = 5,
    #[doc = "6: Internal Main Oscillator (IMO).  This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL).  This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    SLPCTRL = 7,
    #[doc = "8: Precision Internal Low Speed Oscillator (PILO)"]
    PILO = 8,
}
impl From<SLOW_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOW_SEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOW_SEL1`"]
pub type SLOW_SEL1_R = crate::R<u8, SLOW_SEL1_A>;
impl SLOW_SEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOW_SEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOW_SEL1_A::NC),
            1 => Val(SLOW_SEL1_A::ILO),
            2 => Val(SLOW_SEL1_A::WCO),
            3 => Val(SLOW_SEL1_A::BAK),
            4 => Val(SLOW_SEL1_A::ALTLF),
            5 => Val(SLOW_SEL1_A::LFCLK),
            6 => Val(SLOW_SEL1_A::IMO),
            7 => Val(SLOW_SEL1_A::SLPCTRL),
            8 => Val(SLOW_SEL1_A::PILO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == SLOW_SEL1_A::NC
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == SLOW_SEL1_A::ILO
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == SLOW_SEL1_A::WCO
    }
    #[doc = "Checks if the value of the field is `BAK`"]
    #[inline(always)]
    pub fn is_bak(&self) -> bool {
        *self == SLOW_SEL1_A::BAK
    }
    #[doc = "Checks if the value of the field is `ALTLF`"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == SLOW_SEL1_A::ALTLF
    }
    #[doc = "Checks if the value of the field is `LFCLK`"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == SLOW_SEL1_A::LFCLK
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == SLOW_SEL1_A::IMO
    }
    #[doc = "Checks if the value of the field is `SLPCTRL`"]
    #[inline(always)]
    pub fn is_slpctrl(&self) -> bool {
        *self == SLOW_SEL1_A::SLPCTRL
    }
    #[doc = "Checks if the value of the field is `PILO`"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == SLOW_SEL1_A::PILO
    }
}
#[doc = "Write proxy for field `SLOW_SEL1`"]
pub struct SLOW_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOW_SEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::NC)
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::ILO)
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::WCO)
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn bak(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::BAK)
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::ALTLF)
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::LFCLK)
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::IMO)
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn slpctrl(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::SLPCTRL)
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut W {
        self.variant(SLOW_SEL1_A::PILO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Select signal for slow clock output #0"]
    #[inline(always)]
    pub fn slow_sel0(&self) -> SLOW_SEL0_R {
        SLOW_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select signal for slow clock output #1"]
    #[inline(always)]
    pub fn slow_sel1(&self) -> SLOW_SEL1_R {
        SLOW_SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for slow clock output #0"]
    #[inline(always)]
    pub fn slow_sel0(&mut self) -> SLOW_SEL0_W {
        SLOW_SEL0_W { w: self }
    }
    #[doc = "Bits 4:7 - Select signal for slow clock output #1"]
    #[inline(always)]
    pub fn slow_sel1(&mut self) -> SLOW_SEL1_W {
        SLOW_SEL1_W { w: self }
    }
}
