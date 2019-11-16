#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog control register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Sample control register."]
    pub sample_ctrl: SAMPLE_CTRL,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Sample time specification ST0 and ST1"]
    pub sample_time01: SAMPLE_TIME01,
    #[doc = "0x14 - Sample time specification ST2 and ST3"]
    pub sample_time23: SAMPLE_TIME23,
    #[doc = "0x18 - Global range detect threshold register."]
    pub range_thres: RANGE_THRES,
    #[doc = "0x1c - Global range detect mode register."]
    pub range_cond: RANGE_COND,
    #[doc = "0x20 - Enable bits for the channels"]
    pub chan_en: CHAN_EN,
    #[doc = "0x24 - Start control register (firmware trigger)."]
    pub start_ctrl: START_CTRL,
    _reserved8: [u8; 88usize],
    #[doc = "0x80 - Channel configuration register."]
    pub chan_config: [CHAN_CONFIG; 16],
    _reserved9: [u8; 64usize],
    #[doc = "0x100 - Channel working data register"]
    pub chan_work: [CHAN_WORK; 16],
    _reserved10: [u8; 64usize],
    #[doc = "0x180 - Channel result data register"]
    pub chan_result: [CHAN_RESULT; 16],
    _reserved11: [u8; 64usize],
    #[doc = "0x200 - Channel working data register 'updated' bits"]
    pub chan_work_updated: CHAN_WORK_UPDATED,
    #[doc = "0x204 - Channel result data register 'updated' bits"]
    pub chan_result_updated: CHAN_RESULT_UPDATED,
    #[doc = "0x208 - Channel working data register 'new value' bits"]
    pub chan_work_newvalue: CHAN_WORK_NEWVALUE,
    #[doc = "0x20c - Channel result data register 'new value' bits"]
    pub chan_result_newvalue: CHAN_RESULT_NEWVALUE,
    #[doc = "0x210 - Interrupt request register."]
    pub intr: INTR,
    #[doc = "0x214 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x218 - Interrupt mask register."]
    pub intr_mask: INTR_MASK,
    #[doc = "0x21c - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x220 - Saturate interrupt request register."]
    pub saturate_intr: SATURATE_INTR,
    #[doc = "0x224 - Saturate interrupt set request register"]
    pub saturate_intr_set: SATURATE_INTR_SET,
    #[doc = "0x228 - Saturate interrupt mask register."]
    pub saturate_intr_mask: SATURATE_INTR_MASK,
    #[doc = "0x22c - Saturate interrupt masked request register"]
    pub saturate_intr_masked: SATURATE_INTR_MASKED,
    #[doc = "0x230 - Range detect interrupt request register."]
    pub range_intr: RANGE_INTR,
    #[doc = "0x234 - Range detect interrupt set request register"]
    pub range_intr_set: RANGE_INTR_SET,
    #[doc = "0x238 - Range detect interrupt mask register."]
    pub range_intr_mask: RANGE_INTR_MASK,
    #[doc = "0x23c - Range interrupt masked request register"]
    pub range_intr_masked: RANGE_INTR_MASKED,
    #[doc = "0x240 - Interrupt cause register"]
    pub intr_cause: INTR_CAUSE,
    _reserved28: [u8; 60usize],
    #[doc = "0x280 - Injection channel configuration register."]
    pub inj_chan_config: INJ_CHAN_CONFIG,
    _reserved29: [u8; 12usize],
    #[doc = "0x290 - Injection channel result register"]
    pub inj_result: INJ_RESULT,
    _reserved30: [u8; 12usize],
    #[doc = "0x2a0 - Current status of internal SAR registers (mostly for debug)"]
    pub status: STATUS,
    #[doc = "0x2a4 - Current averaging status (for debug)"]
    pub avg_stat: AVG_STAT,
    _reserved32: [u8; 88usize],
    #[doc = "0x300 - SARMUX Firmware switch controls"]
    pub mux_switch0: MUX_SWITCH0,
    #[doc = "0x304 - SARMUX Firmware switch control clear"]
    pub mux_switch_clear0: MUX_SWITCH_CLEAR0,
    _reserved34: [u8; 56usize],
    #[doc = "0x340 - SARMUX switch DSI control"]
    pub mux_switch_ds_ctrl: MUX_SWITCH_DS_CTRL,
    #[doc = "0x344 - SARMUX switch Sar Sequencer control"]
    pub mux_switch_sq_ctrl: MUX_SWITCH_SQ_CTRL,
    #[doc = "0x348 - SARMUX switch status"]
    pub mux_switch_status: MUX_SWITCH_STATUS,
    _reserved37: [u8; 2996usize],
    #[doc = "0xf00 - Analog trim register."]
    pub ana_trim0: ANA_TRIM0,
    #[doc = "0xf04 - Analog trim register."]
    pub ana_trim1: ANA_TRIM1,
}
#[doc = "Analog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Analog control register."]
pub mod ctrl;
#[doc = "Sample control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_ctrl](sample_ctrl) module"]
pub type SAMPLE_CTRL = crate::Reg<u32, _SAMPLE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLE_CTRL;
#[doc = "`read()` method returns [sample_ctrl::R](sample_ctrl::R) reader structure"]
impl crate::Readable for SAMPLE_CTRL {}
#[doc = "`write(|w| ..)` method takes [sample_ctrl::W](sample_ctrl::W) writer structure"]
impl crate::Writable for SAMPLE_CTRL {}
#[doc = "Sample control register."]
pub mod sample_ctrl;
#[doc = "Sample time specification ST0 and ST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_time01](sample_time01) module"]
pub type SAMPLE_TIME01 = crate::Reg<u32, _SAMPLE_TIME01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLE_TIME01;
#[doc = "`read()` method returns [sample_time01::R](sample_time01::R) reader structure"]
impl crate::Readable for SAMPLE_TIME01 {}
#[doc = "`write(|w| ..)` method takes [sample_time01::W](sample_time01::W) writer structure"]
impl crate::Writable for SAMPLE_TIME01 {}
#[doc = "Sample time specification ST0 and ST1"]
pub mod sample_time01;
#[doc = "Sample time specification ST2 and ST3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_time23](sample_time23) module"]
pub type SAMPLE_TIME23 = crate::Reg<u32, _SAMPLE_TIME23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLE_TIME23;
#[doc = "`read()` method returns [sample_time23::R](sample_time23::R) reader structure"]
impl crate::Readable for SAMPLE_TIME23 {}
#[doc = "`write(|w| ..)` method takes [sample_time23::W](sample_time23::W) writer structure"]
impl crate::Writable for SAMPLE_TIME23 {}
#[doc = "Sample time specification ST2 and ST3"]
pub mod sample_time23;
#[doc = "Global range detect threshold register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_thres](range_thres) module"]
pub type RANGE_THRES = crate::Reg<u32, _RANGE_THRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_THRES;
#[doc = "`read()` method returns [range_thres::R](range_thres::R) reader structure"]
impl crate::Readable for RANGE_THRES {}
#[doc = "`write(|w| ..)` method takes [range_thres::W](range_thres::W) writer structure"]
impl crate::Writable for RANGE_THRES {}
#[doc = "Global range detect threshold register."]
pub mod range_thres;
#[doc = "Global range detect mode register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_cond](range_cond) module"]
pub type RANGE_COND = crate::Reg<u32, _RANGE_COND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_COND;
#[doc = "`read()` method returns [range_cond::R](range_cond::R) reader structure"]
impl crate::Readable for RANGE_COND {}
#[doc = "`write(|w| ..)` method takes [range_cond::W](range_cond::W) writer structure"]
impl crate::Writable for RANGE_COND {}
#[doc = "Global range detect mode register."]
pub mod range_cond;
#[doc = "Enable bits for the channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_en](chan_en) module"]
pub type CHAN_EN = crate::Reg<u32, _CHAN_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_EN;
#[doc = "`read()` method returns [chan_en::R](chan_en::R) reader structure"]
impl crate::Readable for CHAN_EN {}
#[doc = "`write(|w| ..)` method takes [chan_en::W](chan_en::W) writer structure"]
impl crate::Writable for CHAN_EN {}
#[doc = "Enable bits for the channels"]
pub mod chan_en;
#[doc = "Start control register (firmware trigger).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start_ctrl](start_ctrl) module"]
pub type START_CTRL = crate::Reg<u32, _START_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START_CTRL;
#[doc = "`read()` method returns [start_ctrl::R](start_ctrl::R) reader structure"]
impl crate::Readable for START_CTRL {}
#[doc = "`write(|w| ..)` method takes [start_ctrl::W](start_ctrl::W) writer structure"]
impl crate::Writable for START_CTRL {}
#[doc = "Start control register (firmware trigger)."]
pub mod start_ctrl;
#[doc = "Channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_config](chan_config) module"]
pub type CHAN_CONFIG = crate::Reg<u32, _CHAN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_CONFIG;
#[doc = "`read()` method returns [chan_config::R](chan_config::R) reader structure"]
impl crate::Readable for CHAN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [chan_config::W](chan_config::W) writer structure"]
impl crate::Writable for CHAN_CONFIG {}
#[doc = "Channel configuration register."]
pub mod chan_config;
#[doc = "Channel working data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work](chan_work) module"]
pub type CHAN_WORK = crate::Reg<u32, _CHAN_WORK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_WORK;
#[doc = "`read()` method returns [chan_work::R](chan_work::R) reader structure"]
impl crate::Readable for CHAN_WORK {}
#[doc = "Channel working data register"]
pub mod chan_work;
#[doc = "Channel result data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result](chan_result) module"]
pub type CHAN_RESULT = crate::Reg<u32, _CHAN_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_RESULT;
#[doc = "`read()` method returns [chan_result::R](chan_result::R) reader structure"]
impl crate::Readable for CHAN_RESULT {}
#[doc = "Channel result data register"]
pub mod chan_result;
#[doc = "Channel working data register 'updated' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work_updated](chan_work_updated) module"]
pub type CHAN_WORK_UPDATED = crate::Reg<u32, _CHAN_WORK_UPDATED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_WORK_UPDATED;
#[doc = "`read()` method returns [chan_work_updated::R](chan_work_updated::R) reader structure"]
impl crate::Readable for CHAN_WORK_UPDATED {}
#[doc = "Channel working data register 'updated' bits"]
pub mod chan_work_updated;
#[doc = "Channel result data register 'updated' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result_updated](chan_result_updated) module"]
pub type CHAN_RESULT_UPDATED = crate::Reg<u32, _CHAN_RESULT_UPDATED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_RESULT_UPDATED;
#[doc = "`read()` method returns [chan_result_updated::R](chan_result_updated::R) reader structure"]
impl crate::Readable for CHAN_RESULT_UPDATED {}
#[doc = "Channel result data register 'updated' bits"]
pub mod chan_result_updated;
#[doc = "Channel working data register 'new value' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work_newvalue](chan_work_newvalue) module"]
pub type CHAN_WORK_NEWVALUE = crate::Reg<u32, _CHAN_WORK_NEWVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_WORK_NEWVALUE;
#[doc = "`read()` method returns [chan_work_newvalue::R](chan_work_newvalue::R) reader structure"]
impl crate::Readable for CHAN_WORK_NEWVALUE {}
#[doc = "Channel working data register 'new value' bits"]
pub mod chan_work_newvalue;
#[doc = "Channel result data register 'new value' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result_newvalue](chan_result_newvalue) module"]
pub type CHAN_RESULT_NEWVALUE = crate::Reg<u32, _CHAN_RESULT_NEWVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHAN_RESULT_NEWVALUE;
#[doc = "`read()` method returns [chan_result_newvalue::R](chan_result_newvalue::R) reader structure"]
impl crate::Readable for CHAN_RESULT_NEWVALUE {}
#[doc = "Channel result data register 'new value' bits"]
pub mod chan_result_newvalue;
#[doc = "Interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt request register."]
pub mod intr;
#[doc = "Interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "Interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask register."]
pub mod intr_mask;
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "Saturate interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr](saturate_intr) module"]
pub type SATURATE_INTR = crate::Reg<u32, _SATURATE_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SATURATE_INTR;
#[doc = "`read()` method returns [saturate_intr::R](saturate_intr::R) reader structure"]
impl crate::Readable for SATURATE_INTR {}
#[doc = "`write(|w| ..)` method takes [saturate_intr::W](saturate_intr::W) writer structure"]
impl crate::Writable for SATURATE_INTR {}
#[doc = "Saturate interrupt request register."]
pub mod saturate_intr;
#[doc = "Saturate interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_set](saturate_intr_set) module"]
pub type SATURATE_INTR_SET = crate::Reg<u32, _SATURATE_INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SATURATE_INTR_SET;
#[doc = "`read()` method returns [saturate_intr_set::R](saturate_intr_set::R) reader structure"]
impl crate::Readable for SATURATE_INTR_SET {}
#[doc = "`write(|w| ..)` method takes [saturate_intr_set::W](saturate_intr_set::W) writer structure"]
impl crate::Writable for SATURATE_INTR_SET {}
#[doc = "Saturate interrupt set request register"]
pub mod saturate_intr_set;
#[doc = "Saturate interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_mask](saturate_intr_mask) module"]
pub type SATURATE_INTR_MASK = crate::Reg<u32, _SATURATE_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SATURATE_INTR_MASK;
#[doc = "`read()` method returns [saturate_intr_mask::R](saturate_intr_mask::R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [saturate_intr_mask::W](saturate_intr_mask::W) writer structure"]
impl crate::Writable for SATURATE_INTR_MASK {}
#[doc = "Saturate interrupt mask register."]
pub mod saturate_intr_mask;
#[doc = "Saturate interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_masked](saturate_intr_masked) module"]
pub type SATURATE_INTR_MASKED = crate::Reg<u32, _SATURATE_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SATURATE_INTR_MASKED;
#[doc = "`read()` method returns [saturate_intr_masked::R](saturate_intr_masked::R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASKED {}
#[doc = "Saturate interrupt masked request register"]
pub mod saturate_intr_masked;
#[doc = "Range detect interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr](range_intr) module"]
pub type RANGE_INTR = crate::Reg<u32, _RANGE_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_INTR;
#[doc = "`read()` method returns [range_intr::R](range_intr::R) reader structure"]
impl crate::Readable for RANGE_INTR {}
#[doc = "`write(|w| ..)` method takes [range_intr::W](range_intr::W) writer structure"]
impl crate::Writable for RANGE_INTR {}
#[doc = "Range detect interrupt request register."]
pub mod range_intr;
#[doc = "Range detect interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr_set](range_intr_set) module"]
pub type RANGE_INTR_SET = crate::Reg<u32, _RANGE_INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_INTR_SET;
#[doc = "`read()` method returns [range_intr_set::R](range_intr_set::R) reader structure"]
impl crate::Readable for RANGE_INTR_SET {}
#[doc = "`write(|w| ..)` method takes [range_intr_set::W](range_intr_set::W) writer structure"]
impl crate::Writable for RANGE_INTR_SET {}
#[doc = "Range detect interrupt set request register"]
pub mod range_intr_set;
#[doc = "Range detect interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr_mask](range_intr_mask) module"]
pub type RANGE_INTR_MASK = crate::Reg<u32, _RANGE_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_INTR_MASK;
#[doc = "`read()` method returns [range_intr_mask::R](range_intr_mask::R) reader structure"]
impl crate::Readable for RANGE_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [range_intr_mask::W](range_intr_mask::W) writer structure"]
impl crate::Writable for RANGE_INTR_MASK {}
#[doc = "Range detect interrupt mask register."]
pub mod range_intr_mask;
#[doc = "Range interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr_masked](range_intr_masked) module"]
pub type RANGE_INTR_MASKED = crate::Reg<u32, _RANGE_INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANGE_INTR_MASKED;
#[doc = "`read()` method returns [range_intr_masked::R](range_intr_masked::R) reader structure"]
impl crate::Readable for RANGE_INTR_MASKED {}
#[doc = "Range interrupt masked request register"]
pub mod range_intr_masked;
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](intr_cause) module"]
pub type INTR_CAUSE = crate::Reg<u32, _INTR_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE;
#[doc = "`read()` method returns [intr_cause::R](intr_cause::R) reader structure"]
impl crate::Readable for INTR_CAUSE {}
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "Injection channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_chan_config](inj_chan_config) module"]
pub type INJ_CHAN_CONFIG = crate::Reg<u32, _INJ_CHAN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INJ_CHAN_CONFIG;
#[doc = "`read()` method returns [inj_chan_config::R](inj_chan_config::R) reader structure"]
impl crate::Readable for INJ_CHAN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [inj_chan_config::W](inj_chan_config::W) writer structure"]
impl crate::Writable for INJ_CHAN_CONFIG {}
#[doc = "Injection channel configuration register."]
pub mod inj_chan_config;
#[doc = "Injection channel result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_result](inj_result) module"]
pub type INJ_RESULT = crate::Reg<u32, _INJ_RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INJ_RESULT;
#[doc = "`read()` method returns [inj_result::R](inj_result::R) reader structure"]
impl crate::Readable for INJ_RESULT {}
#[doc = "Injection channel result register"]
pub mod inj_result;
#[doc = "Current status of internal SAR registers (mostly for debug)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "Current averaging status (for debug)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avg_stat](avg_stat) module"]
pub type AVG_STAT = crate::Reg<u32, _AVG_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVG_STAT;
#[doc = "`read()` method returns [avg_stat::R](avg_stat::R) reader structure"]
impl crate::Readable for AVG_STAT {}
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "SARMUX Firmware switch controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch0](mux_switch0) module"]
pub type MUX_SWITCH0 = crate::Reg<u32, _MUX_SWITCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX_SWITCH0;
#[doc = "`read()` method returns [mux_switch0::R](mux_switch0::R) reader structure"]
impl crate::Readable for MUX_SWITCH0 {}
#[doc = "`write(|w| ..)` method takes [mux_switch0::W](mux_switch0::W) writer structure"]
impl crate::Writable for MUX_SWITCH0 {}
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch0;
#[doc = "SARMUX Firmware switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_clear0](mux_switch_clear0) module"]
pub type MUX_SWITCH_CLEAR0 = crate::Reg<u32, _MUX_SWITCH_CLEAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX_SWITCH_CLEAR0;
#[doc = "`read()` method returns [mux_switch_clear0::R](mux_switch_clear0::R) reader structure"]
impl crate::Readable for MUX_SWITCH_CLEAR0 {}
#[doc = "`write(|w| ..)` method takes [mux_switch_clear0::W](mux_switch_clear0::W) writer structure"]
impl crate::Writable for MUX_SWITCH_CLEAR0 {}
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear0;
#[doc = "SARMUX switch DSI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_ds_ctrl](mux_switch_ds_ctrl) module"]
pub type MUX_SWITCH_DS_CTRL = crate::Reg<u32, _MUX_SWITCH_DS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX_SWITCH_DS_CTRL;
#[doc = "`read()` method returns [mux_switch_ds_ctrl::R](mux_switch_ds_ctrl::R) reader structure"]
impl crate::Readable for MUX_SWITCH_DS_CTRL {}
#[doc = "`write(|w| ..)` method takes [mux_switch_ds_ctrl::W](mux_switch_ds_ctrl::W) writer structure"]
impl crate::Writable for MUX_SWITCH_DS_CTRL {}
#[doc = "SARMUX switch DSI control"]
pub mod mux_switch_ds_ctrl;
#[doc = "SARMUX switch Sar Sequencer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_sq_ctrl](mux_switch_sq_ctrl) module"]
pub type MUX_SWITCH_SQ_CTRL = crate::Reg<u32, _MUX_SWITCH_SQ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX_SWITCH_SQ_CTRL;
#[doc = "`read()` method returns [mux_switch_sq_ctrl::R](mux_switch_sq_ctrl::R) reader structure"]
impl crate::Readable for MUX_SWITCH_SQ_CTRL {}
#[doc = "`write(|w| ..)` method takes [mux_switch_sq_ctrl::W](mux_switch_sq_ctrl::W) writer structure"]
impl crate::Writable for MUX_SWITCH_SQ_CTRL {}
#[doc = "SARMUX switch Sar Sequencer control"]
pub mod mux_switch_sq_ctrl;
#[doc = "SARMUX switch status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_status](mux_switch_status) module"]
pub type MUX_SWITCH_STATUS = crate::Reg<u32, _MUX_SWITCH_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX_SWITCH_STATUS;
#[doc = "`read()` method returns [mux_switch_status::R](mux_switch_status::R) reader structure"]
impl crate::Readable for MUX_SWITCH_STATUS {}
#[doc = "SARMUX switch status"]
pub mod mux_switch_status;
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim0](ana_trim0) module"]
pub type ANA_TRIM0 = crate::Reg<u32, _ANA_TRIM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_TRIM0;
#[doc = "`read()` method returns [ana_trim0::R](ana_trim0::R) reader structure"]
impl crate::Readable for ANA_TRIM0 {}
#[doc = "`write(|w| ..)` method takes [ana_trim0::W](ana_trim0::W) writer structure"]
impl crate::Writable for ANA_TRIM0 {}
#[doc = "Analog trim register."]
pub mod ana_trim0;
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim1](ana_trim1) module"]
pub type ANA_TRIM1 = crate::Reg<u32, _ANA_TRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA_TRIM1;
#[doc = "`read()` method returns [ana_trim1::R](ana_trim1::R) reader structure"]
impl crate::Readable for ANA_TRIM1 {}
#[doc = "`write(|w| ..)` method takes [ana_trim1::W](ana_trim1::W) writer structure"]
impl crate::Writable for ANA_TRIM1 {}
#[doc = "Analog trim register."]
pub mod ana_trim1;
