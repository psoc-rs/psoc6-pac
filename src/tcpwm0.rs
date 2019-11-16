#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCPWM control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TCPWM control clear register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x08 - TCPWM control set register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x0c - TCPWM capture command register"]
    pub cmd_capture: CMD_CAPTURE,
    #[doc = "0x10 - TCPWM reload command register"]
    pub cmd_reload: CMD_RELOAD,
    #[doc = "0x14 - TCPWM stop command register"]
    pub cmd_stop: CMD_STOP,
    #[doc = "0x18 - TCPWM start command register"]
    pub cmd_start: CMD_START,
    #[doc = "0x1c - TCPWM Counter interrupt cause register"]
    pub intr_cause: INTR_CAUSE,
    _reserved8: [u8; 224usize],
    #[doc = "0x100 - Timer/Counter/PWM Counter Module"]
    pub cnt: [CNT; 24],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: self::cnt::CTRL,
    #[doc = "0x04 - Counter status register"]
    pub status: self::cnt::STATUS,
    #[doc = "0x08 - Counter count register"]
    pub counter: self::cnt::COUNTER,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: self::cnt::CC,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: self::cnt::CC_BUFF,
    #[doc = "0x14 - Counter period register"]
    pub period: self::cnt::PERIOD,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: self::cnt::PERIOD_BUFF,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Counter trigger control register 0"]
    pub tr_ctrl0: self::cnt::TR_CTRL0,
    #[doc = "0x24 - Counter trigger control register 1"]
    pub tr_ctrl1: self::cnt::TR_CTRL1,
    #[doc = "0x28 - Counter trigger control register 2"]
    pub tr_ctrl2: self::cnt::TR_CTRL2,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Interrupt request register"]
    pub intr: self::cnt::INTR,
    #[doc = "0x34 - Interrupt set request register"]
    pub intr_set: self::cnt::INTR_SET,
    #[doc = "0x38 - Interrupt mask register"]
    pub intr_mask: self::cnt::INTR_MASK,
    #[doc = "0x3c - Interrupt masked request register"]
    pub intr_masked: self::cnt::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
#[doc = "TCPWM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "TCPWM control register"]
pub mod ctrl;
#[doc = "TCPWM control clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_clr](ctrl_clr) module"]
pub type CTRL_CLR = crate::Reg<u32, _CTRL_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_CLR;
#[doc = "`read()` method returns [ctrl_clr::R](ctrl_clr::R) reader structure"]
impl crate::Readable for CTRL_CLR {}
#[doc = "`write(|w| ..)` method takes [ctrl_clr::W](ctrl_clr::W) writer structure"]
impl crate::Writable for CTRL_CLR {}
#[doc = "TCPWM control clear register"]
pub mod ctrl_clr;
#[doc = "TCPWM control set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](ctrl_set) module"]
pub type CTRL_SET = crate::Reg<u32, _CTRL_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_SET;
#[doc = "`read()` method returns [ctrl_set::R](ctrl_set::R) reader structure"]
impl crate::Readable for CTRL_SET {}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](ctrl_set::W) writer structure"]
impl crate::Writable for CTRL_SET {}
#[doc = "TCPWM control set register"]
pub mod ctrl_set;
#[doc = "TCPWM capture command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_capture](cmd_capture) module"]
pub type CMD_CAPTURE = crate::Reg<u32, _CMD_CAPTURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_CAPTURE;
#[doc = "`read()` method returns [cmd_capture::R](cmd_capture::R) reader structure"]
impl crate::Readable for CMD_CAPTURE {}
#[doc = "`write(|w| ..)` method takes [cmd_capture::W](cmd_capture::W) writer structure"]
impl crate::Writable for CMD_CAPTURE {}
#[doc = "TCPWM capture command register"]
pub mod cmd_capture;
#[doc = "TCPWM reload command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_reload](cmd_reload) module"]
pub type CMD_RELOAD = crate::Reg<u32, _CMD_RELOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RELOAD;
#[doc = "`read()` method returns [cmd_reload::R](cmd_reload::R) reader structure"]
impl crate::Readable for CMD_RELOAD {}
#[doc = "`write(|w| ..)` method takes [cmd_reload::W](cmd_reload::W) writer structure"]
impl crate::Writable for CMD_RELOAD {}
#[doc = "TCPWM reload command register"]
pub mod cmd_reload;
#[doc = "TCPWM stop command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_stop](cmd_stop) module"]
pub type CMD_STOP = crate::Reg<u32, _CMD_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_STOP;
#[doc = "`read()` method returns [cmd_stop::R](cmd_stop::R) reader structure"]
impl crate::Readable for CMD_STOP {}
#[doc = "`write(|w| ..)` method takes [cmd_stop::W](cmd_stop::W) writer structure"]
impl crate::Writable for CMD_STOP {}
#[doc = "TCPWM stop command register"]
pub mod cmd_stop;
#[doc = "TCPWM start command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_start](cmd_start) module"]
pub type CMD_START = crate::Reg<u32, _CMD_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_START;
#[doc = "`read()` method returns [cmd_start::R](cmd_start::R) reader structure"]
impl crate::Readable for CMD_START {}
#[doc = "`write(|w| ..)` method takes [cmd_start::W](cmd_start::W) writer structure"]
impl crate::Writable for CMD_START {}
#[doc = "TCPWM start command register"]
pub mod cmd_start;
#[doc = "TCPWM Counter interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](intr_cause) module"]
pub type INTR_CAUSE = crate::Reg<u32, _INTR_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE;
#[doc = "`read()` method returns [intr_cause::R](intr_cause::R) reader structure"]
impl crate::Readable for INTR_CAUSE {}
#[doc = "TCPWM Counter interrupt cause register"]
pub mod intr_cause;
