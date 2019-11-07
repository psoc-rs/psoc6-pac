#[doc = "Reader of register CLK_PLL_CONFIG[%s]"]
pub type R = crate::R<u32, super::CLK_PLL_CONFIG>;
#[doc = "Writer for register CLK_PLL_CONFIG[%s]"]
pub type W = crate::W<u32, super::CLK_PLL_CONFIG>;
#[doc = "Register CLK_PLL_CONFIG[%s] `reset()`'s with value 0x0002_0116"]
impl crate::ResetValue for super::CLK_PLL_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0116
    }
}
#[doc = "Reader of field `FEEDBACK_DIV`"]
pub type FEEDBACK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FEEDBACK_DIV`"]
pub struct FEEDBACK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FEEDBACK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `REFERENCE_DIV`"]
pub type REFERENCE_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFERENCE_DIV`"]
pub struct REFERENCE_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REFERENCE_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OUTPUT_DIV`"]
pub type OUTPUT_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTPUT_DIV`"]
pub struct OUTPUT_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLL_LF_MODE`"]
pub type PLL_LF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_LF_MODE`"]
pub struct PLL_LF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_SEL_A {
    #[doc = "0: Automatic using lock indicator.  When unlocked, automatically selects PLL reference input (bypass mode).  When locked, automatically selects PLL output."]
    AUTO,
    #[doc = "1: Same as AUTO"]
    AUTO1,
    #[doc = "2: Select PLL reference input (bypass mode).  Ignores lock indicator"]
    PLL_REF,
    #[doc = "3: Select PLL output.  Ignores lock indicator."]
    PLL_OUT,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        match variant {
            BYPASS_SEL_A::AUTO => 0,
            BYPASS_SEL_A::AUTO1 => 1,
            BYPASS_SEL_A::PLL_REF => 2,
            BYPASS_SEL_A::PLL_OUT => 3,
        }
    }
}
#[doc = "Reader of field `BYPASS_SEL`"]
pub type BYPASS_SEL_R = crate::R<u8, BYPASS_SEL_A>;
impl BYPASS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_SEL_A {
        match self.bits {
            0 => BYPASS_SEL_A::AUTO,
            1 => BYPASS_SEL_A::AUTO1,
            2 => BYPASS_SEL_A::PLL_REF,
            3 => BYPASS_SEL_A::PLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO
    }
    #[doc = "Checks if the value of the field is `AUTO1`"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO1
    }
    #[doc = "Checks if the value of the field is `PLL_REF`"]
    #[inline(always)]
    pub fn is_pll_ref(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_REF
    }
    #[doc = "Checks if the value of the field is `PLL_OUT`"]
    #[inline(always)]
    pub fn is_pll_out(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_OUT
    }
}
#[doc = "Write proxy for field `BYPASS_SEL`"]
pub struct BYPASS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "Same as AUTO"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO1)
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn pll_ref(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::PLL_REF)
    }
    #[doc = "Select PLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn pll_out(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::PLL_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&self) -> FEEDBACK_DIV_R {
        FEEDBACK_DIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&self) -> REFERENCE_DIV_R {
        REFERENCE_DIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&self) -> OUTPUT_DIV_R {
        OUTPUT_DIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\] 1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&self) -> PLL_LF_MODE_R {
        PLL_LF_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&mut self) -> FEEDBACK_DIV_W {
        FEEDBACK_DIV_W { w: self }
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&mut self) -> REFERENCE_DIV_W {
        REFERENCE_DIV_W { w: self }
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&mut self) -> OUTPUT_DIV_W {
        OUTPUT_DIV_W { w: self }
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\] 1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&mut self) -> PLL_LF_MODE_W {
        PLL_LF_MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W {
        BYPASS_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
