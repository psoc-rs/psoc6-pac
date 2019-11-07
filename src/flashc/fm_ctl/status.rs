#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `HV_TIMER_RUNNING`"]
pub type HV_TIMER_RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `HV_REGS_ISOLATED`"]
pub type HV_REGS_ISOLATED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ILLEGAL_HVOP`"]
pub type ILLEGAL_HVOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TURBO_N`"]
pub type TURBO_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_EN_MON`"]
pub type WR_EN_MON_R = crate::R<bool, bool>;
#[doc = "Reader of field `IF_SEL_MON`"]
pub type IF_SEL_MON_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates if the high voltage timer is running: '0': not running '1': running"]
    #[inline(always)]
    pub fn hv_timer_running(&self) -> HV_TIMER_RUNNING_R {
        HV_TIMER_RUNNING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the isolation status at HV trim and redundancy registers inputs '0' - Not isolated, writing permitted '1' - isolated writing disabled"]
    #[inline(always)]
    pub fn hv_regs_isolated(&self) -> HV_REGS_ISOLATED_R {
        HV_REGS_ISOLATED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates a bulk,sector erase, program has been requested when axa=1 '0' - no error '1' - illegal HV operation error"]
    #[inline(always)]
    pub fn illegal_hvop(&self) -> ILLEGAL_HVOP_R {
        ILLEGAL_HVOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. '0' - turbo mode '1' - normal mode"]
    #[inline(always)]
    pub fn turbo_n(&self) -> TURBO_N_R {
        TURBO_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn wr_en_mon(&self) -> WR_EN_MON_R {
        WR_EN_MON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn if_sel_mon(&self) -> IF_SEL_MON_R {
        IF_SEL_MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
