#[doc = "Register `LL_CLK_EN` reader"]
pub struct R(crate::R<LL_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LL_CLK_EN` writer"]
pub struct W(crate::W<LL_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LL_CLK_EN_SPEC>;
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
impl From<crate::W<LL_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LL_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Set this bit 1 to enable the clock to Link Layer."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Set this bit 1 to enable the clock to Link Layer."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
#[doc = "Field `CY_CORREL_EN` reader - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
pub type CY_CORREL_EN_R = crate::BitReader<bool>;
#[doc = "Field `CY_CORREL_EN` writer - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
pub type CY_CORREL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
#[doc = "Field `MXD_IF_OPTION` reader - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
pub type MXD_IF_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `MXD_IF_OPTION` writer - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
pub type MXD_IF_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
#[doc = "Field `SEL_RCB_CLK` reader - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
pub type SEL_RCB_CLK_R = crate::BitReader<bool>;
#[doc = "Field `SEL_RCB_CLK` writer - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
pub type SEL_RCB_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
#[doc = "Field `BLESS_RESET` reader - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
pub type BLESS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `BLESS_RESET` writer - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
pub type BLESS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
#[doc = "Field `DPSLP_HWRCB_EN` reader - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
pub type DPSLP_HWRCB_EN_R = crate::BitReader<bool>;
#[doc = "Field `DPSLP_HWRCB_EN` writer - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
pub type DPSLP_HWRCB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LL_CLK_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit 1 to enable the clock to Link Layer."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
    #[inline(always)]
    pub fn cy_correl_en(&self) -> CY_CORREL_EN_R {
        CY_CORREL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
    #[inline(always)]
    pub fn mxd_if_option(&self) -> MXD_IF_OPTION_R {
        MXD_IF_OPTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
    #[inline(always)]
    pub fn sel_rcb_clk(&self) -> SEL_RCB_CLK_R {
        SEL_RCB_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
    #[inline(always)]
    pub fn bless_reset(&self) -> BLESS_RESET_R {
        BLESS_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
    #[inline(always)]
    pub fn dpslp_hwrcb_en(&self) -> DPSLP_HWRCB_EN_R {
        DPSLP_HWRCB_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit 1 to enable the clock to Link Layer."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
    #[inline(always)]
    pub fn cy_correl_en(&mut self) -> CY_CORREL_EN_W<1> {
        CY_CORREL_EN_W::new(self)
    }
    #[doc = "Bit 2 - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
    #[inline(always)]
    pub fn mxd_if_option(&mut self) -> MXD_IF_OPTION_W<2> {
        MXD_IF_OPTION_W::new(self)
    }
    #[doc = "Bit 3 - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
    #[inline(always)]
    pub fn sel_rcb_clk(&mut self) -> SEL_RCB_CLK_W<3> {
        SEL_RCB_CLK_W::new(self)
    }
    #[doc = "Bit 4 - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
    #[inline(always)]
    pub fn bless_reset(&mut self) -> BLESS_RESET_W<4> {
        BLESS_RESET_W::new(self)
    }
    #[doc = "Bit 5 - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
    #[inline(always)]
    pub fn dpslp_hwrcb_en(&mut self) -> DPSLP_HWRCB_EN_W<5> {
        DPSLP_HWRCB_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Layer primary clock enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_clk_en](index.html) module"]
pub struct LL_CLK_EN_SPEC;
impl crate::RegisterSpec for LL_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_clk_en::R](R) reader structure"]
impl crate::Readable for LL_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ll_clk_en::W](W) writer structure"]
impl crate::Writable for LL_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LL_CLK_EN to value 0x26"]
impl crate::Resettable for LL_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x26
    }
}
