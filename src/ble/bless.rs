#[doc = "DDFT_CONFIG register accessor: an alias for `Reg<DDFT_CONFIG_SPEC>`"]
pub type DDFT_CONFIG = crate::Reg<ddft_config::DDFT_CONFIG_SPEC>;
#[doc = "BLESS DDFT configuration register"]
pub mod ddft_config;
#[doc = "XTAL_CLK_DIV_CONFIG register accessor: an alias for `Reg<XTAL_CLK_DIV_CONFIG_SPEC>`"]
pub type XTAL_CLK_DIV_CONFIG = crate::Reg<xtal_clk_div_config::XTAL_CLK_DIV_CONFIG_SPEC>;
#[doc = "Crystal clock divider configuration register"]
pub mod xtal_clk_div_config;
#[doc = "INTR_STAT register accessor: an alias for `Reg<INTR_STAT_SPEC>`"]
pub type INTR_STAT = crate::Reg<intr_stat::INTR_STAT_SPEC>;
#[doc = "Link Layer interrupt status register"]
pub mod intr_stat;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Link Layer interrupt mask register"]
pub mod intr_mask;
#[doc = "LL_CLK_EN register accessor: an alias for `Reg<LL_CLK_EN_SPEC>`"]
pub type LL_CLK_EN = crate::Reg<ll_clk_en::LL_CLK_EN_SPEC>;
#[doc = "Link Layer primary clock enable"]
pub mod ll_clk_en;
#[doc = "LF_CLK_CTRL register accessor: an alias for `Reg<LF_CLK_CTRL_SPEC>`"]
pub type LF_CLK_CTRL = crate::Reg<lf_clk_ctrl::LF_CLK_CTRL_SPEC>;
#[doc = "BLESS LF clock control and BLESS revision ID indicator"]
pub mod lf_clk_ctrl;
#[doc = "EXT_PA_LNA_CTRL register accessor: an alias for `Reg<EXT_PA_LNA_CTRL_SPEC>`"]
pub type EXT_PA_LNA_CTRL = crate::Reg<ext_pa_lna_ctrl::EXT_PA_LNA_CTRL_SPEC>;
#[doc = "External TX PA and RX LNA control"]
pub mod ext_pa_lna_ctrl;
#[doc = "LL_PKT_RSSI_CH_ENERGY register accessor: an alias for `Reg<LL_PKT_RSSI_CH_ENERGY_SPEC>`"]
pub type LL_PKT_RSSI_CH_ENERGY = crate::Reg<ll_pkt_rssi_ch_energy::LL_PKT_RSSI_CH_ENERGY_SPEC>;
#[doc = "Link Layer Last Received packet RSSI/Channel energy and channel number"]
pub mod ll_pkt_rssi_ch_energy;
#[doc = "BT_CLOCK_CAPT register accessor: an alias for `Reg<BT_CLOCK_CAPT_SPEC>`"]
pub type BT_CLOCK_CAPT = crate::Reg<bt_clock_capt::BT_CLOCK_CAPT_SPEC>;
#[doc = "BT clock captured on an LL DSM exit"]
pub mod bt_clock_capt;
#[doc = "MT_CFG register accessor: an alias for `Reg<MT_CFG_SPEC>`"]
pub type MT_CFG = crate::Reg<mt_cfg::MT_CFG_SPEC>;
#[doc = "MT Configuration Register"]
pub mod mt_cfg;
#[doc = "MT_DELAY_CFG register accessor: an alias for `Reg<MT_DELAY_CFG_SPEC>`"]
pub type MT_DELAY_CFG = crate::Reg<mt_delay_cfg::MT_DELAY_CFG_SPEC>;
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg;
#[doc = "MT_DELAY_CFG2 register accessor: an alias for `Reg<MT_DELAY_CFG2_SPEC>`"]
pub type MT_DELAY_CFG2 = crate::Reg<mt_delay_cfg2::MT_DELAY_CFG2_SPEC>;
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg2;
#[doc = "MT_DELAY_CFG3 register accessor: an alias for `Reg<MT_DELAY_CFG3_SPEC>`"]
pub type MT_DELAY_CFG3 = crate::Reg<mt_delay_cfg3::MT_DELAY_CFG3_SPEC>;
#[doc = "MT Delay configuration for state transitions"]
pub mod mt_delay_cfg3;
#[doc = "MT_VIO_CTRL register accessor: an alias for `Reg<MT_VIO_CTRL_SPEC>`"]
pub type MT_VIO_CTRL = crate::Reg<mt_vio_ctrl::MT_VIO_CTRL_SPEC>;
#[doc = "MT Configuration Register to control VIO switches"]
pub mod mt_vio_ctrl;
#[doc = "MT_STATUS register accessor: an alias for `Reg<MT_STATUS_SPEC>`"]
pub type MT_STATUS = crate::Reg<mt_status::MT_STATUS_SPEC>;
#[doc = "MT Status Register"]
pub mod mt_status;
#[doc = "PWR_CTRL_SM_ST register accessor: an alias for `Reg<PWR_CTRL_SM_ST_SPEC>`"]
pub type PWR_CTRL_SM_ST = crate::Reg<pwr_ctrl_sm_st::PWR_CTRL_SM_ST_SPEC>;
#[doc = "Link Layer Power Control FSM Status Register"]
pub mod pwr_ctrl_sm_st;
#[doc = "HVLDO_CTRL register accessor: an alias for `Reg<HVLDO_CTRL_SPEC>`"]
pub type HVLDO_CTRL = crate::Reg<hvldo_ctrl::HVLDO_CTRL_SPEC>;
#[doc = "HVLDO Configuration register"]
pub mod hvldo_ctrl;
#[doc = "MISC_EN_CTRL register accessor: an alias for `Reg<MISC_EN_CTRL_SPEC>`"]
pub type MISC_EN_CTRL = crate::Reg<misc_en_ctrl::MISC_EN_CTRL_SPEC>;
#[doc = "Radio Buck and Active regulator enable control"]
pub mod misc_en_ctrl;
#[doc = "EFUSE_CONFIG register accessor: an alias for `Reg<EFUSE_CONFIG_SPEC>`"]
pub type EFUSE_CONFIG = crate::Reg<efuse_config::EFUSE_CONFIG_SPEC>;
#[doc = "EFUSE mode configuration register"]
pub mod efuse_config;
#[doc = "EFUSE_TIM_CTRL1 register accessor: an alias for `Reg<EFUSE_TIM_CTRL1_SPEC>`"]
pub type EFUSE_TIM_CTRL1 = crate::Reg<efuse_tim_ctrl1::EFUSE_TIM_CTRL1_SPEC>;
#[doc = "EFUSE timing control register (common for Program and Read modes)"]
pub mod efuse_tim_ctrl1;
#[doc = "EFUSE_TIM_CTRL2 register accessor: an alias for `Reg<EFUSE_TIM_CTRL2_SPEC>`"]
pub type EFUSE_TIM_CTRL2 = crate::Reg<efuse_tim_ctrl2::EFUSE_TIM_CTRL2_SPEC>;
#[doc = "EFUSE timing control Register (for Read)"]
pub mod efuse_tim_ctrl2;
#[doc = "EFUSE_TIM_CTRL3 register accessor: an alias for `Reg<EFUSE_TIM_CTRL3_SPEC>`"]
pub type EFUSE_TIM_CTRL3 = crate::Reg<efuse_tim_ctrl3::EFUSE_TIM_CTRL3_SPEC>;
#[doc = "EFUSE timing control Register (for Program)"]
pub mod efuse_tim_ctrl3;
#[doc = "EFUSE_RDATA_L register accessor: an alias for `Reg<EFUSE_RDATA_L_SPEC>`"]
pub type EFUSE_RDATA_L = crate::Reg<efuse_rdata_l::EFUSE_RDATA_L_SPEC>;
#[doc = "EFUSE Lower read data"]
pub mod efuse_rdata_l;
#[doc = "EFUSE_RDATA_H register accessor: an alias for `Reg<EFUSE_RDATA_H_SPEC>`"]
pub type EFUSE_RDATA_H = crate::Reg<efuse_rdata_h::EFUSE_RDATA_H_SPEC>;
#[doc = "EFUSE higher read data"]
pub mod efuse_rdata_h;
#[doc = "EFUSE_WDATA_L register accessor: an alias for `Reg<EFUSE_WDATA_L_SPEC>`"]
pub type EFUSE_WDATA_L = crate::Reg<efuse_wdata_l::EFUSE_WDATA_L_SPEC>;
#[doc = "EFUSE lower write word"]
pub mod efuse_wdata_l;
#[doc = "EFUSE_WDATA_H register accessor: an alias for `Reg<EFUSE_WDATA_H_SPEC>`"]
pub type EFUSE_WDATA_H = crate::Reg<efuse_wdata_h::EFUSE_WDATA_H_SPEC>;
#[doc = "EFUSE higher write word"]
pub mod efuse_wdata_h;
#[doc = "DIV_BY_625_CFG register accessor: an alias for `Reg<DIV_BY_625_CFG_SPEC>`"]
pub type DIV_BY_625_CFG = crate::Reg<div_by_625_cfg::DIV_BY_625_CFG_SPEC>;
#[doc = "Divide by 625 for FW Use"]
pub mod div_by_625_cfg;
#[doc = "DIV_BY_625_STS register accessor: an alias for `Reg<DIV_BY_625_STS_SPEC>`"]
pub type DIV_BY_625_STS = crate::Reg<div_by_625_sts::DIV_BY_625_STS_SPEC>;
#[doc = "Output of divide by 625 divider"]
pub mod div_by_625_sts;
#[doc = "PACKET_COUNTER0 register accessor: an alias for `Reg<PACKET_COUNTER0_SPEC>`"]
pub type PACKET_COUNTER0 = crate::Reg<packet_counter0::PACKET_COUNTER0_SPEC>;
#[doc = "Packet counter 0"]
pub mod packet_counter0;
#[doc = "PACKET_COUNTER2 register accessor: an alias for `Reg<PACKET_COUNTER2_SPEC>`"]
pub type PACKET_COUNTER2 = crate::Reg<packet_counter2::PACKET_COUNTER2_SPEC>;
#[doc = "Packet counter 2"]
pub mod packet_counter2;
#[doc = "IV_MASTER0 register accessor: an alias for `Reg<IV_MASTER0_SPEC>`"]
pub type IV_MASTER0 = crate::Reg<iv_master0::IV_MASTER0_SPEC>;
#[doc = "Master Initialization Vector 0"]
pub mod iv_master0;
#[doc = "IV_SLAVE0 register accessor: an alias for `Reg<IV_SLAVE0_SPEC>`"]
pub type IV_SLAVE0 = crate::Reg<iv_slave0::IV_SLAVE0_SPEC>;
#[doc = "Slave Initialization Vector 0"]
pub mod iv_slave0;
#[doc = "ENC_KEY register accessor: an alias for `Reg<ENC_KEY_SPEC>`"]
pub type ENC_KEY = crate::Reg<enc_key::ENC_KEY_SPEC>;
#[doc = "Encryption Key register 0-3"]
pub mod enc_key;
#[doc = "MIC_IN0 register accessor: an alias for `Reg<MIC_IN0_SPEC>`"]
pub type MIC_IN0 = crate::Reg<mic_in0::MIC_IN0_SPEC>;
#[doc = "MIC input register"]
pub mod mic_in0;
#[doc = "MIC_OUT0 register accessor: an alias for `Reg<MIC_OUT0_SPEC>`"]
pub type MIC_OUT0 = crate::Reg<mic_out0::MIC_OUT0_SPEC>;
#[doc = "MIC output register"]
pub mod mic_out0;
#[doc = "ENC_PARAMS register accessor: an alias for `Reg<ENC_PARAMS_SPEC>`"]
pub type ENC_PARAMS = crate::Reg<enc_params::ENC_PARAMS_SPEC>;
#[doc = "Encryption Parameter register"]
pub mod enc_params;
#[doc = "ENC_CONFIG register accessor: an alias for `Reg<ENC_CONFIG_SPEC>`"]
pub type ENC_CONFIG = crate::Reg<enc_config::ENC_CONFIG_SPEC>;
#[doc = "Encryption Configuration"]
pub mod enc_config;
#[doc = "ENC_INTR_EN register accessor: an alias for `Reg<ENC_INTR_EN_SPEC>`"]
pub type ENC_INTR_EN = crate::Reg<enc_intr_en::ENC_INTR_EN_SPEC>;
#[doc = "Encryption Interrupt enable"]
pub mod enc_intr_en;
#[doc = "ENC_INTR register accessor: an alias for `Reg<ENC_INTR_SPEC>`"]
pub type ENC_INTR = crate::Reg<enc_intr::ENC_INTR_SPEC>;
#[doc = "Encryption Interrupt status and clear register"]
pub mod enc_intr;
#[doc = "B1_DATA_REG register accessor: an alias for `Reg<B1_DATA_REG_SPEC>`"]
pub type B1_DATA_REG = crate::Reg<b1_data_reg::B1_DATA_REG_SPEC>;
#[doc = "Programmable B1 Data register (0-3)"]
pub mod b1_data_reg;
#[doc = "ENC_MEM_BASE_ADDR register accessor: an alias for `Reg<ENC_MEM_BASE_ADDR_SPEC>`"]
pub type ENC_MEM_BASE_ADDR = crate::Reg<enc_mem_base_addr::ENC_MEM_BASE_ADDR_SPEC>;
#[doc = "Encryption memory base address"]
pub mod enc_mem_base_addr;
#[doc = "TRIM_LDO_0 register accessor: an alias for `Reg<TRIM_LDO_0_SPEC>`"]
pub type TRIM_LDO_0 = crate::Reg<trim_ldo_0::TRIM_LDO_0_SPEC>;
#[doc = "LDO Trim register 0"]
pub mod trim_ldo_0;
#[doc = "TRIM_LDO_1 register accessor: an alias for `Reg<TRIM_LDO_1_SPEC>`"]
pub type TRIM_LDO_1 = crate::Reg<trim_ldo_1::TRIM_LDO_1_SPEC>;
#[doc = "LDO Trim register 1"]
pub mod trim_ldo_1;
#[doc = "TRIM_LDO_2 register accessor: an alias for `Reg<TRIM_LDO_2_SPEC>`"]
pub type TRIM_LDO_2 = crate::Reg<trim_ldo_2::TRIM_LDO_2_SPEC>;
#[doc = "LDO Trim register 2"]
pub mod trim_ldo_2;
#[doc = "TRIM_LDO_3 register accessor: an alias for `Reg<TRIM_LDO_3_SPEC>`"]
pub type TRIM_LDO_3 = crate::Reg<trim_ldo_3::TRIM_LDO_3_SPEC>;
#[doc = "LDO Trim register 3"]
pub mod trim_ldo_3;
#[doc = "TRIM_MXD register accessor: an alias for `Reg<TRIM_MXD_SPEC>`"]
pub type TRIM_MXD = crate::Reg<trim_mxd::TRIM_MXD_SPEC>;
#[doc = "MXD die Trim registers"]
pub mod trim_mxd;
#[doc = "TRIM_LDO_4 register accessor: an alias for `Reg<TRIM_LDO_4_SPEC>`"]
pub type TRIM_LDO_4 = crate::Reg<trim_ldo_4::TRIM_LDO_4_SPEC>;
#[doc = "LDO Trim register 4"]
pub mod trim_ldo_4;
#[doc = "TRIM_LDO_5 register accessor: an alias for `Reg<TRIM_LDO_5_SPEC>`"]
pub type TRIM_LDO_5 = crate::Reg<trim_ldo_5::TRIM_LDO_5_SPEC>;
#[doc = "LDO Trim register 5"]
pub mod trim_ldo_5;
