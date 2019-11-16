#[doc = "BLESS DDFT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_config](ddft_config) module"]
pub type DDFT_CONFIG = crate::Reg<u32, _DDFT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDFT_CONFIG;
#[doc = "`read()` method returns [ddft_config::R](ddft_config::R) reader structure"]
impl crate::Readable for DDFT_CONFIG {}
#[doc = "`write(|w| ..)` method takes [ddft_config::W](ddft_config::W) writer structure"]
impl crate::Writable for DDFT_CONFIG {}
#[doc = "BLESS DDFT configuration register"]
pub mod ddft_config;
#[doc = "Crystal clock divider configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_clk_div_config](xtal_clk_div_config) module"]
pub type XTAL_CLK_DIV_CONFIG = crate::Reg<u32, _XTAL_CLK_DIV_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL_CLK_DIV_CONFIG;
#[doc = "`read()` method returns [xtal_clk_div_config::R](xtal_clk_div_config::R) reader structure"]
impl crate::Readable for XTAL_CLK_DIV_CONFIG {}
#[doc = "`write(|w| ..)` method takes [xtal_clk_div_config::W](xtal_clk_div_config::W) writer structure"]
impl crate::Writable for XTAL_CLK_DIV_CONFIG {}
#[doc = "Crystal clock divider configuration register"]
pub mod xtal_clk_div_config;
#[doc = "Link Layer interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_stat](intr_stat) module"]
pub type INTR_STAT = crate::Reg<u32, _INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_STAT;
#[doc = "`read()` method returns [intr_stat::R](intr_stat::R) reader structure"]
impl crate::Readable for INTR_STAT {}
#[doc = "`write(|w| ..)` method takes [intr_stat::W](intr_stat::W) writer structure"]
impl crate::Writable for INTR_STAT {}
#[doc = "Link Layer interrupt status register"]
pub mod intr_stat;
#[doc = "Link Layer interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Link Layer interrupt mask register"]
pub mod intr_mask;
#[doc = "Link Layer primary clock enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_clk_en](ll_clk_en) module"]
pub type LL_CLK_EN = crate::Reg<u32, _LL_CLK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_CLK_EN;
#[doc = "`read()` method returns [ll_clk_en::R](ll_clk_en::R) reader structure"]
impl crate::Readable for LL_CLK_EN {}
#[doc = "`write(|w| ..)` method takes [ll_clk_en::W](ll_clk_en::W) writer structure"]
impl crate::Writable for LL_CLK_EN {}
#[doc = "Link Layer primary clock enable"]
pub mod ll_clk_en;
#[doc = "BLESS LF clock control and BLESS revision ID indicator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lf_clk_ctrl](lf_clk_ctrl) module"]
pub type LF_CLK_CTRL = crate::Reg<u32, _LF_CLK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LF_CLK_CTRL;
#[doc = "`read()` method returns [lf_clk_ctrl::R](lf_clk_ctrl::R) reader structure"]
impl crate::Readable for LF_CLK_CTRL {}
#[doc = "`write(|w| ..)` method takes [lf_clk_ctrl::W](lf_clk_ctrl::W) writer structure"]
impl crate::Writable for LF_CLK_CTRL {}
#[doc = "BLESS LF clock control and BLESS revision ID indicator"]
pub mod lf_clk_ctrl;
#[doc = "External TX PA and RX LNA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_pa_lna_ctrl](ext_pa_lna_ctrl) module"]
pub type EXT_PA_LNA_CTRL = crate::Reg<u32, _EXT_PA_LNA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_PA_LNA_CTRL;
#[doc = "`read()` method returns [ext_pa_lna_ctrl::R](ext_pa_lna_ctrl::R) reader structure"]
impl crate::Readable for EXT_PA_LNA_CTRL {}
#[doc = "`write(|w| ..)` method takes [ext_pa_lna_ctrl::W](ext_pa_lna_ctrl::W) writer structure"]
impl crate::Writable for EXT_PA_LNA_CTRL {}
#[doc = "External TX PA and RX LNA control"]
pub mod ext_pa_lna_ctrl;
#[doc = "Link Layer Last Received packet RSSI/Channel energy and channel number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_pkt_rssi_ch_energy](ll_pkt_rssi_ch_energy) module"]
pub type LL_PKT_RSSI_CH_ENERGY = crate::Reg<u32, _LL_PKT_RSSI_CH_ENERGY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_PKT_RSSI_CH_ENERGY;
#[doc = "`read()` method returns [ll_pkt_rssi_ch_energy::R](ll_pkt_rssi_ch_energy::R) reader structure"]
impl crate::Readable for LL_PKT_RSSI_CH_ENERGY {}
#[doc = "Link Layer Last Received packet RSSI/Channel energy and channel number"]
pub mod ll_pkt_rssi_ch_energy;
#[doc = "BT clock captured on an LL DSM exit\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_clock_capt](bt_clock_capt) module"]
pub type BT_CLOCK_CAPT = crate::Reg<u32, _BT_CLOCK_CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_CLOCK_CAPT;
#[doc = "`read()` method returns [bt_clock_capt::R](bt_clock_capt::R) reader structure"]
impl crate::Readable for BT_CLOCK_CAPT {}
#[doc = "BT clock captured on an LL DSM exit"]
pub mod bt_clock_capt;
#[doc = "MT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_cfg](mt_cfg) module"]
pub type MT_CFG = crate::Reg<u32, _MT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_CFG;
#[doc = "`read()` method returns [mt_cfg::R](mt_cfg::R) reader structure"]
impl crate::Readable for MT_CFG {}
#[doc = "`write(|w| ..)` method takes [mt_cfg::W](mt_cfg::W) writer structure"]
impl crate::Writable for MT_CFG {}
#[doc = "MT Configuration Register"]
pub mod mt_cfg;
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg](mt_delay_cfg) module"]
pub type MT_DELAY_CFG = crate::Reg<u32, _MT_DELAY_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_DELAY_CFG;
#[doc = "`read()` method returns [mt_delay_cfg::R](mt_delay_cfg::R) reader structure"]
impl crate::Readable for MT_DELAY_CFG {}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg::W](mt_delay_cfg::W) writer structure"]
impl crate::Writable for MT_DELAY_CFG {}
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg;
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg2](mt_delay_cfg2) module"]
pub type MT_DELAY_CFG2 = crate::Reg<u32, _MT_DELAY_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_DELAY_CFG2;
#[doc = "`read()` method returns [mt_delay_cfg2::R](mt_delay_cfg2::R) reader structure"]
impl crate::Readable for MT_DELAY_CFG2 {}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg2::W](mt_delay_cfg2::W) writer structure"]
impl crate::Writable for MT_DELAY_CFG2 {}
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg2;
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg3](mt_delay_cfg3) module"]
pub type MT_DELAY_CFG3 = crate::Reg<u32, _MT_DELAY_CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_DELAY_CFG3;
#[doc = "`read()` method returns [mt_delay_cfg3::R](mt_delay_cfg3::R) reader structure"]
impl crate::Readable for MT_DELAY_CFG3 {}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg3::W](mt_delay_cfg3::W) writer structure"]
impl crate::Writable for MT_DELAY_CFG3 {}
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg3;
#[doc = "MT Configuration Register to control VIO switches\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_vio_ctrl](mt_vio_ctrl) module"]
pub type MT_VIO_CTRL = crate::Reg<u32, _MT_VIO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_VIO_CTRL;
#[doc = "`read()` method returns [mt_vio_ctrl::R](mt_vio_ctrl::R) reader structure"]
impl crate::Readable for MT_VIO_CTRL {}
#[doc = "`write(|w| ..)` method takes [mt_vio_ctrl::W](mt_vio_ctrl::W) writer structure"]
impl crate::Writable for MT_VIO_CTRL {}
#[doc = "MT Configuration Register to control VIO switches"]
pub mod mt_vio_ctrl;
#[doc = "MT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_status](mt_status) module"]
pub type MT_STATUS = crate::Reg<u32, _MT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MT_STATUS;
#[doc = "`read()` method returns [mt_status::R](mt_status::R) reader structure"]
impl crate::Readable for MT_STATUS {}
#[doc = "MT Status Register"]
pub mod mt_status;
#[doc = "Link Layer Power Control FSM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl_sm_st](pwr_ctrl_sm_st) module"]
pub type PWR_CTRL_SM_ST = crate::Reg<u32, _PWR_CTRL_SM_ST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWR_CTRL_SM_ST;
#[doc = "`read()` method returns [pwr_ctrl_sm_st::R](pwr_ctrl_sm_st::R) reader structure"]
impl crate::Readable for PWR_CTRL_SM_ST {}
#[doc = "Link Layer Power Control FSM Status Register"]
pub mod pwr_ctrl_sm_st;
#[doc = "HVLDO Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hvldo_ctrl](hvldo_ctrl) module"]
pub type HVLDO_CTRL = crate::Reg<u32, _HVLDO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HVLDO_CTRL;
#[doc = "`read()` method returns [hvldo_ctrl::R](hvldo_ctrl::R) reader structure"]
impl crate::Readable for HVLDO_CTRL {}
#[doc = "`write(|w| ..)` method takes [hvldo_ctrl::W](hvldo_ctrl::W) writer structure"]
impl crate::Writable for HVLDO_CTRL {}
#[doc = "HVLDO Configuration register"]
pub mod hvldo_ctrl;
#[doc = "Radio Buck and Active regulator enable control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_en_ctrl](misc_en_ctrl) module"]
pub type MISC_EN_CTRL = crate::Reg<u32, _MISC_EN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_EN_CTRL;
#[doc = "`read()` method returns [misc_en_ctrl::R](misc_en_ctrl::R) reader structure"]
impl crate::Readable for MISC_EN_CTRL {}
#[doc = "`write(|w| ..)` method takes [misc_en_ctrl::W](misc_en_ctrl::W) writer structure"]
impl crate::Writable for MISC_EN_CTRL {}
#[doc = "Radio Buck and Active regulator enable control"]
pub mod misc_en_ctrl;
#[doc = "EFUSE mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_config](efuse_config) module"]
pub type EFUSE_CONFIG = crate::Reg<u32, _EFUSE_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_CONFIG;
#[doc = "`read()` method returns [efuse_config::R](efuse_config::R) reader structure"]
impl crate::Readable for EFUSE_CONFIG {}
#[doc = "`write(|w| ..)` method takes [efuse_config::W](efuse_config::W) writer structure"]
impl crate::Writable for EFUSE_CONFIG {}
#[doc = "EFUSE mode configuration register"]
pub mod efuse_config;
#[doc = "EFUSE timing control register (common for Program and Read modes)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl1](efuse_tim_ctrl1) module"]
pub type EFUSE_TIM_CTRL1 = crate::Reg<u32, _EFUSE_TIM_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_TIM_CTRL1;
#[doc = "`read()` method returns [efuse_tim_ctrl1::R](efuse_tim_ctrl1::R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl1::W](efuse_tim_ctrl1::W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL1 {}
#[doc = "EFUSE timing control register (common for Program and Read modes)"]
pub mod efuse_tim_ctrl1;
#[doc = "EFUSE timing control Register (for Read)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl2](efuse_tim_ctrl2) module"]
pub type EFUSE_TIM_CTRL2 = crate::Reg<u32, _EFUSE_TIM_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_TIM_CTRL2;
#[doc = "`read()` method returns [efuse_tim_ctrl2::R](efuse_tim_ctrl2::R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl2::W](efuse_tim_ctrl2::W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL2 {}
#[doc = "EFUSE timing control Register (for Read)"]
pub mod efuse_tim_ctrl2;
#[doc = "EFUSE timing control Register (for Program)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_tim_ctrl3](efuse_tim_ctrl3) module"]
pub type EFUSE_TIM_CTRL3 = crate::Reg<u32, _EFUSE_TIM_CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_TIM_CTRL3;
#[doc = "`read()` method returns [efuse_tim_ctrl3::R](efuse_tim_ctrl3::R) reader structure"]
impl crate::Readable for EFUSE_TIM_CTRL3 {}
#[doc = "`write(|w| ..)` method takes [efuse_tim_ctrl3::W](efuse_tim_ctrl3::W) writer structure"]
impl crate::Writable for EFUSE_TIM_CTRL3 {}
#[doc = "EFUSE timing control Register (for Program)"]
pub mod efuse_tim_ctrl3;
#[doc = "EFUSE Lower read data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rdata_l](efuse_rdata_l) module"]
pub type EFUSE_RDATA_L = crate::Reg<u32, _EFUSE_RDATA_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RDATA_L;
#[doc = "`read()` method returns [efuse_rdata_l::R](efuse_rdata_l::R) reader structure"]
impl crate::Readable for EFUSE_RDATA_L {}
#[doc = "EFUSE Lower read data"]
pub mod efuse_rdata_l;
#[doc = "EFUSE higher read data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rdata_h](efuse_rdata_h) module"]
pub type EFUSE_RDATA_H = crate::Reg<u32, _EFUSE_RDATA_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_RDATA_H;
#[doc = "`read()` method returns [efuse_rdata_h::R](efuse_rdata_h::R) reader structure"]
impl crate::Readable for EFUSE_RDATA_H {}
#[doc = "EFUSE higher read data"]
pub mod efuse_rdata_h;
#[doc = "EFUSE lower write word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_wdata_l](efuse_wdata_l) module"]
pub type EFUSE_WDATA_L = crate::Reg<u32, _EFUSE_WDATA_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_WDATA_L;
#[doc = "`read()` method returns [efuse_wdata_l::R](efuse_wdata_l::R) reader structure"]
impl crate::Readable for EFUSE_WDATA_L {}
#[doc = "`write(|w| ..)` method takes [efuse_wdata_l::W](efuse_wdata_l::W) writer structure"]
impl crate::Writable for EFUSE_WDATA_L {}
#[doc = "EFUSE lower write word"]
pub mod efuse_wdata_l;
#[doc = "EFUSE higher write word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_wdata_h](efuse_wdata_h) module"]
pub type EFUSE_WDATA_H = crate::Reg<u32, _EFUSE_WDATA_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE_WDATA_H;
#[doc = "`read()` method returns [efuse_wdata_h::R](efuse_wdata_h::R) reader structure"]
impl crate::Readable for EFUSE_WDATA_H {}
#[doc = "`write(|w| ..)` method takes [efuse_wdata_h::W](efuse_wdata_h::W) writer structure"]
impl crate::Writable for EFUSE_WDATA_H {}
#[doc = "EFUSE higher write word"]
pub mod efuse_wdata_h;
#[doc = "Divide by 625 for FW Use\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_by_625_cfg](div_by_625_cfg) module"]
pub type DIV_BY_625_CFG = crate::Reg<u32, _DIV_BY_625_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_BY_625_CFG;
#[doc = "`read()` method returns [div_by_625_cfg::R](div_by_625_cfg::R) reader structure"]
impl crate::Readable for DIV_BY_625_CFG {}
#[doc = "`write(|w| ..)` method takes [div_by_625_cfg::W](div_by_625_cfg::W) writer structure"]
impl crate::Writable for DIV_BY_625_CFG {}
#[doc = "Divide by 625 for FW Use"]
pub mod div_by_625_cfg;
#[doc = "Output of divide by 625 divider\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_by_625_sts](div_by_625_sts) module"]
pub type DIV_BY_625_STS = crate::Reg<u32, _DIV_BY_625_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV_BY_625_STS;
#[doc = "`read()` method returns [div_by_625_sts::R](div_by_625_sts::R) reader structure"]
impl crate::Readable for DIV_BY_625_STS {}
#[doc = "Output of divide by 625 divider"]
pub mod div_by_625_sts;
#[doc = "Packet counter 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_counter0](packet_counter0) module"]
pub type PACKET_COUNTER0 = crate::Reg<u32, _PACKET_COUNTER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET_COUNTER0;
#[doc = "`read()` method returns [packet_counter0::R](packet_counter0::R) reader structure"]
impl crate::Readable for PACKET_COUNTER0 {}
#[doc = "`write(|w| ..)` method takes [packet_counter0::W](packet_counter0::W) writer structure"]
impl crate::Writable for PACKET_COUNTER0 {}
#[doc = "Packet counter 0"]
pub mod packet_counter0;
#[doc = "Packet counter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_counter2](packet_counter2) module"]
pub type PACKET_COUNTER2 = crate::Reg<u32, _PACKET_COUNTER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKET_COUNTER2;
#[doc = "`read()` method returns [packet_counter2::R](packet_counter2::R) reader structure"]
impl crate::Readable for PACKET_COUNTER2 {}
#[doc = "`write(|w| ..)` method takes [packet_counter2::W](packet_counter2::W) writer structure"]
impl crate::Writable for PACKET_COUNTER2 {}
#[doc = "Packet counter 2"]
pub mod packet_counter2;
#[doc = "Master Initialization Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_master0](iv_master0) module"]
pub type IV_MASTER0 = crate::Reg<u32, _IV_MASTER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_MASTER0;
#[doc = "`read()` method returns [iv_master0::R](iv_master0::R) reader structure"]
impl crate::Readable for IV_MASTER0 {}
#[doc = "`write(|w| ..)` method takes [iv_master0::W](iv_master0::W) writer structure"]
impl crate::Writable for IV_MASTER0 {}
#[doc = "Master Initialization Vector 0"]
pub mod iv_master0;
#[doc = "Slave Initialization Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_slave0](iv_slave0) module"]
pub type IV_SLAVE0 = crate::Reg<u32, _IV_SLAVE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_SLAVE0;
#[doc = "`read()` method returns [iv_slave0::R](iv_slave0::R) reader structure"]
impl crate::Readable for IV_SLAVE0 {}
#[doc = "`write(|w| ..)` method takes [iv_slave0::W](iv_slave0::W) writer structure"]
impl crate::Writable for IV_SLAVE0 {}
#[doc = "Slave Initialization Vector 0"]
pub mod iv_slave0;
#[doc = "Encryption Key register 0-3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_key](enc_key) module"]
pub type ENC_KEY = crate::Reg<u32, _ENC_KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_KEY;
#[doc = "`write(|w| ..)` method takes [enc_key::W](enc_key::W) writer structure"]
impl crate::Writable for ENC_KEY {}
#[doc = "Encryption Key register 0-3"]
pub mod enc_key;
#[doc = "MIC input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mic_in0](mic_in0) module"]
pub type MIC_IN0 = crate::Reg<u32, _MIC_IN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIC_IN0;
#[doc = "`read()` method returns [mic_in0::R](mic_in0::R) reader structure"]
impl crate::Readable for MIC_IN0 {}
#[doc = "`write(|w| ..)` method takes [mic_in0::W](mic_in0::W) writer structure"]
impl crate::Writable for MIC_IN0 {}
#[doc = "MIC input register"]
pub mod mic_in0;
#[doc = "MIC output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mic_out0](mic_out0) module"]
pub type MIC_OUT0 = crate::Reg<u32, _MIC_OUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIC_OUT0;
#[doc = "`read()` method returns [mic_out0::R](mic_out0::R) reader structure"]
impl crate::Readable for MIC_OUT0 {}
#[doc = "MIC output register"]
pub mod mic_out0;
#[doc = "Encryption Parameter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_params](enc_params) module"]
pub type ENC_PARAMS = crate::Reg<u32, _ENC_PARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_PARAMS;
#[doc = "`read()` method returns [enc_params::R](enc_params::R) reader structure"]
impl crate::Readable for ENC_PARAMS {}
#[doc = "`write(|w| ..)` method takes [enc_params::W](enc_params::W) writer structure"]
impl crate::Writable for ENC_PARAMS {}
#[doc = "Encryption Parameter register"]
pub mod enc_params;
#[doc = "Encryption Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_config](enc_config) module"]
pub type ENC_CONFIG = crate::Reg<u32, _ENC_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_CONFIG;
#[doc = "`read()` method returns [enc_config::R](enc_config::R) reader structure"]
impl crate::Readable for ENC_CONFIG {}
#[doc = "`write(|w| ..)` method takes [enc_config::W](enc_config::W) writer structure"]
impl crate::Writable for ENC_CONFIG {}
#[doc = "Encryption Configuration"]
pub mod enc_config;
#[doc = "Encryption Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_intr_en](enc_intr_en) module"]
pub type ENC_INTR_EN = crate::Reg<u32, _ENC_INTR_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_INTR_EN;
#[doc = "`read()` method returns [enc_intr_en::R](enc_intr_en::R) reader structure"]
impl crate::Readable for ENC_INTR_EN {}
#[doc = "`write(|w| ..)` method takes [enc_intr_en::W](enc_intr_en::W) writer structure"]
impl crate::Writable for ENC_INTR_EN {}
#[doc = "Encryption Interrupt enable"]
pub mod enc_intr_en;
#[doc = "Encryption Interrupt status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_intr](enc_intr) module"]
pub type ENC_INTR = crate::Reg<u32, _ENC_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_INTR;
#[doc = "`read()` method returns [enc_intr::R](enc_intr::R) reader structure"]
impl crate::Readable for ENC_INTR {}
#[doc = "`write(|w| ..)` method takes [enc_intr::W](enc_intr::W) writer structure"]
impl crate::Writable for ENC_INTR {}
#[doc = "Encryption Interrupt status and clear register"]
pub mod enc_intr;
#[doc = "Programmable B1 Data register (0-3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1_data_reg](b1_data_reg) module"]
pub type B1_DATA_REG = crate::Reg<u32, _B1_DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B1_DATA_REG;
#[doc = "`read()` method returns [b1_data_reg::R](b1_data_reg::R) reader structure"]
impl crate::Readable for B1_DATA_REG {}
#[doc = "`write(|w| ..)` method takes [b1_data_reg::W](b1_data_reg::W) writer structure"]
impl crate::Writable for B1_DATA_REG {}
#[doc = "Programmable B1 Data register (0-3)"]
pub mod b1_data_reg;
#[doc = "Encryption memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_mem_base_addr](enc_mem_base_addr) module"]
pub type ENC_MEM_BASE_ADDR = crate::Reg<u32, _ENC_MEM_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_MEM_BASE_ADDR;
#[doc = "`read()` method returns [enc_mem_base_addr::R](enc_mem_base_addr::R) reader structure"]
impl crate::Readable for ENC_MEM_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [enc_mem_base_addr::W](enc_mem_base_addr::W) writer structure"]
impl crate::Writable for ENC_MEM_BASE_ADDR {}
#[doc = "Encryption memory base address"]
pub mod enc_mem_base_addr;
#[doc = "LDO Trim register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_0](trim_ldo_0) module"]
pub type TRIM_LDO_0 = crate::Reg<u32, _TRIM_LDO_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_0;
#[doc = "`read()` method returns [trim_ldo_0::R](trim_ldo_0::R) reader structure"]
impl crate::Readable for TRIM_LDO_0 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_0::W](trim_ldo_0::W) writer structure"]
impl crate::Writable for TRIM_LDO_0 {}
#[doc = "LDO Trim register 0"]
pub mod trim_ldo_0;
#[doc = "LDO Trim register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_1](trim_ldo_1) module"]
pub type TRIM_LDO_1 = crate::Reg<u32, _TRIM_LDO_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_1;
#[doc = "`read()` method returns [trim_ldo_1::R](trim_ldo_1::R) reader structure"]
impl crate::Readable for TRIM_LDO_1 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_1::W](trim_ldo_1::W) writer structure"]
impl crate::Writable for TRIM_LDO_1 {}
#[doc = "LDO Trim register 1"]
pub mod trim_ldo_1;
#[doc = "LDO Trim register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_2](trim_ldo_2) module"]
pub type TRIM_LDO_2 = crate::Reg<u32, _TRIM_LDO_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_2;
#[doc = "`read()` method returns [trim_ldo_2::R](trim_ldo_2::R) reader structure"]
impl crate::Readable for TRIM_LDO_2 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_2::W](trim_ldo_2::W) writer structure"]
impl crate::Writable for TRIM_LDO_2 {}
#[doc = "LDO Trim register 2"]
pub mod trim_ldo_2;
#[doc = "LDO Trim register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_3](trim_ldo_3) module"]
pub type TRIM_LDO_3 = crate::Reg<u32, _TRIM_LDO_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_3;
#[doc = "`read()` method returns [trim_ldo_3::R](trim_ldo_3::R) reader structure"]
impl crate::Readable for TRIM_LDO_3 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_3::W](trim_ldo_3::W) writer structure"]
impl crate::Writable for TRIM_LDO_3 {}
#[doc = "LDO Trim register 3"]
pub mod trim_ldo_3;
#[doc = "MXD die Trim registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_mxd](trim_mxd) module"]
pub type TRIM_MXD = crate::Reg<u32, _TRIM_MXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_MXD;
#[doc = "`read()` method returns [trim_mxd::R](trim_mxd::R) reader structure"]
impl crate::Readable for TRIM_MXD {}
#[doc = "`write(|w| ..)` method takes [trim_mxd::W](trim_mxd::W) writer structure"]
impl crate::Writable for TRIM_MXD {}
#[doc = "MXD die Trim registers"]
pub mod trim_mxd;
#[doc = "LDO Trim register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_4](trim_ldo_4) module"]
pub type TRIM_LDO_4 = crate::Reg<u32, _TRIM_LDO_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_4;
#[doc = "`read()` method returns [trim_ldo_4::R](trim_ldo_4::R) reader structure"]
impl crate::Readable for TRIM_LDO_4 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_4::W](trim_ldo_4::W) writer structure"]
impl crate::Writable for TRIM_LDO_4 {}
#[doc = "LDO Trim register 4"]
pub mod trim_ldo_4;
#[doc = "LDO Trim register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_5](trim_ldo_5) module"]
pub type TRIM_LDO_5 = crate::Reg<u32, _TRIM_LDO_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_LDO_5;
#[doc = "`read()` method returns [trim_ldo_5::R](trim_ldo_5::R) reader structure"]
impl crate::Readable for TRIM_LDO_5 {}
#[doc = "`write(|w| ..)` method takes [trim_ldo_5::W](trim_ldo_5::W) writer structure"]
impl crate::Writable for TRIM_LDO_5 {}
#[doc = "LDO Trim register 5"]
pub mod trim_ldo_5;
