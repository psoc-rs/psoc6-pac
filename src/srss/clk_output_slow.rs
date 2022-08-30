#[doc = "Register `CLK_OUTPUT_SLOW` reader"]
pub struct R(crate::R<CLK_OUTPUT_SLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_OUTPUT_SLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_OUTPUT_SLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_OUTPUT_SLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_OUTPUT_SLOW` writer"]
pub struct W(crate::W<CLK_OUTPUT_SLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_OUTPUT_SLOW_SPEC>;
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
impl From<crate::W<CLK_OUTPUT_SLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_OUTPUT_SLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select signal for slow clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOW_SEL0_A {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
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
    #[doc = "6: Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
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
#[doc = "Field `SLOW_SEL0` reader - Select signal for slow clock output #0"]
pub type SLOW_SEL0_R = crate::FieldReader<u8, SLOW_SEL0_A>;
impl SLOW_SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOW_SEL0_A> {
        match self.bits {
            0 => Some(SLOW_SEL0_A::NC),
            1 => Some(SLOW_SEL0_A::ILO),
            2 => Some(SLOW_SEL0_A::WCO),
            3 => Some(SLOW_SEL0_A::BAK),
            4 => Some(SLOW_SEL0_A::ALTLF),
            5 => Some(SLOW_SEL0_A::LFCLK),
            6 => Some(SLOW_SEL0_A::IMO),
            7 => Some(SLOW_SEL0_A::SLPCTRL),
            8 => Some(SLOW_SEL0_A::PILO),
            _ => None,
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
#[doc = "Field `SLOW_SEL0` writer - Select signal for slow clock output #0"]
pub type SLOW_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_SLOW_SPEC, u8, SLOW_SEL0_A, 4, O>;
impl<'a, const O: u8> SLOW_SEL0_W<'a, O> {
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
}
#[doc = "Select signal for slow clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOW_SEL1_A {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
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
    #[doc = "6: Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    IMO = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
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
#[doc = "Field `SLOW_SEL1` reader - Select signal for slow clock output #1"]
pub type SLOW_SEL1_R = crate::FieldReader<u8, SLOW_SEL1_A>;
impl SLOW_SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOW_SEL1_A> {
        match self.bits {
            0 => Some(SLOW_SEL1_A::NC),
            1 => Some(SLOW_SEL1_A::ILO),
            2 => Some(SLOW_SEL1_A::WCO),
            3 => Some(SLOW_SEL1_A::BAK),
            4 => Some(SLOW_SEL1_A::ALTLF),
            5 => Some(SLOW_SEL1_A::LFCLK),
            6 => Some(SLOW_SEL1_A::IMO),
            7 => Some(SLOW_SEL1_A::SLPCTRL),
            8 => Some(SLOW_SEL1_A::PILO),
            _ => None,
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
#[doc = "Field `SLOW_SEL1` writer - Select signal for slow clock output #1"]
pub type SLOW_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_SLOW_SPEC, u8, SLOW_SEL1_A, 4, O>;
impl<'a, const O: u8> SLOW_SEL1_W<'a, O> {
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
    pub fn slow_sel0(&mut self) -> SLOW_SEL0_W<0> {
        SLOW_SEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Select signal for slow clock output #1"]
    #[inline(always)]
    pub fn slow_sel1(&mut self) -> SLOW_SEL1_W<4> {
        SLOW_SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow Clock Output Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_output_slow](index.html) module"]
pub struct CLK_OUTPUT_SLOW_SPEC;
impl crate::RegisterSpec for CLK_OUTPUT_SLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_output_slow::R](R) reader structure"]
impl crate::Readable for CLK_OUTPUT_SLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_output_slow::W](W) writer structure"]
impl crate::Writable for CLK_OUTPUT_SLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_OUTPUT_SLOW to value 0"]
impl crate::Resettable for CLK_OUTPUT_SLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
