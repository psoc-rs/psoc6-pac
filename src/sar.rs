#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog control register."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Sample control register."]
    pub sample_ctrl: crate::Reg<sample_ctrl::SAMPLE_CTRL_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Sample time specification ST0 and ST1"]
    pub sample_time01: crate::Reg<sample_time01::SAMPLE_TIME01_SPEC>,
    #[doc = "0x14 - Sample time specification ST2 and ST3"]
    pub sample_time23: crate::Reg<sample_time23::SAMPLE_TIME23_SPEC>,
    #[doc = "0x18 - Global range detect threshold register."]
    pub range_thres: crate::Reg<range_thres::RANGE_THRES_SPEC>,
    #[doc = "0x1c - Global range detect mode register."]
    pub range_cond: crate::Reg<range_cond::RANGE_COND_SPEC>,
    #[doc = "0x20 - Enable bits for the channels"]
    pub chan_en: crate::Reg<chan_en::CHAN_EN_SPEC>,
    #[doc = "0x24 - Start control register (firmware trigger)."]
    pub start_ctrl: crate::Reg<start_ctrl::START_CTRL_SPEC>,
    _reserved8: [u8; 0x58],
    #[doc = "0x80..0xc0 - Channel configuration register."]
    pub chan_config: [crate::Reg<chan_config::CHAN_CONFIG_SPEC>; 16],
    _reserved9: [u8; 0x40],
    #[doc = "0x100..0x140 - Channel working data register"]
    pub chan_work: [crate::Reg<chan_work::CHAN_WORK_SPEC>; 16],
    _reserved10: [u8; 0x40],
    #[doc = "0x180..0x1c0 - Channel result data register"]
    pub chan_result: [crate::Reg<chan_result::CHAN_RESULT_SPEC>; 16],
    _reserved11: [u8; 0x40],
    #[doc = "0x200 - Channel working data register 'updated' bits"]
    pub chan_work_updated: crate::Reg<chan_work_updated::CHAN_WORK_UPDATED_SPEC>,
    #[doc = "0x204 - Channel result data register 'updated' bits"]
    pub chan_result_updated: crate::Reg<chan_result_updated::CHAN_RESULT_UPDATED_SPEC>,
    #[doc = "0x208 - Channel working data register 'new value' bits"]
    pub chan_work_newvalue: crate::Reg<chan_work_newvalue::CHAN_WORK_NEWVALUE_SPEC>,
    #[doc = "0x20c - Channel result data register 'new value' bits"]
    pub chan_result_newvalue: crate::Reg<chan_result_newvalue::CHAN_RESULT_NEWVALUE_SPEC>,
    #[doc = "0x210 - Interrupt request register."]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x214 - Interrupt set request register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x218 - Interrupt mask register."]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x21c - Interrupt masked request register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x220 - Saturate interrupt request register."]
    pub saturate_intr: crate::Reg<saturate_intr::SATURATE_INTR_SPEC>,
    #[doc = "0x224 - Saturate interrupt set request register"]
    pub saturate_intr_set: crate::Reg<saturate_intr_set::SATURATE_INTR_SET_SPEC>,
    #[doc = "0x228 - Saturate interrupt mask register."]
    pub saturate_intr_mask: crate::Reg<saturate_intr_mask::SATURATE_INTR_MASK_SPEC>,
    #[doc = "0x22c - Saturate interrupt masked request register"]
    pub saturate_intr_masked: crate::Reg<saturate_intr_masked::SATURATE_INTR_MASKED_SPEC>,
    #[doc = "0x230 - Range detect interrupt request register."]
    pub range_intr: crate::Reg<range_intr::RANGE_INTR_SPEC>,
    #[doc = "0x234 - Range detect interrupt set request register"]
    pub range_intr_set: crate::Reg<range_intr_set::RANGE_INTR_SET_SPEC>,
    #[doc = "0x238 - Range detect interrupt mask register."]
    pub range_intr_mask: crate::Reg<range_intr_mask::RANGE_INTR_MASK_SPEC>,
    #[doc = "0x23c - Range interrupt masked request register"]
    pub range_intr_masked: crate::Reg<range_intr_masked::RANGE_INTR_MASKED_SPEC>,
    #[doc = "0x240 - Interrupt cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved28: [u8; 0x3c],
    #[doc = "0x280 - Injection channel configuration register."]
    pub inj_chan_config: crate::Reg<inj_chan_config::INJ_CHAN_CONFIG_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x290 - Injection channel result register"]
    pub inj_result: crate::Reg<inj_result::INJ_RESULT_SPEC>,
    _reserved30: [u8; 0x0c],
    #[doc = "0x2a0 - Current status of internal SAR registers (mostly for debug)"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x2a4 - Current averaging status (for debug)"]
    pub avg_stat: crate::Reg<avg_stat::AVG_STAT_SPEC>,
    _reserved32: [u8; 0x58],
    #[doc = "0x300 - SARMUX Firmware switch controls"]
    pub mux_switch0: crate::Reg<mux_switch0::MUX_SWITCH0_SPEC>,
    #[doc = "0x304 - SARMUX Firmware switch control clear"]
    pub mux_switch_clear0: crate::Reg<mux_switch_clear0::MUX_SWITCH_CLEAR0_SPEC>,
    _reserved34: [u8; 0x38],
    #[doc = "0x340 - SARMUX switch DSI control"]
    pub mux_switch_ds_ctrl: crate::Reg<mux_switch_ds_ctrl::MUX_SWITCH_DS_CTRL_SPEC>,
    #[doc = "0x344 - SARMUX switch Sar Sequencer control"]
    pub mux_switch_sq_ctrl: crate::Reg<mux_switch_sq_ctrl::MUX_SWITCH_SQ_CTRL_SPEC>,
    #[doc = "0x348 - SARMUX switch status"]
    pub mux_switch_status: crate::Reg<mux_switch_status::MUX_SWITCH_STATUS_SPEC>,
    _reserved37: [u8; 0x0bb4],
    #[doc = "0xf00 - Analog trim register."]
    pub ana_trim0: crate::Reg<ana_trim0::ANA_TRIM0_SPEC>,
    #[doc = "0xf04 - Analog trim register."]
    pub ana_trim1: crate::Reg<ana_trim1::ANA_TRIM1_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Analog control register."]
pub mod ctrl;
#[doc = "SAMPLE_CTRL register accessor: an alias for `Reg<SAMPLE_CTRL_SPEC>`"]
pub type SAMPLE_CTRL = crate::Reg<sample_ctrl::SAMPLE_CTRL_SPEC>;
#[doc = "Sample control register."]
pub mod sample_ctrl;
#[doc = "SAMPLE_TIME01 register accessor: an alias for `Reg<SAMPLE_TIME01_SPEC>`"]
pub type SAMPLE_TIME01 = crate::Reg<sample_time01::SAMPLE_TIME01_SPEC>;
#[doc = "Sample time specification ST0 and ST1"]
pub mod sample_time01;
#[doc = "SAMPLE_TIME23 register accessor: an alias for `Reg<SAMPLE_TIME23_SPEC>`"]
pub type SAMPLE_TIME23 = crate::Reg<sample_time23::SAMPLE_TIME23_SPEC>;
#[doc = "Sample time specification ST2 and ST3"]
pub mod sample_time23;
#[doc = "RANGE_THRES register accessor: an alias for `Reg<RANGE_THRES_SPEC>`"]
pub type RANGE_THRES = crate::Reg<range_thres::RANGE_THRES_SPEC>;
#[doc = "Global range detect threshold register."]
pub mod range_thres;
#[doc = "RANGE_COND register accessor: an alias for `Reg<RANGE_COND_SPEC>`"]
pub type RANGE_COND = crate::Reg<range_cond::RANGE_COND_SPEC>;
#[doc = "Global range detect mode register."]
pub mod range_cond;
#[doc = "CHAN_EN register accessor: an alias for `Reg<CHAN_EN_SPEC>`"]
pub type CHAN_EN = crate::Reg<chan_en::CHAN_EN_SPEC>;
#[doc = "Enable bits for the channels"]
pub mod chan_en;
#[doc = "START_CTRL register accessor: an alias for `Reg<START_CTRL_SPEC>`"]
pub type START_CTRL = crate::Reg<start_ctrl::START_CTRL_SPEC>;
#[doc = "Start control register (firmware trigger)."]
pub mod start_ctrl;
#[doc = "CHAN_CONFIG register accessor: an alias for `Reg<CHAN_CONFIG_SPEC>`"]
pub type CHAN_CONFIG = crate::Reg<chan_config::CHAN_CONFIG_SPEC>;
#[doc = "Channel configuration register."]
pub mod chan_config;
#[doc = "CHAN_WORK register accessor: an alias for `Reg<CHAN_WORK_SPEC>`"]
pub type CHAN_WORK = crate::Reg<chan_work::CHAN_WORK_SPEC>;
#[doc = "Channel working data register"]
pub mod chan_work;
#[doc = "CHAN_RESULT register accessor: an alias for `Reg<CHAN_RESULT_SPEC>`"]
pub type CHAN_RESULT = crate::Reg<chan_result::CHAN_RESULT_SPEC>;
#[doc = "Channel result data register"]
pub mod chan_result;
#[doc = "CHAN_WORK_UPDATED register accessor: an alias for `Reg<CHAN_WORK_UPDATED_SPEC>`"]
pub type CHAN_WORK_UPDATED = crate::Reg<chan_work_updated::CHAN_WORK_UPDATED_SPEC>;
#[doc = "Channel working data register 'updated' bits"]
pub mod chan_work_updated;
#[doc = "CHAN_RESULT_UPDATED register accessor: an alias for `Reg<CHAN_RESULT_UPDATED_SPEC>`"]
pub type CHAN_RESULT_UPDATED = crate::Reg<chan_result_updated::CHAN_RESULT_UPDATED_SPEC>;
#[doc = "Channel result data register 'updated' bits"]
pub mod chan_result_updated;
#[doc = "CHAN_WORK_NEWVALUE register accessor: an alias for `Reg<CHAN_WORK_NEWVALUE_SPEC>`"]
pub type CHAN_WORK_NEWVALUE = crate::Reg<chan_work_newvalue::CHAN_WORK_NEWVALUE_SPEC>;
#[doc = "Channel working data register 'new value' bits"]
pub mod chan_work_newvalue;
#[doc = "CHAN_RESULT_NEWVALUE register accessor: an alias for `Reg<CHAN_RESULT_NEWVALUE_SPEC>`"]
pub type CHAN_RESULT_NEWVALUE = crate::Reg<chan_result_newvalue::CHAN_RESULT_NEWVALUE_SPEC>;
#[doc = "Channel result data register 'new value' bits"]
pub mod chan_result_newvalue;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register."]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register."]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "SATURATE_INTR register accessor: an alias for `Reg<SATURATE_INTR_SPEC>`"]
pub type SATURATE_INTR = crate::Reg<saturate_intr::SATURATE_INTR_SPEC>;
#[doc = "Saturate interrupt request register."]
pub mod saturate_intr;
#[doc = "SATURATE_INTR_SET register accessor: an alias for `Reg<SATURATE_INTR_SET_SPEC>`"]
pub type SATURATE_INTR_SET = crate::Reg<saturate_intr_set::SATURATE_INTR_SET_SPEC>;
#[doc = "Saturate interrupt set request register"]
pub mod saturate_intr_set;
#[doc = "SATURATE_INTR_MASK register accessor: an alias for `Reg<SATURATE_INTR_MASK_SPEC>`"]
pub type SATURATE_INTR_MASK = crate::Reg<saturate_intr_mask::SATURATE_INTR_MASK_SPEC>;
#[doc = "Saturate interrupt mask register."]
pub mod saturate_intr_mask;
#[doc = "SATURATE_INTR_MASKED register accessor: an alias for `Reg<SATURATE_INTR_MASKED_SPEC>`"]
pub type SATURATE_INTR_MASKED = crate::Reg<saturate_intr_masked::SATURATE_INTR_MASKED_SPEC>;
#[doc = "Saturate interrupt masked request register"]
pub mod saturate_intr_masked;
#[doc = "RANGE_INTR register accessor: an alias for `Reg<RANGE_INTR_SPEC>`"]
pub type RANGE_INTR = crate::Reg<range_intr::RANGE_INTR_SPEC>;
#[doc = "Range detect interrupt request register."]
pub mod range_intr;
#[doc = "RANGE_INTR_SET register accessor: an alias for `Reg<RANGE_INTR_SET_SPEC>`"]
pub type RANGE_INTR_SET = crate::Reg<range_intr_set::RANGE_INTR_SET_SPEC>;
#[doc = "Range detect interrupt set request register"]
pub mod range_intr_set;
#[doc = "RANGE_INTR_MASK register accessor: an alias for `Reg<RANGE_INTR_MASK_SPEC>`"]
pub type RANGE_INTR_MASK = crate::Reg<range_intr_mask::RANGE_INTR_MASK_SPEC>;
#[doc = "Range detect interrupt mask register."]
pub mod range_intr_mask;
#[doc = "RANGE_INTR_MASKED register accessor: an alias for `Reg<RANGE_INTR_MASKED_SPEC>`"]
pub type RANGE_INTR_MASKED = crate::Reg<range_intr_masked::RANGE_INTR_MASKED_SPEC>;
#[doc = "Range interrupt masked request register"]
pub mod range_intr_masked;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "INJ_CHAN_CONFIG register accessor: an alias for `Reg<INJ_CHAN_CONFIG_SPEC>`"]
pub type INJ_CHAN_CONFIG = crate::Reg<inj_chan_config::INJ_CHAN_CONFIG_SPEC>;
#[doc = "Injection channel configuration register."]
pub mod inj_chan_config;
#[doc = "INJ_RESULT register accessor: an alias for `Reg<INJ_RESULT_SPEC>`"]
pub type INJ_RESULT = crate::Reg<inj_result::INJ_RESULT_SPEC>;
#[doc = "Injection channel result register"]
pub mod inj_result;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "AVG_STAT register accessor: an alias for `Reg<AVG_STAT_SPEC>`"]
pub type AVG_STAT = crate::Reg<avg_stat::AVG_STAT_SPEC>;
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "MUX_SWITCH0 register accessor: an alias for `Reg<MUX_SWITCH0_SPEC>`"]
pub type MUX_SWITCH0 = crate::Reg<mux_switch0::MUX_SWITCH0_SPEC>;
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch0;
#[doc = "MUX_SWITCH_CLEAR0 register accessor: an alias for `Reg<MUX_SWITCH_CLEAR0_SPEC>`"]
pub type MUX_SWITCH_CLEAR0 = crate::Reg<mux_switch_clear0::MUX_SWITCH_CLEAR0_SPEC>;
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear0;
#[doc = "MUX_SWITCH_DS_CTRL register accessor: an alias for `Reg<MUX_SWITCH_DS_CTRL_SPEC>`"]
pub type MUX_SWITCH_DS_CTRL = crate::Reg<mux_switch_ds_ctrl::MUX_SWITCH_DS_CTRL_SPEC>;
#[doc = "SARMUX switch DSI control"]
pub mod mux_switch_ds_ctrl;
#[doc = "MUX_SWITCH_SQ_CTRL register accessor: an alias for `Reg<MUX_SWITCH_SQ_CTRL_SPEC>`"]
pub type MUX_SWITCH_SQ_CTRL = crate::Reg<mux_switch_sq_ctrl::MUX_SWITCH_SQ_CTRL_SPEC>;
#[doc = "SARMUX switch Sar Sequencer control"]
pub mod mux_switch_sq_ctrl;
#[doc = "MUX_SWITCH_STATUS register accessor: an alias for `Reg<MUX_SWITCH_STATUS_SPEC>`"]
pub type MUX_SWITCH_STATUS = crate::Reg<mux_switch_status::MUX_SWITCH_STATUS_SPEC>;
#[doc = "SARMUX switch status"]
pub mod mux_switch_status;
#[doc = "ANA_TRIM0 register accessor: an alias for `Reg<ANA_TRIM0_SPEC>`"]
pub type ANA_TRIM0 = crate::Reg<ana_trim0::ANA_TRIM0_SPEC>;
#[doc = "Analog trim register."]
pub mod ana_trim0;
#[doc = "ANA_TRIM1 register accessor: an alias for `Reg<ANA_TRIM1_SPEC>`"]
pub type ANA_TRIM1 = crate::Reg<ana_trim1::ANA_TRIM1_SPEC>;
#[doc = "Analog trim register."]
pub mod ana_trim1;
