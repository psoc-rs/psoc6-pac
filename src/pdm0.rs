#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Clock control"]
    pub clock_ctl: CLOCK_CTL,
    #[doc = "0x14 - Mode control"]
    pub mode_ctl: MODE_CTL,
    #[doc = "0x18 - Data control"]
    pub data_ctl: DATA_CTL,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Command"]
    pub cmd: CMD,
    _reserved5: [u8; 28usize],
    #[doc = "0x40 - Trigger control"]
    pub tr_ctl: TR_CTL,
    _reserved6: [u8; 700usize],
    #[doc = "0x300 - RX FIFO control"]
    pub rx_fifo_ctl: RX_FIFO_CTL,
    #[doc = "0x304 - RX FIFO status"]
    pub rx_fifo_status: RX_FIFO_STATUS,
    #[doc = "0x308 - RX FIFO read"]
    pub rx_fifo_rd: RX_FIFO_RD,
    #[doc = "0x30c - RX FIFO silent read"]
    pub rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved10: [u8; 3056usize],
    #[doc = "0xf00 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0xf04 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0xf08 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xf0c - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](clock_ctl) module"]
pub type CLOCK_CTL = crate::Reg<u32, _CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTL;
#[doc = "`read()` method returns [clock_ctl::R](clock_ctl::R) reader structure"]
impl crate::Readable for CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](clock_ctl::W) writer structure"]
impl crate::Writable for CLOCK_CTL {}
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_ctl](mode_ctl) module"]
pub type MODE_CTL = crate::Reg<u32, _MODE_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_CTL;
#[doc = "`read()` method returns [mode_ctl::R](mode_ctl::R) reader structure"]
impl crate::Readable for MODE_CTL {}
#[doc = "`write(|w| ..)` method takes [mode_ctl::W](mode_ctl::W) writer structure"]
impl crate::Writable for MODE_CTL {}
#[doc = "Mode control"]
pub mod mode_ctl;
#[doc = "Data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_ctl](data_ctl) module"]
pub type DATA_CTL = crate::Reg<u32, _DATA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CTL;
#[doc = "`read()` method returns [data_ctl::R](data_ctl::R) reader structure"]
impl crate::Readable for DATA_CTL {}
#[doc = "`write(|w| ..)` method takes [data_ctl::W](data_ctl::W) writer structure"]
impl crate::Writable for DATA_CTL {}
#[doc = "Data control"]
pub mod data_ctl;
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command"]
pub mod cmd;
#[doc = "Trigger control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](tr_ctl) module"]
pub type TR_CTL = crate::Reg<u32, _TR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR_CTL;
#[doc = "`read()` method returns [tr_ctl::R](tr_ctl::R) reader structure"]
impl crate::Readable for TR_CTL {}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](tr_ctl::W) writer structure"]
impl crate::Writable for TR_CTL {}
#[doc = "Trigger control"]
pub mod tr_ctl;
#[doc = "RX FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_ctl](rx_fifo_ctl) module"]
pub type RX_FIFO_CTL = crate::Reg<u32, _RX_FIFO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_CTL;
#[doc = "`read()` method returns [rx_fifo_ctl::R](rx_fifo_ctl::R) reader structure"]
impl crate::Readable for RX_FIFO_CTL {}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctl::W](rx_fifo_ctl::W) writer structure"]
impl crate::Writable for RX_FIFO_CTL {}
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](rx_fifo_status) module"]
pub type RX_FIFO_STATUS = crate::Reg<u32, _RX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_STATUS;
#[doc = "`read()` method returns [rx_fifo_status::R](rx_fifo_status::R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS {}
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd](rx_fifo_rd) module"]
pub type RX_FIFO_RD = crate::Reg<u32, _RX_FIFO_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD;
#[doc = "`read()` method returns [rx_fifo_rd::R](rx_fifo_rd::R) reader structure"]
impl crate::Readable for RX_FIFO_RD {}
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX FIFO silent read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd_silent](rx_fifo_rd_silent) module"]
pub type RX_FIFO_RD_SILENT = crate::Reg<u32, _RX_FIFO_RD_SILENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD_SILENT;
#[doc = "`read()` method returns [rx_fifo_rd_silent::R](rx_fifo_rd_silent::R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT {}
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Interrupt masked register"]
pub mod intr_masked;
