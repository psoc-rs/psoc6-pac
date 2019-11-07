#[doc = "Reader of register LL_CLK_EN"]
pub type R = crate::R<u32, super::LL_CLK_EN>;
#[doc = "Writer for register LL_CLK_EN"]
pub type W = crate::W<u32, super::LL_CLK_EN>;
#[doc = "Register LL_CLK_EN `reset()`'s with value 0x26"]
impl crate::ResetValue for super::LL_CLK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x26
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
#[doc = "Reader of field `CY_CORREL_EN`"]
pub type CY_CORREL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CY_CORREL_EN`"]
pub struct CY_CORREL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CY_CORREL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MXD_IF_OPTION`"]
pub type MXD_IF_OPTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MXD_IF_OPTION`"]
pub struct MXD_IF_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> MXD_IF_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SEL_RCB_CLK`"]
pub type SEL_RCB_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_RCB_CLK`"]
pub struct SEL_RCB_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_RCB_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BLESS_RESET`"]
pub type BLESS_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLESS_RESET`"]
pub struct BLESS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> BLESS_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPSLP_HWRCB_EN`"]
pub type DPSLP_HWRCB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPSLP_HWRCB_EN`"]
pub struct DPSLP_HWRCB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_HWRCB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit 1 to enable the clock to Link Layer."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
    #[inline(always)]
    pub fn cy_correl_en(&self) -> CY_CORREL_EN_R {
        CY_CORREL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
    #[inline(always)]
    pub fn mxd_if_option(&self) -> MXD_IF_OPTION_R {
        MXD_IF_OPTION_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
    #[inline(always)]
    pub fn sel_rcb_clk(&self) -> SEL_RCB_CLK_R {
        SEL_RCB_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
    #[inline(always)]
    pub fn bless_reset(&self) -> BLESS_RESET_R {
        BLESS_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
    #[inline(always)]
    pub fn dpslp_hwrcb_en(&self) -> DPSLP_HWRCB_EN_R {
        DPSLP_HWRCB_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit 1 to enable the clock to Link Layer."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - If MXD_IF option is 1, this bit needs to be set to enable configuring the correlator through BLELL.DPLL_CONFIG register"]
    #[inline(always)]
    pub fn cy_correl_en(&mut self) -> CY_CORREL_EN_W {
        CY_CORREL_EN_W { w: self }
    }
    #[doc = "Bit 2 - 1: MXD IF option 0: CYBLERD55 correlates Access Code 0: MXD IF option 1: LL correlates Access Code"]
    #[inline(always)]
    pub fn mxd_if_option(&mut self) -> MXD_IF_OPTION_W {
        MXD_IF_OPTION_W { w: self }
    }
    #[doc = "Bit 3 - 0: AHB clock (clk_sys) is used as the clock for RCB access 1: LL clock (clk_eco) is used as the clock for RCB access"]
    #[inline(always)]
    pub fn sel_rcb_clk(&mut self) -> SEL_RCB_CLK_W {
        SEL_RCB_CLK_W { w: self }
    }
    #[doc = "Bit 4 - 0: No Soft Reset 1: Initiate Soft Reset Setting this bit will reset entire BLESS_VER3"]
    #[inline(always)]
    pub fn bless_reset(&mut self) -> BLESS_RESET_W {
        BLESS_RESET_W { w: self }
    }
    #[doc = "Bit 5 - Controls the DPSLP entry and exit writes to RD and controls the active domain reset and clock. 1 - LL HW controls the RD active domain reset and clock. 0 - The RD active domain reset and clock. Must be controlled by the FW"]
    #[inline(always)]
    pub fn dpslp_hwrcb_en(&mut self) -> DPSLP_HWRCB_EN_W {
        DPSLP_HWRCB_EN_W { w: self }
    }
}
