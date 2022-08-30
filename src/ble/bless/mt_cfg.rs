#[doc = "Register `MT_CFG` reader"]
pub struct R(crate::R<MT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_CFG` writer"]
pub struct W(crate::W<MT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_CFG_SPEC>;
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
impl From<crate::W<MT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_BLERD` reader - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
pub type ENABLE_BLERD_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_BLERD` writer - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
pub type ENABLE_BLERD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `DEEPSLEEP_EXIT_CFG` reader - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
pub type DEEPSLEEP_EXIT_CFG_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_EXIT_CFG` writer - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
pub type DEEPSLEEP_EXIT_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `DEEPSLEEP_EXITED` reader - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
pub type DEEPSLEEP_EXITED_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_EXITED` writer - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
pub type DEEPSLEEP_EXITED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `ACT_LDO_NOT_BUCK` reader - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
pub type ACT_LDO_NOT_BUCK_R = crate::BitReader<bool>;
#[doc = "Field `ACT_LDO_NOT_BUCK` writer - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
pub type ACT_LDO_NOT_BUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_HVLDO_BYPASS` reader - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
pub type OVERRIDE_HVLDO_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_HVLDO_BYPASS` writer - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
pub type OVERRIDE_HVLDO_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `HVLDO_BYPASS` reader - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
pub type HVLDO_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_BYPASS` writer - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
pub type HVLDO_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_ACT_REGULATOR` reader - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
pub type OVERRIDE_ACT_REGULATOR_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_ACT_REGULATOR` writer - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
pub type OVERRIDE_ACT_REGULATOR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `ACT_REGULATOR_EN` reader - Override value for ACT_LDO_EN/BUCK_EN"]
pub type ACT_REGULATOR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACT_REGULATOR_EN` writer - Override value for ACT_LDO_EN/BUCK_EN"]
pub type ACT_REGULATOR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_DIG_REGULATOR` reader - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
pub type OVERRIDE_DIG_REGULATOR_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_DIG_REGULATOR` writer - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
pub type OVERRIDE_DIG_REGULATOR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `DIG_REGULATOR_EN` reader - Override value for digital regulator of CYBLERD55"]
pub type DIG_REGULATOR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_REGULATOR_EN` writer - Override value for digital regulator of CYBLERD55"]
pub type DIG_REGULATOR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_RET_SWITCH` reader - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
pub type OVERRIDE_RET_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_RET_SWITCH` writer - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
pub type OVERRIDE_RET_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `RET_SWITCH` reader - Override value for RET_SWITCH"]
pub type RET_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `RET_SWITCH` writer - Override value for RET_SWITCH"]
pub type RET_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_ISOLATE` reader - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
pub type OVERRIDE_ISOLATE_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_ISOLATE` writer - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
pub type OVERRIDE_ISOLATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `ISOLATE_N` reader - Override value for isolation to CYBLERD55"]
pub type ISOLATE_N_R = crate::BitReader<bool>;
#[doc = "Field `ISOLATE_N` writer - Override value for isolation to CYBLERD55"]
pub type ISOLATE_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_LL_CLK_EN` reader - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
pub type OVERRIDE_LL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_LL_CLK_EN` writer - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
pub type OVERRIDE_LL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `LL_CLK_EN` reader - Override value for LL Clock gate"]
pub type LL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LL_CLK_EN` writer - Override value for LL Clock gate"]
pub type LL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_HVLDO_EN` reader - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
pub type OVERRIDE_HVLDO_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_HVLDO_EN` writer - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
pub type OVERRIDE_HVLDO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `HVLDO_EN` reader - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
pub type HVLDO_EN_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_EN` writer - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
pub type HVLDO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `DPSLP_ECO_ON` reader - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
pub type DPSLP_ECO_ON_R = crate::BitReader<bool>;
#[doc = "Field `DPSLP_ECO_ON` writer - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
pub type DPSLP_ECO_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_RESET_N` reader - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
pub type OVERRIDE_RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_RESET_N` writer - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
pub type OVERRIDE_RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `RESET_N` reader - Overrie value for CYBLERD55 RESET_N"]
pub type RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `RESET_N` writer - Overrie value for CYBLERD55 RESET_N"]
pub type RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_XTAL_EN` reader - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
pub type OVERRIDE_XTAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_XTAL_EN` writer - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
pub type OVERRIDE_XTAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `XTAL_EN` reader - Overrie value for CYBLERD55 XTAL_EN"]
pub type XTAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_EN` writer - Overrie value for CYBLERD55 XTAL_EN"]
pub type XTAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_CLK_EN` reader - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
pub type OVERRIDE_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_CLK_EN` writer - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
pub type OVERRIDE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `BLERD_CLK_EN` reader - Overrie value for CYBLERD55 CLK_EN"]
pub type BLERD_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `BLERD_CLK_EN` writer - Overrie value for CYBLERD55 CLK_EN"]
pub type BLERD_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `OVERRIDE_RET_LDO_OL` reader - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
pub type OVERRIDE_RET_LDO_OL_R = crate::BitReader<bool>;
#[doc = "Field `OVERRIDE_RET_LDO_OL` writer - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
pub type OVERRIDE_RET_LDO_OL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `RET_LDO_OL` reader - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
pub type RET_LDO_OL_R = crate::BitReader<bool>;
#[doc = "Field `RET_LDO_OL` writer - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
pub type RET_LDO_OL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
#[doc = "Field `HVLDO_POR_HV` reader - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
pub type HVLDO_POR_HV_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_POR_HV` writer - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
pub type HVLDO_POR_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
    #[inline(always)]
    pub fn enable_blerd(&self) -> ENABLE_BLERD_R {
        ENABLE_BLERD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
    #[inline(always)]
    pub fn deepsleep_exit_cfg(&self) -> DEEPSLEEP_EXIT_CFG_R {
        DEEPSLEEP_EXIT_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
    #[inline(always)]
    pub fn deepsleep_exited(&self) -> DEEPSLEEP_EXITED_R {
        DEEPSLEEP_EXITED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
    #[inline(always)]
    pub fn act_ldo_not_buck(&self) -> ACT_LDO_NOT_BUCK_R {
        ACT_LDO_NOT_BUCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
    #[inline(always)]
    pub fn override_hvldo_bypass(&self) -> OVERRIDE_HVLDO_BYPASS_R {
        OVERRIDE_HVLDO_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
    #[inline(always)]
    pub fn hvldo_bypass(&self) -> HVLDO_BYPASS_R {
        HVLDO_BYPASS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_act_regulator(&self) -> OVERRIDE_ACT_REGULATOR_R {
        OVERRIDE_ACT_REGULATOR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override value for ACT_LDO_EN/BUCK_EN"]
    #[inline(always)]
    pub fn act_regulator_en(&self) -> ACT_REGULATOR_EN_R {
        ACT_REGULATOR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_dig_regulator(&self) -> OVERRIDE_DIG_REGULATOR_R {
        OVERRIDE_DIG_REGULATOR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Override value for digital regulator of CYBLERD55"]
    #[inline(always)]
    pub fn dig_regulator_en(&self) -> DIG_REGULATOR_EN_R {
        DIG_REGULATOR_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
    #[inline(always)]
    pub fn override_ret_switch(&self) -> OVERRIDE_RET_SWITCH_R {
        OVERRIDE_RET_SWITCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Override value for RET_SWITCH"]
    #[inline(always)]
    pub fn ret_switch(&self) -> RET_SWITCH_R {
        RET_SWITCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
    #[inline(always)]
    pub fn override_isolate(&self) -> OVERRIDE_ISOLATE_R {
        OVERRIDE_ISOLATE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Override value for isolation to CYBLERD55"]
    #[inline(always)]
    pub fn isolate_n(&self) -> ISOLATE_N_R {
        ISOLATE_N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
    #[inline(always)]
    pub fn override_ll_clk_en(&self) -> OVERRIDE_LL_CLK_EN_R {
        OVERRIDE_LL_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Override value for LL Clock gate"]
    #[inline(always)]
    pub fn ll_clk_en(&self) -> LL_CLK_EN_R {
        LL_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
    #[inline(always)]
    pub fn override_hvldo_en(&self) -> OVERRIDE_HVLDO_EN_R {
        OVERRIDE_HVLDO_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
    #[inline(always)]
    pub fn hvldo_en(&self) -> HVLDO_EN_R {
        HVLDO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
    #[inline(always)]
    pub fn dpslp_eco_on(&self) -> DPSLP_ECO_ON_R {
        DPSLP_ECO_ON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
    #[inline(always)]
    pub fn override_reset_n(&self) -> OVERRIDE_RESET_N_R {
        OVERRIDE_RESET_N_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Overrie value for CYBLERD55 RESET_N"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
    #[inline(always)]
    pub fn override_xtal_en(&self) -> OVERRIDE_XTAL_EN_R {
        OVERRIDE_XTAL_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Overrie value for CYBLERD55 XTAL_EN"]
    #[inline(always)]
    pub fn xtal_en(&self) -> XTAL_EN_R {
        XTAL_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_clk_en(&self) -> OVERRIDE_CLK_EN_R {
        OVERRIDE_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Overrie value for CYBLERD55 CLK_EN"]
    #[inline(always)]
    pub fn blerd_clk_en(&self) -> BLERD_CLK_EN_R {
        BLERD_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_ret_ldo_ol(&self) -> OVERRIDE_RET_LDO_OL_R {
        OVERRIDE_RET_LDO_OL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
    #[inline(always)]
    pub fn ret_ldo_ol(&self) -> RET_LDO_OL_R {
        RET_LDO_OL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
    #[inline(always)]
    pub fn hvldo_por_hv(&self) -> HVLDO_POR_HV_R {
        HVLDO_POR_HV_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register bit needs to be set to enable CYBLERD55 1'b1 - CYBLERD55 enabled 1'b0 - CYBLERD55 disabled On power up this bit needs to be set to make CYBLERD55 active."]
    #[inline(always)]
    pub fn enable_blerd(&mut self) -> ENABLE_BLERD_W<0> {
        ENABLE_BLERD_W::new(self)
    }
    #[doc = "Bit 1 - This register bit indicates the source for PSoC DeepSleep exit to BLESS 1'b0 - act_power_good from SRSS indicates PSoC DeepSleep exit 1'b1 - MT_CFG.DEEPSLEEP_EXITED indicates PSoC DeepSleep exit"]
    #[inline(always)]
    pub fn deepsleep_exit_cfg(&mut self) -> DEEPSLEEP_EXIT_CFG_W<1> {
        DEEPSLEEP_EXIT_CFG_W::new(self)
    }
    #[doc = "Bit 2 - This register bit is used by FW to indicate that PSoC is out of DeepSleep 1'b0 - PSoC in DeepSleep 1'b1 - PSoC out of DeepSleep This bit is cleared by HW on exit from DPSLP"]
    #[inline(always)]
    pub fn deepsleep_exited(&mut self) -> DEEPSLEEP_EXITED_W<2> {
        DEEPSLEEP_EXITED_W::new(self)
    }
    #[doc = "Bit 3 - This register bit specifies whether the Active LDO or BUCK in CYBLERD55 is used in active mode"]
    #[inline(always)]
    pub fn act_ldo_not_buck(&mut self) -> ACT_LDO_NOT_BUCK_W<3> {
        ACT_LDO_NOT_BUCK_W::new(self)
    }
    #[doc = "Bit 4 - This register should be set to override the HW generated signal to HVLDO. When set HVLDO_BYPASS is driven to the IP"]
    #[inline(always)]
    pub fn override_hvldo_bypass(&mut self) -> OVERRIDE_HVLDO_BYPASS_W<4> {
        OVERRIDE_HVLDO_BYPASS_W::new(self)
    }
    #[doc = "Bit 5 - Override value for HVLDO BYPASS 1'b0: bypass the HVLDO 1'b1: Do not bypass the HVLDO"]
    #[inline(always)]
    pub fn hvldo_bypass(&mut self) -> HVLDO_BYPASS_W<5> {
        HVLDO_BYPASS_W::new(self)
    }
    #[doc = "Bit 6 - This register should be set to override the HW generated signal to enable ACTIVE_LDO/BUCK. When set ACT_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_act_regulator(&mut self) -> OVERRIDE_ACT_REGULATOR_W<6> {
        OVERRIDE_ACT_REGULATOR_W::new(self)
    }
    #[doc = "Bit 7 - Override value for ACT_LDO_EN/BUCK_EN"]
    #[inline(always)]
    pub fn act_regulator_en(&mut self) -> ACT_REGULATOR_EN_W<7> {
        ACT_REGULATOR_EN_W::new(self)
    }
    #[doc = "Bit 8 - This register should be set to override the HW generated signal to Digital regulator of CYBLERD55. When set DIG_REGULATOR_EN is driven to CYBLERD55"]
    #[inline(always)]
    pub fn override_dig_regulator(&mut self) -> OVERRIDE_DIG_REGULATOR_W<8> {
        OVERRIDE_DIG_REGULATOR_W::new(self)
    }
    #[doc = "Bit 9 - Override value for digital regulator of CYBLERD55"]
    #[inline(always)]
    pub fn dig_regulator_en(&mut self) -> DIG_REGULATOR_EN_W<9> {
        DIG_REGULATOR_EN_W::new(self)
    }
    #[doc = "Bit 10 - This register should be set to override the HW generated signal to the retention switch of CYBLERD55. When set OVERRIDE_RET_SWITCH is driven to the IP"]
    #[inline(always)]
    pub fn override_ret_switch(&mut self) -> OVERRIDE_RET_SWITCH_W<10> {
        OVERRIDE_RET_SWITCH_W::new(self)
    }
    #[doc = "Bit 11 - Override value for RET_SWITCH"]
    #[inline(always)]
    pub fn ret_switch(&mut self) -> RET_SWITCH_W<11> {
        RET_SWITCH_W::new(self)
    }
    #[doc = "Bit 12 - This register should be set to override the HW generated isolation signal to CYBLERD55. When set ISOLATE_N is driven to the IP"]
    #[inline(always)]
    pub fn override_isolate(&mut self) -> OVERRIDE_ISOLATE_W<12> {
        OVERRIDE_ISOLATE_W::new(self)
    }
    #[doc = "Bit 13 - Override value for isolation to CYBLERD55"]
    #[inline(always)]
    pub fn isolate_n(&mut self) -> ISOLATE_N_W<13> {
        ISOLATE_N_W::new(self)
    }
    #[doc = "Bit 14 - This register should be set to override the HW generated ECO Clock gate. When set LL_CLK_EN is used to gate the clock"]
    #[inline(always)]
    pub fn override_ll_clk_en(&mut self) -> OVERRIDE_LL_CLK_EN_W<14> {
        OVERRIDE_LL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 15 - Override value for LL Clock gate"]
    #[inline(always)]
    pub fn ll_clk_en(&mut self) -> LL_CLK_EN_W<15> {
        LL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 16 - This register should be set to override the HW generated enable to HVLSO. When set HVLDO_EN is used."]
    #[inline(always)]
    pub fn override_hvldo_en(&mut self) -> OVERRIDE_HVLDO_EN_W<16> {
        OVERRIDE_HVLDO_EN_W::new(self)
    }
    #[doc = "Bit 17 - Overrie value for HVLDO enable 1'b1: switch to Active LDO 1'b0: switch to standby LDO"]
    #[inline(always)]
    pub fn hvldo_en(&mut self) -> HVLDO_EN_W<17> {
        HVLDO_EN_W::new(self)
    }
    #[doc = "Bit 18 - This bit when set indicates that ECO clock should be kept on even in BLESS DPSLP. This bit must be toggled only when the Link Layer is active."]
    #[inline(always)]
    pub fn dpslp_eco_on(&mut self) -> DPSLP_ECO_ON_W<18> {
        DPSLP_ECO_ON_W::new(self)
    }
    #[doc = "Bit 19 - This register should be set to override the HW generated reset to CYBLERD55. When set RESET_N is used."]
    #[inline(always)]
    pub fn override_reset_n(&mut self) -> OVERRIDE_RESET_N_W<19> {
        OVERRIDE_RESET_N_W::new(self)
    }
    #[doc = "Bit 20 - Overrie value for CYBLERD55 RESET_N"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W<20> {
        RESET_N_W::new(self)
    }
    #[doc = "Bit 21 - This register should be set to override the HW generated XTAL_EN to CYBLERD55. When set XTAL_EN is used."]
    #[inline(always)]
    pub fn override_xtal_en(&mut self) -> OVERRIDE_XTAL_EN_W<21> {
        OVERRIDE_XTAL_EN_W::new(self)
    }
    #[doc = "Bit 22 - Overrie value for CYBLERD55 XTAL_EN"]
    #[inline(always)]
    pub fn xtal_en(&mut self) -> XTAL_EN_W<22> {
        XTAL_EN_W::new(self)
    }
    #[doc = "Bit 23 - This register should be set to override the HW generated CLK_EN to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_clk_en(&mut self) -> OVERRIDE_CLK_EN_W<23> {
        OVERRIDE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 24 - Overrie value for CYBLERD55 CLK_EN"]
    #[inline(always)]
    pub fn blerd_clk_en(&mut self) -> BLERD_CLK_EN_W<24> {
        BLERD_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - This register should be set to override the HW generated RET_LDO_OL_HV to CYBLERD55. When set CLK_EN is used."]
    #[inline(always)]
    pub fn override_ret_ldo_ol(&mut self) -> OVERRIDE_RET_LDO_OL_W<25> {
        OVERRIDE_RET_LDO_OL_W::new(self)
    }
    #[doc = "Bit 26 - Overrie value for CYBLERD55 RET_LDO_OL_HV"]
    #[inline(always)]
    pub fn ret_ldo_ol(&mut self) -> RET_LDO_OL_W<26> {
        RET_LDO_OL_W::new(self)
    }
    #[doc = "Bit 27 - Reset for HVLDO 1'b1 - HVLDO Disabled 1'b0 - HVLDO Enabled"]
    #[inline(always)]
    pub fn hvldo_por_hv(&mut self) -> HVLDO_POR_HV_W<27> {
        HVLDO_POR_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_cfg](index.html) module"]
pub struct MT_CFG_SPEC;
impl crate::RegisterSpec for MT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_cfg::R](R) reader structure"]
impl crate::Readable for MT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_cfg::W](W) writer structure"]
impl crate::Writable for MT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_CFG to value 0x0810_0000"]
impl crate::Resettable for MT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0810_0000
    }
}
