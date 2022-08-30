#[doc = "Register `CLK_OUTPUT_FAST` reader"]
pub struct R(crate::R<CLK_OUTPUT_FAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_OUTPUT_FAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_OUTPUT_FAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_OUTPUT_FAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_OUTPUT_FAST` writer"]
pub struct W(crate::W<CLK_OUTPUT_FAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_OUTPUT_FAST_SPEC>;
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
impl From<crate::W<CLK_OUTPUT_FAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_OUTPUT_FAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select signal for fast clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAST_SEL0_A {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    NC = 0,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    ECO = 1,
    #[doc = "2: External clock input (EXTCLK)"]
    EXTCLK = 2,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF = 3,
    #[doc = "4: Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK = 4,
    #[doc = "5: Selects the clock path chosen by PATH_SEL0 field"]
    PATH_SEL0 = 5,
    #[doc = "6: Selects the output of the HFCLK_SEL0 mux"]
    HFCLK_SEL0 = 6,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    SLOW_SEL0 = 7,
}
impl From<FAST_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: FAST_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FAST_SEL0` reader - Select signal for fast clock output #0"]
pub type FAST_SEL0_R = crate::FieldReader<u8, FAST_SEL0_A>;
impl FAST_SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAST_SEL0_A> {
        match self.bits {
            0 => Some(FAST_SEL0_A::NC),
            1 => Some(FAST_SEL0_A::ECO),
            2 => Some(FAST_SEL0_A::EXTCLK),
            3 => Some(FAST_SEL0_A::ALTHF),
            4 => Some(FAST_SEL0_A::TIMERCLK),
            5 => Some(FAST_SEL0_A::PATH_SEL0),
            6 => Some(FAST_SEL0_A::HFCLK_SEL0),
            7 => Some(FAST_SEL0_A::SLOW_SEL0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == FAST_SEL0_A::NC
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == FAST_SEL0_A::ECO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == FAST_SEL0_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `ALTHF`"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == FAST_SEL0_A::ALTHF
    }
    #[doc = "Checks if the value of the field is `TIMERCLK`"]
    #[inline(always)]
    pub fn is_timerclk(&self) -> bool {
        *self == FAST_SEL0_A::TIMERCLK
    }
    #[doc = "Checks if the value of the field is `PATH_SEL0`"]
    #[inline(always)]
    pub fn is_path_sel0(&self) -> bool {
        *self == FAST_SEL0_A::PATH_SEL0
    }
    #[doc = "Checks if the value of the field is `HFCLK_SEL0`"]
    #[inline(always)]
    pub fn is_hfclk_sel0(&self) -> bool {
        *self == FAST_SEL0_A::HFCLK_SEL0
    }
    #[doc = "Checks if the value of the field is `SLOW_SEL0`"]
    #[inline(always)]
    pub fn is_slow_sel0(&self) -> bool {
        *self == FAST_SEL0_A::SLOW_SEL0
    }
}
#[doc = "Field `FAST_SEL0` writer - Select signal for fast clock output #0"]
pub type FAST_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, FAST_SEL0_A, 4, O>;
impl<'a, const O: u8> FAST_SEL0_W<'a, O> {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::NC)
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::ECO)
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::EXTCLK)
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::ALTHF)
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn timerclk(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::TIMERCLK)
    }
    #[doc = "Selects the clock path chosen by PATH_SEL0 field"]
    #[inline(always)]
    pub fn path_sel0(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::PATH_SEL0)
    }
    #[doc = "Selects the output of the HFCLK_SEL0 mux"]
    #[inline(always)]
    pub fn hfclk_sel0(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::HFCLK_SEL0)
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    #[inline(always)]
    pub fn slow_sel0(self) -> &'a mut W {
        self.variant(FAST_SEL0_A::SLOW_SEL0)
    }
}
#[doc = "Field `PATH_SEL0` reader - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PATH_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PATH_SEL0` writer - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PATH_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, u8, 4, O>;
#[doc = "Field `HFCLK_SEL0` reader - Selects a HFCLK tree for use in fast clock output #0"]
pub type HFCLK_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFCLK_SEL0` writer - Selects a HFCLK tree for use in fast clock output #0"]
pub type HFCLK_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, u8, 4, O>;
#[doc = "Select signal for fast clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAST_SEL1_A {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    NC = 0,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    ECO = 1,
    #[doc = "2: External clock input (EXTCLK)"]
    EXTCLK = 2,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF = 3,
    #[doc = "4: Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK = 4,
    #[doc = "5: Selects the clock path chosen by PATH_SEL1 field"]
    PATH_SEL1 = 5,
    #[doc = "6: Selects the output of the HFCLK_SEL1 mux"]
    HFCLK_SEL1 = 6,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    SLOW_SEL1 = 7,
}
impl From<FAST_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: FAST_SEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FAST_SEL1` reader - Select signal for fast clock output #1"]
pub type FAST_SEL1_R = crate::FieldReader<u8, FAST_SEL1_A>;
impl FAST_SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAST_SEL1_A> {
        match self.bits {
            0 => Some(FAST_SEL1_A::NC),
            1 => Some(FAST_SEL1_A::ECO),
            2 => Some(FAST_SEL1_A::EXTCLK),
            3 => Some(FAST_SEL1_A::ALTHF),
            4 => Some(FAST_SEL1_A::TIMERCLK),
            5 => Some(FAST_SEL1_A::PATH_SEL1),
            6 => Some(FAST_SEL1_A::HFCLK_SEL1),
            7 => Some(FAST_SEL1_A::SLOW_SEL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == FAST_SEL1_A::NC
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == FAST_SEL1_A::ECO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == FAST_SEL1_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `ALTHF`"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == FAST_SEL1_A::ALTHF
    }
    #[doc = "Checks if the value of the field is `TIMERCLK`"]
    #[inline(always)]
    pub fn is_timerclk(&self) -> bool {
        *self == FAST_SEL1_A::TIMERCLK
    }
    #[doc = "Checks if the value of the field is `PATH_SEL1`"]
    #[inline(always)]
    pub fn is_path_sel1(&self) -> bool {
        *self == FAST_SEL1_A::PATH_SEL1
    }
    #[doc = "Checks if the value of the field is `HFCLK_SEL1`"]
    #[inline(always)]
    pub fn is_hfclk_sel1(&self) -> bool {
        *self == FAST_SEL1_A::HFCLK_SEL1
    }
    #[doc = "Checks if the value of the field is `SLOW_SEL1`"]
    #[inline(always)]
    pub fn is_slow_sel1(&self) -> bool {
        *self == FAST_SEL1_A::SLOW_SEL1
    }
}
#[doc = "Field `FAST_SEL1` writer - Select signal for fast clock output #1"]
pub type FAST_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, FAST_SEL1_A, 4, O>;
impl<'a, const O: u8> FAST_SEL1_W<'a, O> {
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::NC)
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::ECO)
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::EXTCLK)
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::ALTHF)
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn timerclk(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::TIMERCLK)
    }
    #[doc = "Selects the clock path chosen by PATH_SEL1 field"]
    #[inline(always)]
    pub fn path_sel1(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::PATH_SEL1)
    }
    #[doc = "Selects the output of the HFCLK_SEL1 mux"]
    #[inline(always)]
    pub fn hfclk_sel1(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::HFCLK_SEL1)
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    #[inline(always)]
    pub fn slow_sel1(self) -> &'a mut W {
        self.variant(FAST_SEL1_A::SLOW_SEL1)
    }
}
#[doc = "Field `PATH_SEL1` reader - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PATH_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PATH_SEL1` writer - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PATH_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, u8, 4, O>;
#[doc = "Field `HFCLK_SEL1` reader - Selects a HFCLK tree for use in fast clock output #1 logic"]
pub type HFCLK_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFCLK_SEL1` writer - Selects a HFCLK tree for use in fast clock output #1 logic"]
pub type HFCLK_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_OUTPUT_FAST_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Select signal for fast clock output #0"]
    #[inline(always)]
    pub fn fast_sel0(&self) -> FAST_SEL0_R {
        FAST_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel0(&self) -> PATH_SEL0_R {
        PATH_SEL0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub fn hfclk_sel0(&self) -> HFCLK_SEL0_R {
        HFCLK_SEL0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select signal for fast clock output #1"]
    #[inline(always)]
    pub fn fast_sel1(&self) -> FAST_SEL1_R {
        FAST_SEL1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel1(&self) -> PATH_SEL1_R {
        PATH_SEL1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub fn hfclk_sel1(&self) -> HFCLK_SEL1_R {
        HFCLK_SEL1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for fast clock output #0"]
    #[inline(always)]
    pub fn fast_sel0(&mut self) -> FAST_SEL0_W<0> {
        FAST_SEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel0(&mut self) -> PATH_SEL0_W<4> {
        PATH_SEL0_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub fn hfclk_sel0(&mut self) -> HFCLK_SEL0_W<8> {
        HFCLK_SEL0_W::new(self)
    }
    #[doc = "Bits 16:19 - Select signal for fast clock output #1"]
    #[inline(always)]
    pub fn fast_sel1(&mut self) -> FAST_SEL1_W<16> {
        FAST_SEL1_W::new(self)
    }
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel1(&mut self) -> PATH_SEL1_W<20> {
        PATH_SEL1_W::new(self)
    }
    #[doc = "Bits 24:27 - Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub fn hfclk_sel1(&mut self) -> HFCLK_SEL1_W<24> {
        HFCLK_SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Clock Output Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_output_fast](index.html) module"]
pub struct CLK_OUTPUT_FAST_SPEC;
impl crate::RegisterSpec for CLK_OUTPUT_FAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_output_fast::R](R) reader structure"]
impl crate::Readable for CLK_OUTPUT_FAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_output_fast::W](W) writer structure"]
impl crate::Writable for CLK_OUTPUT_FAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_OUTPUT_FAST to value 0"]
impl crate::Resettable for CLK_OUTPUT_FAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
