#[doc = "Register `CLK_PLL_CONFIG[%s]` reader"]
pub struct R(crate::R<CLK_PLL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PLL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PLL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PLL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PLL_CONFIG[%s]` writer"]
pub struct W(crate::W<CLK_PLL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PLL_CONFIG_SPEC>;
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
impl From<crate::W<CLK_PLL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PLL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEEDBACK_DIV` reader - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEEDBACK_DIV` writer - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_PLL_CONFIG_SPEC, u8, u8, 7, O>;
#[doc = "Field `REFERENCE_DIV` reader - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFERENCE_DIV` writer - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_PLL_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_PLL_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `PLL_LF_MODE` reader - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PLL_LF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_LF_MODE` writer - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PLL_LF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_PLL_CONFIG_SPEC, bool, O>;
#[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    AUTO = 0,
    #[doc = "1: Same as AUTO"]
    AUTO1 = 1,
    #[doc = "2: Select PLL reference input (bypass mode). Ignores lock indicator"]
    PLL_REF = 2,
    #[doc = "3: Select PLL output. Ignores lock indicator."]
    PLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BYPASS_SEL_R = crate::FieldReader<u8, BYPASS_SEL_A>;
impl BYPASS_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BYPASS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_PLL_CONFIG_SPEC, u8, BYPASS_SEL_A, 2, O>;
impl<'a, const O: u8> BYPASS_SEL_W<'a, O> {
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
}
#[doc = "Field `ENABLE` reader - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_PLL_CONFIG_SPEC, bool, O>;
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
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&self) -> PLL_LF_MODE_R {
        PLL_LF_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&mut self) -> FEEDBACK_DIV_W<0> {
        FEEDBACK_DIV_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&mut self) -> REFERENCE_DIV_W<8> {
        REFERENCE_DIV_W::new(self)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&mut self) -> OUTPUT_DIV_W<16> {
        OUTPUT_DIV_W::new(self)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&mut self) -> PLL_LF_MODE_W<27> {
        PLL_LF_MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W<28> {
        BYPASS_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_config](index.html) module"]
pub struct CLK_PLL_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_PLL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pll_config::R](R) reader structure"]
impl crate::Readable for CLK_PLL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pll_config::W](W) writer structure"]
impl crate::Writable for CLK_PLL_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PLL_CONFIG[%s]
to value 0x0002_0116"]
impl crate::Resettable for CLK_PLL_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0116
    }
}
