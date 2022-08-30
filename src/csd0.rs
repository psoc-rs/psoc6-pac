#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and Control"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - Spare MMIO"]
    pub spare: crate::Reg<spare::SPARE_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x84 - Current Sequencer status"]
    pub stat_seq: crate::Reg<stat_seq::STAT_SEQ_SPEC>,
    #[doc = "0x88 - Current status counts"]
    pub stat_cnts: crate::Reg<stat_cnts::STAT_CNTS_SPEC>,
    #[doc = "0x8c - Current count of the HSCMP counter"]
    pub stat_hcnt: crate::Reg<stat_hcnt::STAT_HCNT_SPEC>,
    _reserved6: [u8; 0x40],
    #[doc = "0xd0 - Result CSD/CSX accumulation counter value 1"]
    pub result_val1: crate::Reg<result_val1::RESULT_VAL1_SPEC>,
    #[doc = "0xd4 - Result CSX accumulation counter value 2"]
    pub result_val2: crate::Reg<result_val2::RESULT_VAL2_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0xe0 - ADC measurement"]
    pub adc_res: crate::Reg<adc_res::ADC_RES_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0xf0 - CSD Interrupt Request Register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0xf4 - CSD Interrupt set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0xf8 - CSD Interrupt mask register"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0xfc - CSD Interrupt masked register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    _reserved13: [u8; 0x80],
    #[doc = "0x180 - High Speed Comparator configuration"]
    pub hscmp: crate::Reg<hscmp::HSCMP_SPEC>,
    #[doc = "0x184 - Reference Generator configuration"]
    pub ambuf: crate::Reg<ambuf::AMBUF_SPEC>,
    #[doc = "0x188 - Reference Generator configuration"]
    pub refgen: crate::Reg<refgen::REFGEN_SPEC>,
    #[doc = "0x18c - CSD Comparator configuration"]
    pub csdcmp: crate::Reg<csdcmp::CSDCMP_SPEC>,
    _reserved17: [u8; 0x60],
    #[doc = "0x1f0 - Switch Resistance configuration"]
    pub sw_res: crate::Reg<sw_res::SW_RES_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x200 - Sense clock period"]
    pub sense_period: crate::Reg<sense_period::SENSE_PERIOD_SPEC>,
    #[doc = "0x204 - Sense clock duty cycle"]
    pub sense_duty: crate::Reg<sense_duty::SENSE_DUTY_SPEC>,
    _reserved20: [u8; 0x78],
    #[doc = "0x280 - HSCMP Pos input switch Waveform selection"]
    pub sw_hs_p_sel: crate::Reg<sw_hs_p_sel::SW_HS_P_SEL_SPEC>,
    #[doc = "0x284 - HSCMP Neg input switch Waveform selection"]
    pub sw_hs_n_sel: crate::Reg<sw_hs_n_sel::SW_HS_N_SEL_SPEC>,
    #[doc = "0x288 - Shielding switches Waveform selection"]
    pub sw_shield_sel: crate::Reg<sw_shield_sel::SW_SHIELD_SEL_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x290 - Amuxbuffer switches Waveform selection"]
    pub sw_amuxbuf_sel: crate::Reg<sw_amuxbuf_sel::SW_AMUXBUF_SEL_SPEC>,
    #[doc = "0x294 - AMUXBUS bypass switches Waveform selection"]
    pub sw_byp_sel: crate::Reg<sw_byp_sel::SW_BYP_SEL_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x2a0 - CSDCMP Pos Switch Waveform selection"]
    pub sw_cmp_p_sel: crate::Reg<sw_cmp_p_sel::SW_CMP_P_SEL_SPEC>,
    #[doc = "0x2a4 - CSDCMP Neg Switch Waveform selection"]
    pub sw_cmp_n_sel: crate::Reg<sw_cmp_n_sel::SW_CMP_N_SEL_SPEC>,
    #[doc = "0x2a8 - Reference Generator Switch Waveform selection"]
    pub sw_refgen_sel: crate::Reg<sw_refgen_sel::SW_REFGEN_SEL_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x2b0 - Full Wave Cmod Switch Waveform selection"]
    pub sw_fw_mod_sel: crate::Reg<sw_fw_mod_sel::SW_FW_MOD_SEL_SPEC>,
    #[doc = "0x2b4 - Full Wave Csh_tank Switch Waveform selection"]
    pub sw_fw_tank_sel: crate::Reg<sw_fw_tank_sel::SW_FW_TANK_SEL_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0x2c0 - DSI output switch control Waveform selection"]
    pub sw_dsi_sel: crate::Reg<sw_dsi_sel::SW_DSI_SEL_SPEC>,
    _reserved31: [u8; 0x0c],
    #[doc = "0x2d0 - IO output control Waveform selection"]
    pub io_sel: crate::Reg<io_sel::IO_SEL_SPEC>,
    _reserved32: [u8; 0x2c],
    #[doc = "0x300 - Sequencer Timing"]
    pub seq_time: crate::Reg<seq_time::SEQ_TIME_SPEC>,
    _reserved33: [u8; 0x0c],
    #[doc = "0x310 - Sequencer Initial conversion and sample counts"]
    pub seq_init_cnt: crate::Reg<seq_init_cnt::SEQ_INIT_CNT_SPEC>,
    #[doc = "0x314 - Sequencer Normal conversion and sample counts"]
    pub seq_norm_cnt: crate::Reg<seq_norm_cnt::SEQ_NORM_CNT_SPEC>,
    _reserved35: [u8; 0x08],
    #[doc = "0x320 - ADC Control"]
    pub adc_ctl: crate::Reg<adc_ctl::ADC_CTL_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0x340 - Sequencer start"]
    pub seq_start: crate::Reg<seq_start::SEQ_START_SPEC>,
    _reserved37: [u8; 0xbc],
    #[doc = "0x400 - IDACA Configuration"]
    pub idaca: crate::Reg<idaca::IDACA_SPEC>,
    _reserved38: [u8; 0xfc],
    #[doc = "0x500 - IDACB Configuration"]
    pub idacb: crate::Reg<idacb::IDACB_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration and Control"]
pub mod config;
#[doc = "SPARE register accessor: an alias for `Reg<SPARE_SPEC>`"]
pub type SPARE = crate::Reg<spare::SPARE_SPEC>;
#[doc = "Spare MMIO"]
pub mod spare;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "STAT_SEQ register accessor: an alias for `Reg<STAT_SEQ_SPEC>`"]
pub type STAT_SEQ = crate::Reg<stat_seq::STAT_SEQ_SPEC>;
#[doc = "Current Sequencer status"]
pub mod stat_seq;
#[doc = "STAT_CNTS register accessor: an alias for `Reg<STAT_CNTS_SPEC>`"]
pub type STAT_CNTS = crate::Reg<stat_cnts::STAT_CNTS_SPEC>;
#[doc = "Current status counts"]
pub mod stat_cnts;
#[doc = "STAT_HCNT register accessor: an alias for `Reg<STAT_HCNT_SPEC>`"]
pub type STAT_HCNT = crate::Reg<stat_hcnt::STAT_HCNT_SPEC>;
#[doc = "Current count of the HSCMP counter"]
pub mod stat_hcnt;
#[doc = "RESULT_VAL1 register accessor: an alias for `Reg<RESULT_VAL1_SPEC>`"]
pub type RESULT_VAL1 = crate::Reg<result_val1::RESULT_VAL1_SPEC>;
#[doc = "Result CSD/CSX accumulation counter value 1"]
pub mod result_val1;
#[doc = "RESULT_VAL2 register accessor: an alias for `Reg<RESULT_VAL2_SPEC>`"]
pub type RESULT_VAL2 = crate::Reg<result_val2::RESULT_VAL2_SPEC>;
#[doc = "Result CSX accumulation counter value 2"]
pub mod result_val2;
#[doc = "ADC_RES register accessor: an alias for `Reg<ADC_RES_SPEC>`"]
pub type ADC_RES = crate::Reg<adc_res::ADC_RES_SPEC>;
#[doc = "ADC measurement"]
pub mod adc_res;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "CSD Interrupt Request Register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "CSD Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "CSD Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "CSD Interrupt masked register"]
pub mod intr_masked;
#[doc = "HSCMP register accessor: an alias for `Reg<HSCMP_SPEC>`"]
pub type HSCMP = crate::Reg<hscmp::HSCMP_SPEC>;
#[doc = "High Speed Comparator configuration"]
pub mod hscmp;
#[doc = "AMBUF register accessor: an alias for `Reg<AMBUF_SPEC>`"]
pub type AMBUF = crate::Reg<ambuf::AMBUF_SPEC>;
#[doc = "Reference Generator configuration"]
pub mod ambuf;
#[doc = "REFGEN register accessor: an alias for `Reg<REFGEN_SPEC>`"]
pub type REFGEN = crate::Reg<refgen::REFGEN_SPEC>;
#[doc = "Reference Generator configuration"]
pub mod refgen;
#[doc = "CSDCMP register accessor: an alias for `Reg<CSDCMP_SPEC>`"]
pub type CSDCMP = crate::Reg<csdcmp::CSDCMP_SPEC>;
#[doc = "CSD Comparator configuration"]
pub mod csdcmp;
#[doc = "SW_RES register accessor: an alias for `Reg<SW_RES_SPEC>`"]
pub type SW_RES = crate::Reg<sw_res::SW_RES_SPEC>;
#[doc = "Switch Resistance configuration"]
pub mod sw_res;
#[doc = "SENSE_PERIOD register accessor: an alias for `Reg<SENSE_PERIOD_SPEC>`"]
pub type SENSE_PERIOD = crate::Reg<sense_period::SENSE_PERIOD_SPEC>;
#[doc = "Sense clock period"]
pub mod sense_period;
#[doc = "SENSE_DUTY register accessor: an alias for `Reg<SENSE_DUTY_SPEC>`"]
pub type SENSE_DUTY = crate::Reg<sense_duty::SENSE_DUTY_SPEC>;
#[doc = "Sense clock duty cycle"]
pub mod sense_duty;
#[doc = "SW_HS_P_SEL register accessor: an alias for `Reg<SW_HS_P_SEL_SPEC>`"]
pub type SW_HS_P_SEL = crate::Reg<sw_hs_p_sel::SW_HS_P_SEL_SPEC>;
#[doc = "HSCMP Pos input switch Waveform selection"]
pub mod sw_hs_p_sel;
#[doc = "SW_HS_N_SEL register accessor: an alias for `Reg<SW_HS_N_SEL_SPEC>`"]
pub type SW_HS_N_SEL = crate::Reg<sw_hs_n_sel::SW_HS_N_SEL_SPEC>;
#[doc = "HSCMP Neg input switch Waveform selection"]
pub mod sw_hs_n_sel;
#[doc = "SW_SHIELD_SEL register accessor: an alias for `Reg<SW_SHIELD_SEL_SPEC>`"]
pub type SW_SHIELD_SEL = crate::Reg<sw_shield_sel::SW_SHIELD_SEL_SPEC>;
#[doc = "Shielding switches Waveform selection"]
pub mod sw_shield_sel;
#[doc = "SW_AMUXBUF_SEL register accessor: an alias for `Reg<SW_AMUXBUF_SEL_SPEC>`"]
pub type SW_AMUXBUF_SEL = crate::Reg<sw_amuxbuf_sel::SW_AMUXBUF_SEL_SPEC>;
#[doc = "Amuxbuffer switches Waveform selection"]
pub mod sw_amuxbuf_sel;
#[doc = "SW_BYP_SEL register accessor: an alias for `Reg<SW_BYP_SEL_SPEC>`"]
pub type SW_BYP_SEL = crate::Reg<sw_byp_sel::SW_BYP_SEL_SPEC>;
#[doc = "AMUXBUS bypass switches Waveform selection"]
pub mod sw_byp_sel;
#[doc = "SW_CMP_P_SEL register accessor: an alias for `Reg<SW_CMP_P_SEL_SPEC>`"]
pub type SW_CMP_P_SEL = crate::Reg<sw_cmp_p_sel::SW_CMP_P_SEL_SPEC>;
#[doc = "CSDCMP Pos Switch Waveform selection"]
pub mod sw_cmp_p_sel;
#[doc = "SW_CMP_N_SEL register accessor: an alias for `Reg<SW_CMP_N_SEL_SPEC>`"]
pub type SW_CMP_N_SEL = crate::Reg<sw_cmp_n_sel::SW_CMP_N_SEL_SPEC>;
#[doc = "CSDCMP Neg Switch Waveform selection"]
pub mod sw_cmp_n_sel;
#[doc = "SW_REFGEN_SEL register accessor: an alias for `Reg<SW_REFGEN_SEL_SPEC>`"]
pub type SW_REFGEN_SEL = crate::Reg<sw_refgen_sel::SW_REFGEN_SEL_SPEC>;
#[doc = "Reference Generator Switch Waveform selection"]
pub mod sw_refgen_sel;
#[doc = "SW_FW_MOD_SEL register accessor: an alias for `Reg<SW_FW_MOD_SEL_SPEC>`"]
pub type SW_FW_MOD_SEL = crate::Reg<sw_fw_mod_sel::SW_FW_MOD_SEL_SPEC>;
#[doc = "Full Wave Cmod Switch Waveform selection"]
pub mod sw_fw_mod_sel;
#[doc = "SW_FW_TANK_SEL register accessor: an alias for `Reg<SW_FW_TANK_SEL_SPEC>`"]
pub type SW_FW_TANK_SEL = crate::Reg<sw_fw_tank_sel::SW_FW_TANK_SEL_SPEC>;
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
pub mod sw_fw_tank_sel;
#[doc = "SW_DSI_SEL register accessor: an alias for `Reg<SW_DSI_SEL_SPEC>`"]
pub type SW_DSI_SEL = crate::Reg<sw_dsi_sel::SW_DSI_SEL_SPEC>;
#[doc = "DSI output switch control Waveform selection"]
pub mod sw_dsi_sel;
#[doc = "IO_SEL register accessor: an alias for `Reg<IO_SEL_SPEC>`"]
pub type IO_SEL = crate::Reg<io_sel::IO_SEL_SPEC>;
#[doc = "IO output control Waveform selection"]
pub mod io_sel;
#[doc = "SEQ_TIME register accessor: an alias for `Reg<SEQ_TIME_SPEC>`"]
pub type SEQ_TIME = crate::Reg<seq_time::SEQ_TIME_SPEC>;
#[doc = "Sequencer Timing"]
pub mod seq_time;
#[doc = "SEQ_INIT_CNT register accessor: an alias for `Reg<SEQ_INIT_CNT_SPEC>`"]
pub type SEQ_INIT_CNT = crate::Reg<seq_init_cnt::SEQ_INIT_CNT_SPEC>;
#[doc = "Sequencer Initial conversion and sample counts"]
pub mod seq_init_cnt;
#[doc = "SEQ_NORM_CNT register accessor: an alias for `Reg<SEQ_NORM_CNT_SPEC>`"]
pub type SEQ_NORM_CNT = crate::Reg<seq_norm_cnt::SEQ_NORM_CNT_SPEC>;
#[doc = "Sequencer Normal conversion and sample counts"]
pub mod seq_norm_cnt;
#[doc = "ADC_CTL register accessor: an alias for `Reg<ADC_CTL_SPEC>`"]
pub type ADC_CTL = crate::Reg<adc_ctl::ADC_CTL_SPEC>;
#[doc = "ADC Control"]
pub mod adc_ctl;
#[doc = "SEQ_START register accessor: an alias for `Reg<SEQ_START_SPEC>`"]
pub type SEQ_START = crate::Reg<seq_start::SEQ_START_SPEC>;
#[doc = "Sequencer start"]
pub mod seq_start;
#[doc = "IDACA register accessor: an alias for `Reg<IDACA_SPEC>`"]
pub type IDACA = crate::Reg<idaca::IDACA_SPEC>;
#[doc = "IDACA Configuration"]
pub mod idaca;
#[doc = "IDACB register accessor: an alias for `Reg<IDACB_SPEC>`"]
pub type IDACB = crate::Reg<idacb::IDACB_SPEC>;
#[doc = "IDACB Configuration"]
pub mod idacb;
