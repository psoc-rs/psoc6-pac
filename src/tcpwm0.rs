#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCPWM control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - TCPWM control clear register"]
    pub ctrl_clr: crate::Reg<ctrl_clr::CTRL_CLR_SPEC>,
    #[doc = "0x08 - TCPWM control set register"]
    pub ctrl_set: crate::Reg<ctrl_set::CTRL_SET_SPEC>,
    #[doc = "0x0c - TCPWM capture command register"]
    pub cmd_capture: crate::Reg<cmd_capture::CMD_CAPTURE_SPEC>,
    #[doc = "0x10 - TCPWM reload command register"]
    pub cmd_reload: crate::Reg<cmd_reload::CMD_RELOAD_SPEC>,
    #[doc = "0x14 - TCPWM stop command register"]
    pub cmd_stop: crate::Reg<cmd_stop::CMD_STOP_SPEC>,
    #[doc = "0x18 - TCPWM start command register"]
    pub cmd_start: crate::Reg<cmd_start::CMD_START_SPEC>,
    #[doc = "0x1c - TCPWM Counter interrupt cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved8: [u8; 0xe0],
    #[doc = "0x100..0x700 - Timer/Counter/PWM Counter Module"]
    pub cnt: [CNT; 24],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: crate::Reg<self::cnt::ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Counter status register"]
    pub status: crate::Reg<self::cnt::status::STATUS_SPEC>,
    #[doc = "0x08 - Counter count register"]
    pub counter: crate::Reg<self::cnt::counter::COUNTER_SPEC>,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: crate::Reg<self::cnt::cc::CC_SPEC>,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: crate::Reg<self::cnt::cc_buff::CC_BUFF_SPEC>,
    #[doc = "0x14 - Counter period register"]
    pub period: crate::Reg<self::cnt::period::PERIOD_SPEC>,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: crate::Reg<self::cnt::period_buff::PERIOD_BUFF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0"]
    pub tr_ctrl0: crate::Reg<self::cnt::tr_ctrl0::TR_CTRL0_SPEC>,
    #[doc = "0x24 - Counter trigger control register 1"]
    pub tr_ctrl1: crate::Reg<self::cnt::tr_ctrl1::TR_CTRL1_SPEC>,
    #[doc = "0x28 - Counter trigger control register 2"]
    pub tr_ctrl2: crate::Reg<self::cnt::tr_ctrl2::TR_CTRL2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register"]
    pub intr: crate::Reg<self::cnt::intr::INTR_SPEC>,
    #[doc = "0x34 - Interrupt set request register"]
    pub intr_set: crate::Reg<self::cnt::intr_set::INTR_SET_SPEC>,
    #[doc = "0x38 - Interrupt mask register"]
    pub intr_mask: crate::Reg<self::cnt::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x3c - Interrupt masked request register"]
    pub intr_masked: crate::Reg<self::cnt::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TCPWM control register"]
pub mod ctrl;
#[doc = "CTRL_CLR register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "TCPWM control clear register"]
pub mod ctrl_clr;
#[doc = "CTRL_SET register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "TCPWM control set register"]
pub mod ctrl_set;
#[doc = "CMD_CAPTURE register accessor: an alias for `Reg<CMD_CAPTURE_SPEC>`"]
pub type CMD_CAPTURE = crate::Reg<cmd_capture::CMD_CAPTURE_SPEC>;
#[doc = "TCPWM capture command register"]
pub mod cmd_capture;
#[doc = "CMD_RELOAD register accessor: an alias for `Reg<CMD_RELOAD_SPEC>`"]
pub type CMD_RELOAD = crate::Reg<cmd_reload::CMD_RELOAD_SPEC>;
#[doc = "TCPWM reload command register"]
pub mod cmd_reload;
#[doc = "CMD_STOP register accessor: an alias for `Reg<CMD_STOP_SPEC>`"]
pub type CMD_STOP = crate::Reg<cmd_stop::CMD_STOP_SPEC>;
#[doc = "TCPWM stop command register"]
pub mod cmd_stop;
#[doc = "CMD_START register accessor: an alias for `Reg<CMD_START_SPEC>`"]
pub type CMD_START = crate::Reg<cmd_start::CMD_START_SPEC>;
#[doc = "TCPWM start command register"]
pub mod cmd_start;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "TCPWM Counter interrupt cause register"]
pub mod intr_cause;
