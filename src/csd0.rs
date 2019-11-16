#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and Control"]
    pub config: CONFIG,
    #[doc = "0x04 - Spare MMIO"]
    pub spare: SPARE,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Status Register"]
    pub status: STATUS,
    #[doc = "0x84 - Current Sequencer status"]
    pub stat_seq: STAT_SEQ,
    #[doc = "0x88 - Current status counts"]
    pub stat_cnts: STAT_CNTS,
    #[doc = "0x8c - Current count of the HSCMP counter"]
    pub stat_hcnt: STAT_HCNT,
    _reserved6: [u8; 64usize],
    #[doc = "0xd0 - Result CSD/CSX accumulation counter value 1"]
    pub result_val1: RESULT_VAL1,
    #[doc = "0xd4 - Result CSX accumulation counter value 2"]
    pub result_val2: RESULT_VAL2,
    _reserved8: [u8; 8usize],
    #[doc = "0xe0 - ADC measurement"]
    pub adc_res: ADC_RES,
    _reserved9: [u8; 12usize],
    #[doc = "0xf0 - CSD Interrupt Request Register"]
    pub intr: INTR,
    #[doc = "0xf4 - CSD Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0xf8 - CSD Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xfc - CSD Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
    _reserved13: [u8; 128usize],
    #[doc = "0x180 - High Speed Comparator configuration"]
    pub hscmp: HSCMP,
    #[doc = "0x184 - Reference Generator configuration"]
    pub ambuf: AMBUF,
    #[doc = "0x188 - Reference Generator configuration"]
    pub refgen: REFGEN,
    #[doc = "0x18c - CSD Comparator configuration"]
    pub csdcmp: CSDCMP,
    _reserved17: [u8; 96usize],
    #[doc = "0x1f0 - Switch Resistance configuration"]
    pub sw_res: SW_RES,
    _reserved18: [u8; 12usize],
    #[doc = "0x200 - Sense clock period"]
    pub sense_period: SENSE_PERIOD,
    #[doc = "0x204 - Sense clock duty cycle"]
    pub sense_duty: SENSE_DUTY,
    _reserved20: [u8; 120usize],
    #[doc = "0x280 - HSCMP Pos input switch Waveform selection"]
    pub sw_hs_p_sel: SW_HS_P_SEL,
    #[doc = "0x284 - HSCMP Neg input switch Waveform selection"]
    pub sw_hs_n_sel: SW_HS_N_SEL,
    #[doc = "0x288 - Shielding switches Waveform selection"]
    pub sw_shield_sel: SW_SHIELD_SEL,
    _reserved23: [u8; 4usize],
    #[doc = "0x290 - Amuxbuffer switches Waveform selection"]
    pub sw_amuxbuf_sel: SW_AMUXBUF_SEL,
    #[doc = "0x294 - AMUXBUS bypass switches Waveform selection"]
    pub sw_byp_sel: SW_BYP_SEL,
    _reserved25: [u8; 8usize],
    #[doc = "0x2a0 - CSDCMP Pos Switch Waveform selection"]
    pub sw_cmp_p_sel: SW_CMP_P_SEL,
    #[doc = "0x2a4 - CSDCMP Neg Switch Waveform selection"]
    pub sw_cmp_n_sel: SW_CMP_N_SEL,
    #[doc = "0x2a8 - Reference Generator Switch Waveform selection"]
    pub sw_refgen_sel: SW_REFGEN_SEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x2b0 - Full Wave Cmod Switch Waveform selection"]
    pub sw_fw_mod_sel: SW_FW_MOD_SEL,
    #[doc = "0x2b4 - Full Wave Csh_tank Switch Waveform selection"]
    pub sw_fw_tank_sel: SW_FW_TANK_SEL,
    _reserved30: [u8; 8usize],
    #[doc = "0x2c0 - DSI output switch control Waveform selection"]
    pub sw_dsi_sel: SW_DSI_SEL,
    _reserved31: [u8; 12usize],
    #[doc = "0x2d0 - IO output control Waveform selection"]
    pub io_sel: IO_SEL,
    _reserved32: [u8; 44usize],
    #[doc = "0x300 - Sequencer Timing"]
    pub seq_time: SEQ_TIME,
    _reserved33: [u8; 12usize],
    #[doc = "0x310 - Sequencer Initial conversion and sample counts"]
    pub seq_init_cnt: SEQ_INIT_CNT,
    #[doc = "0x314 - Sequencer Normal conversion and sample counts"]
    pub seq_norm_cnt: SEQ_NORM_CNT,
    _reserved35: [u8; 8usize],
    #[doc = "0x320 - ADC Control"]
    pub adc_ctl: ADC_CTL,
    _reserved36: [u8; 28usize],
    #[doc = "0x340 - Sequencer start"]
    pub seq_start: SEQ_START,
    _reserved37: [u8; 188usize],
    #[doc = "0x400 - IDACA Configuration"]
    pub idaca: IDACA,
    _reserved38: [u8; 252usize],
    #[doc = "0x500 - IDACB Configuration"]
    pub idacb: IDACB,
}
#[doc = "Configuration and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration and Control"]
pub mod config;
#[doc = "Spare MMIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare](spare) module"]
pub type SPARE = crate::Reg<u32, _SPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPARE;
#[doc = "`read()` method returns [spare::R](spare::R) reader structure"]
impl crate::Readable for SPARE {}
#[doc = "`write(|w| ..)` method takes [spare::W](spare::W) writer structure"]
impl crate::Writable for SPARE {}
#[doc = "Spare MMIO"]
pub mod spare;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Current Sequencer status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_seq](stat_seq) module"]
pub type STAT_SEQ = crate::Reg<u32, _STAT_SEQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_SEQ;
#[doc = "`read()` method returns [stat_seq::R](stat_seq::R) reader structure"]
impl crate::Readable for STAT_SEQ {}
#[doc = "Current Sequencer status"]
pub mod stat_seq;
#[doc = "Current status counts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_cnts](stat_cnts) module"]
pub type STAT_CNTS = crate::Reg<u32, _STAT_CNTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_CNTS;
#[doc = "`read()` method returns [stat_cnts::R](stat_cnts::R) reader structure"]
impl crate::Readable for STAT_CNTS {}
#[doc = "Current status counts"]
pub mod stat_cnts;
#[doc = "Current count of the HSCMP counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_hcnt](stat_hcnt) module"]
pub type STAT_HCNT = crate::Reg<u32, _STAT_HCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT_HCNT;
#[doc = "`read()` method returns [stat_hcnt::R](stat_hcnt::R) reader structure"]
impl crate::Readable for STAT_HCNT {}
#[doc = "Current count of the HSCMP counter"]
pub mod stat_hcnt;
#[doc = "Result CSD/CSX accumulation counter value 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_val1](result_val1) module"]
pub type RESULT_VAL1 = crate::Reg<u32, _RESULT_VAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT_VAL1;
#[doc = "`read()` method returns [result_val1::R](result_val1::R) reader structure"]
impl crate::Readable for RESULT_VAL1 {}
#[doc = "Result CSD/CSX accumulation counter value 1"]
pub mod result_val1;
#[doc = "Result CSX accumulation counter value 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_val2](result_val2) module"]
pub type RESULT_VAL2 = crate::Reg<u32, _RESULT_VAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT_VAL2;
#[doc = "`read()` method returns [result_val2::R](result_val2::R) reader structure"]
impl crate::Readable for RESULT_VAL2 {}
#[doc = "Result CSX accumulation counter value 2"]
pub mod result_val2;
#[doc = "ADC measurement\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_res](adc_res) module"]
pub type ADC_RES = crate::Reg<u32, _ADC_RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_RES;
#[doc = "`read()` method returns [adc_res::R](adc_res::R) reader structure"]
impl crate::Readable for ADC_RES {}
#[doc = "ADC measurement"]
pub mod adc_res;
#[doc = "CSD Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "CSD Interrupt Request Register"]
pub mod intr;
#[doc = "CSD Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "CSD Interrupt set register"]
pub mod intr_set;
#[doc = "CSD Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "CSD Interrupt mask register"]
pub mod intr_mask;
#[doc = "CSD Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "CSD Interrupt masked register"]
pub mod intr_masked;
#[doc = "High Speed Comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscmp](hscmp) module"]
pub type HSCMP = crate::Reg<u32, _HSCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSCMP;
#[doc = "`read()` method returns [hscmp::R](hscmp::R) reader structure"]
impl crate::Readable for HSCMP {}
#[doc = "`write(|w| ..)` method takes [hscmp::W](hscmp::W) writer structure"]
impl crate::Writable for HSCMP {}
#[doc = "High Speed Comparator configuration"]
pub mod hscmp;
#[doc = "Reference Generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ambuf](ambuf) module"]
pub type AMBUF = crate::Reg<u32, _AMBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMBUF;
#[doc = "`read()` method returns [ambuf::R](ambuf::R) reader structure"]
impl crate::Readable for AMBUF {}
#[doc = "`write(|w| ..)` method takes [ambuf::W](ambuf::W) writer structure"]
impl crate::Writable for AMBUF {}
#[doc = "Reference Generator configuration"]
pub mod ambuf;
#[doc = "Reference Generator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refgen](refgen) module"]
pub type REFGEN = crate::Reg<u32, _REFGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFGEN;
#[doc = "`read()` method returns [refgen::R](refgen::R) reader structure"]
impl crate::Readable for REFGEN {}
#[doc = "`write(|w| ..)` method takes [refgen::W](refgen::W) writer structure"]
impl crate::Writable for REFGEN {}
#[doc = "Reference Generator configuration"]
pub mod refgen;
#[doc = "CSD Comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdcmp](csdcmp) module"]
pub type CSDCMP = crate::Reg<u32, _CSDCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSDCMP;
#[doc = "`read()` method returns [csdcmp::R](csdcmp::R) reader structure"]
impl crate::Readable for CSDCMP {}
#[doc = "`write(|w| ..)` method takes [csdcmp::W](csdcmp::W) writer structure"]
impl crate::Writable for CSDCMP {}
#[doc = "CSD Comparator configuration"]
pub mod csdcmp;
#[doc = "Switch Resistance configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_res](sw_res) module"]
pub type SW_RES = crate::Reg<u32, _SW_RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_RES;
#[doc = "`read()` method returns [sw_res::R](sw_res::R) reader structure"]
impl crate::Readable for SW_RES {}
#[doc = "`write(|w| ..)` method takes [sw_res::W](sw_res::W) writer structure"]
impl crate::Writable for SW_RES {}
#[doc = "Switch Resistance configuration"]
pub mod sw_res;
#[doc = "Sense clock period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sense_period](sense_period) module"]
pub type SENSE_PERIOD = crate::Reg<u32, _SENSE_PERIOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSE_PERIOD;
#[doc = "`read()` method returns [sense_period::R](sense_period::R) reader structure"]
impl crate::Readable for SENSE_PERIOD {}
#[doc = "`write(|w| ..)` method takes [sense_period::W](sense_period::W) writer structure"]
impl crate::Writable for SENSE_PERIOD {}
#[doc = "Sense clock period"]
pub mod sense_period;
#[doc = "Sense clock duty cycle\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sense_duty](sense_duty) module"]
pub type SENSE_DUTY = crate::Reg<u32, _SENSE_DUTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSE_DUTY;
#[doc = "`read()` method returns [sense_duty::R](sense_duty::R) reader structure"]
impl crate::Readable for SENSE_DUTY {}
#[doc = "`write(|w| ..)` method takes [sense_duty::W](sense_duty::W) writer structure"]
impl crate::Writable for SENSE_DUTY {}
#[doc = "Sense clock duty cycle"]
pub mod sense_duty;
#[doc = "HSCMP Pos input switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_hs_p_sel](sw_hs_p_sel) module"]
pub type SW_HS_P_SEL = crate::Reg<u32, _SW_HS_P_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_HS_P_SEL;
#[doc = "`read()` method returns [sw_hs_p_sel::R](sw_hs_p_sel::R) reader structure"]
impl crate::Readable for SW_HS_P_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_hs_p_sel::W](sw_hs_p_sel::W) writer structure"]
impl crate::Writable for SW_HS_P_SEL {}
#[doc = "HSCMP Pos input switch Waveform selection"]
pub mod sw_hs_p_sel;
#[doc = "HSCMP Neg input switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_hs_n_sel](sw_hs_n_sel) module"]
pub type SW_HS_N_SEL = crate::Reg<u32, _SW_HS_N_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_HS_N_SEL;
#[doc = "`read()` method returns [sw_hs_n_sel::R](sw_hs_n_sel::R) reader structure"]
impl crate::Readable for SW_HS_N_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_hs_n_sel::W](sw_hs_n_sel::W) writer structure"]
impl crate::Writable for SW_HS_N_SEL {}
#[doc = "HSCMP Neg input switch Waveform selection"]
pub mod sw_hs_n_sel;
#[doc = "Shielding switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_shield_sel](sw_shield_sel) module"]
pub type SW_SHIELD_SEL = crate::Reg<u32, _SW_SHIELD_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_SHIELD_SEL;
#[doc = "`read()` method returns [sw_shield_sel::R](sw_shield_sel::R) reader structure"]
impl crate::Readable for SW_SHIELD_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_shield_sel::W](sw_shield_sel::W) writer structure"]
impl crate::Writable for SW_SHIELD_SEL {}
#[doc = "Shielding switches Waveform selection"]
pub mod sw_shield_sel;
#[doc = "Amuxbuffer switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_amuxbuf_sel](sw_amuxbuf_sel) module"]
pub type SW_AMUXBUF_SEL = crate::Reg<u32, _SW_AMUXBUF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_AMUXBUF_SEL;
#[doc = "`read()` method returns [sw_amuxbuf_sel::R](sw_amuxbuf_sel::R) reader structure"]
impl crate::Readable for SW_AMUXBUF_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_amuxbuf_sel::W](sw_amuxbuf_sel::W) writer structure"]
impl crate::Writable for SW_AMUXBUF_SEL {}
#[doc = "Amuxbuffer switches Waveform selection"]
pub mod sw_amuxbuf_sel;
#[doc = "AMUXBUS bypass switches Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_byp_sel](sw_byp_sel) module"]
pub type SW_BYP_SEL = crate::Reg<u32, _SW_BYP_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_BYP_SEL;
#[doc = "`read()` method returns [sw_byp_sel::R](sw_byp_sel::R) reader structure"]
impl crate::Readable for SW_BYP_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_byp_sel::W](sw_byp_sel::W) writer structure"]
impl crate::Writable for SW_BYP_SEL {}
#[doc = "AMUXBUS bypass switches Waveform selection"]
pub mod sw_byp_sel;
#[doc = "CSDCMP Pos Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cmp_p_sel](sw_cmp_p_sel) module"]
pub type SW_CMP_P_SEL = crate::Reg<u32, _SW_CMP_P_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CMP_P_SEL;
#[doc = "`read()` method returns [sw_cmp_p_sel::R](sw_cmp_p_sel::R) reader structure"]
impl crate::Readable for SW_CMP_P_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_cmp_p_sel::W](sw_cmp_p_sel::W) writer structure"]
impl crate::Writable for SW_CMP_P_SEL {}
#[doc = "CSDCMP Pos Switch Waveform selection"]
pub mod sw_cmp_p_sel;
#[doc = "CSDCMP Neg Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_cmp_n_sel](sw_cmp_n_sel) module"]
pub type SW_CMP_N_SEL = crate::Reg<u32, _SW_CMP_N_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CMP_N_SEL;
#[doc = "`read()` method returns [sw_cmp_n_sel::R](sw_cmp_n_sel::R) reader structure"]
impl crate::Readable for SW_CMP_N_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_cmp_n_sel::W](sw_cmp_n_sel::W) writer structure"]
impl crate::Writable for SW_CMP_N_SEL {}
#[doc = "CSDCMP Neg Switch Waveform selection"]
pub mod sw_cmp_n_sel;
#[doc = "Reference Generator Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_refgen_sel](sw_refgen_sel) module"]
pub type SW_REFGEN_SEL = crate::Reg<u32, _SW_REFGEN_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_REFGEN_SEL;
#[doc = "`read()` method returns [sw_refgen_sel::R](sw_refgen_sel::R) reader structure"]
impl crate::Readable for SW_REFGEN_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_refgen_sel::W](sw_refgen_sel::W) writer structure"]
impl crate::Writable for SW_REFGEN_SEL {}
#[doc = "Reference Generator Switch Waveform selection"]
pub mod sw_refgen_sel;
#[doc = "Full Wave Cmod Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_mod_sel](sw_fw_mod_sel) module"]
pub type SW_FW_MOD_SEL = crate::Reg<u32, _SW_FW_MOD_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_FW_MOD_SEL;
#[doc = "`read()` method returns [sw_fw_mod_sel::R](sw_fw_mod_sel::R) reader structure"]
impl crate::Readable for SW_FW_MOD_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_fw_mod_sel::W](sw_fw_mod_sel::W) writer structure"]
impl crate::Writable for SW_FW_MOD_SEL {}
#[doc = "Full Wave Cmod Switch Waveform selection"]
pub mod sw_fw_mod_sel;
#[doc = "Full Wave Csh_tank Switch Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_fw_tank_sel](sw_fw_tank_sel) module"]
pub type SW_FW_TANK_SEL = crate::Reg<u32, _SW_FW_TANK_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_FW_TANK_SEL;
#[doc = "`read()` method returns [sw_fw_tank_sel::R](sw_fw_tank_sel::R) reader structure"]
impl crate::Readable for SW_FW_TANK_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_fw_tank_sel::W](sw_fw_tank_sel::W) writer structure"]
impl crate::Writable for SW_FW_TANK_SEL {}
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
pub mod sw_fw_tank_sel;
#[doc = "DSI output switch control Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_dsi_sel](sw_dsi_sel) module"]
pub type SW_DSI_SEL = crate::Reg<u32, _SW_DSI_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_DSI_SEL;
#[doc = "`read()` method returns [sw_dsi_sel::R](sw_dsi_sel::R) reader structure"]
impl crate::Readable for SW_DSI_SEL {}
#[doc = "`write(|w| ..)` method takes [sw_dsi_sel::W](sw_dsi_sel::W) writer structure"]
impl crate::Writable for SW_DSI_SEL {}
#[doc = "DSI output switch control Waveform selection"]
pub mod sw_dsi_sel;
#[doc = "IO output control Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_sel](io_sel) module"]
pub type IO_SEL = crate::Reg<u32, _IO_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO_SEL;
#[doc = "`read()` method returns [io_sel::R](io_sel::R) reader structure"]
impl crate::Readable for IO_SEL {}
#[doc = "`write(|w| ..)` method takes [io_sel::W](io_sel::W) writer structure"]
impl crate::Writable for IO_SEL {}
#[doc = "IO output control Waveform selection"]
pub mod io_sel;
#[doc = "Sequencer Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_time](seq_time) module"]
pub type SEQ_TIME = crate::Reg<u32, _SEQ_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_TIME;
#[doc = "`read()` method returns [seq_time::R](seq_time::R) reader structure"]
impl crate::Readable for SEQ_TIME {}
#[doc = "`write(|w| ..)` method takes [seq_time::W](seq_time::W) writer structure"]
impl crate::Writable for SEQ_TIME {}
#[doc = "Sequencer Timing"]
pub mod seq_time;
#[doc = "Sequencer Initial conversion and sample counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_init_cnt](seq_init_cnt) module"]
pub type SEQ_INIT_CNT = crate::Reg<u32, _SEQ_INIT_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_INIT_CNT;
#[doc = "`read()` method returns [seq_init_cnt::R](seq_init_cnt::R) reader structure"]
impl crate::Readable for SEQ_INIT_CNT {}
#[doc = "`write(|w| ..)` method takes [seq_init_cnt::W](seq_init_cnt::W) writer structure"]
impl crate::Writable for SEQ_INIT_CNT {}
#[doc = "Sequencer Initial conversion and sample counts"]
pub mod seq_init_cnt;
#[doc = "Sequencer Normal conversion and sample counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_norm_cnt](seq_norm_cnt) module"]
pub type SEQ_NORM_CNT = crate::Reg<u32, _SEQ_NORM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_NORM_CNT;
#[doc = "`read()` method returns [seq_norm_cnt::R](seq_norm_cnt::R) reader structure"]
impl crate::Readable for SEQ_NORM_CNT {}
#[doc = "`write(|w| ..)` method takes [seq_norm_cnt::W](seq_norm_cnt::W) writer structure"]
impl crate::Writable for SEQ_NORM_CNT {}
#[doc = "Sequencer Normal conversion and sample counts"]
pub mod seq_norm_cnt;
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctl](adc_ctl) module"]
pub type ADC_CTL = crate::Reg<u32, _ADC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CTL;
#[doc = "`read()` method returns [adc_ctl::R](adc_ctl::R) reader structure"]
impl crate::Readable for ADC_CTL {}
#[doc = "`write(|w| ..)` method takes [adc_ctl::W](adc_ctl::W) writer structure"]
impl crate::Writable for ADC_CTL {}
#[doc = "ADC Control"]
pub mod adc_ctl;
#[doc = "Sequencer start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_start](seq_start) module"]
pub type SEQ_START = crate::Reg<u32, _SEQ_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_START;
#[doc = "`read()` method returns [seq_start::R](seq_start::R) reader structure"]
impl crate::Readable for SEQ_START {}
#[doc = "`write(|w| ..)` method takes [seq_start::W](seq_start::W) writer structure"]
impl crate::Writable for SEQ_START {}
#[doc = "Sequencer start"]
pub mod seq_start;
#[doc = "IDACA Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idaca](idaca) module"]
pub type IDACA = crate::Reg<u32, _IDACA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDACA;
#[doc = "`read()` method returns [idaca::R](idaca::R) reader structure"]
impl crate::Readable for IDACA {}
#[doc = "`write(|w| ..)` method takes [idaca::W](idaca::W) writer structure"]
impl crate::Writable for IDACA {}
#[doc = "IDACA Configuration"]
pub mod idaca;
#[doc = "IDACB Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idacb](idacb) module"]
pub type IDACB = crate::Reg<u32, _IDACB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDACB;
#[doc = "`read()` method returns [idacb::R](idacb::R) reader structure"]
impl crate::Readable for IDACB {}
#[doc = "`write(|w| ..)` method takes [idacb::W](idacb::W) writer structure"]
impl crate::Writable for IDACB {}
#[doc = "IDACB Configuration"]
pub mod idacb;
