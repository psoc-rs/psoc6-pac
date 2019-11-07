#[doc = "Reader of register CLK_OUTPUT_FAST"]
pub type R = crate::R<u32, super::CLK_OUTPUT_FAST>;
#[doc = "Writer for register CLK_OUTPUT_FAST"]
pub type W = crate::W<u32, super::CLK_OUTPUT_FAST>;
#[doc = "Register CLK_OUTPUT_FAST `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_OUTPUT_FAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select signal for fast clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_SEL0_A {
    #[doc = "0: Disabled - output is 0.  For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    NC,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    ECO,
    #[doc = "2: External clock input (EXTCLK)"]
    EXTCLK,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF,
    #[doc = "4: Timer clock.  It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK,
    #[doc = "5: Selects the clock path chosen by PATH_SEL0 field"]
    PATH_SEL0,
    #[doc = "6: Selects the output of the HFCLK_SEL0 mux"]
    HFCLK_SEL0,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    SLOW_SEL0,
}
impl From<FAST_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: FAST_SEL0_A) -> Self {
        match variant {
            FAST_SEL0_A::NC => 0,
            FAST_SEL0_A::ECO => 1,
            FAST_SEL0_A::EXTCLK => 2,
            FAST_SEL0_A::ALTHF => 3,
            FAST_SEL0_A::TIMERCLK => 4,
            FAST_SEL0_A::PATH_SEL0 => 5,
            FAST_SEL0_A::HFCLK_SEL0 => 6,
            FAST_SEL0_A::SLOW_SEL0 => 7,
        }
    }
}
#[doc = "Reader of field `FAST_SEL0`"]
pub type FAST_SEL0_R = crate::R<u8, FAST_SEL0_A>;
impl FAST_SEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAST_SEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAST_SEL0_A::NC),
            1 => Val(FAST_SEL0_A::ECO),
            2 => Val(FAST_SEL0_A::EXTCLK),
            3 => Val(FAST_SEL0_A::ALTHF),
            4 => Val(FAST_SEL0_A::TIMERCLK),
            5 => Val(FAST_SEL0_A::PATH_SEL0),
            6 => Val(FAST_SEL0_A::HFCLK_SEL0),
            7 => Val(FAST_SEL0_A::SLOW_SEL0),
            i => Res(i),
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
#[doc = "Write proxy for field `FAST_SEL0`"]
pub struct FAST_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_SEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PATH_SEL0`"]
pub type PATH_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PATH_SEL0`"]
pub struct PATH_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `HFCLK_SEL0`"]
pub type HFCLK_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFCLK_SEL0`"]
pub struct HFCLK_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLK_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Select signal for fast clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_SEL1_A {
    #[doc = "0: Disabled - output is 0.  For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    NC,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    ECO,
    #[doc = "2: External clock input (EXTCLK)"]
    EXTCLK,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    ALTHF,
    #[doc = "4: Timer clock.  It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    TIMERCLK,
    #[doc = "5: Selects the clock path chosen by PATH_SEL1 field"]
    PATH_SEL1,
    #[doc = "6: Selects the output of the HFCLK_SEL1 mux"]
    HFCLK_SEL1,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    SLOW_SEL1,
}
impl From<FAST_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: FAST_SEL1_A) -> Self {
        match variant {
            FAST_SEL1_A::NC => 0,
            FAST_SEL1_A::ECO => 1,
            FAST_SEL1_A::EXTCLK => 2,
            FAST_SEL1_A::ALTHF => 3,
            FAST_SEL1_A::TIMERCLK => 4,
            FAST_SEL1_A::PATH_SEL1 => 5,
            FAST_SEL1_A::HFCLK_SEL1 => 6,
            FAST_SEL1_A::SLOW_SEL1 => 7,
        }
    }
}
#[doc = "Reader of field `FAST_SEL1`"]
pub type FAST_SEL1_R = crate::R<u8, FAST_SEL1_A>;
impl FAST_SEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAST_SEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAST_SEL1_A::NC),
            1 => Val(FAST_SEL1_A::ECO),
            2 => Val(FAST_SEL1_A::EXTCLK),
            3 => Val(FAST_SEL1_A::ALTHF),
            4 => Val(FAST_SEL1_A::TIMERCLK),
            5 => Val(FAST_SEL1_A::PATH_SEL1),
            6 => Val(FAST_SEL1_A::HFCLK_SEL1),
            7 => Val(FAST_SEL1_A::SLOW_SEL1),
            i => Res(i),
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
#[doc = "Write proxy for field `FAST_SEL1`"]
pub struct FAST_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_SEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PATH_SEL1`"]
pub type PATH_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PATH_SEL1`"]
pub struct PATH_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `HFCLK_SEL1`"]
pub type HFCLK_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFCLK_SEL1`"]
pub struct HFCLK_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLK_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Select signal for fast clock output #0"]
    #[inline(always)]
    pub fn fast_sel0(&self) -> FAST_SEL0_R {
        FAST_SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. For FLL path, it connects after the bypass mux. For PLL path(s), it connects after the CLK_PLL_DDFT mux. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
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
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. For FLL path, it connects after the bypass mux. For PLL path(s), it connects after the CLK_PLL_DDFT mux. 1-15: PLL output on path1-path15 (if available)"]
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
    pub fn fast_sel0(&mut self) -> FAST_SEL0_W {
        FAST_SEL0_W { w: self }
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. For FLL path, it connects after the bypass mux. For PLL path(s), it connects after the CLK_PLL_DDFT mux. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel0(&mut self) -> PATH_SEL0_W {
        PATH_SEL0_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub fn hfclk_sel0(&mut self) -> HFCLK_SEL0_W {
        HFCLK_SEL0_W { w: self }
    }
    #[doc = "Bits 16:19 - Select signal for fast clock output #1"]
    #[inline(always)]
    pub fn fast_sel1(&mut self) -> FAST_SEL1_W {
        FAST_SEL1_W { w: self }
    }
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. For FLL path, it connects after the bypass mux. For PLL path(s), it connects after the CLK_PLL_DDFT mux. 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel1(&mut self) -> PATH_SEL1_W {
        PATH_SEL1_W { w: self }
    }
    #[doc = "Bits 24:27 - Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub fn hfclk_sel1(&mut self) -> HFCLK_SEL1_W {
        HFCLK_SEL1_W { w: self }
    }
}
