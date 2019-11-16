#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 60usize],
    #[doc = "0x44 - Transmitter command FIFO status"]
    pub tx_cmd_fifo_status: TX_CMD_FIFO_STATUS,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - Transmitter command FIFO write"]
    pub tx_cmd_fifo_wr: TX_CMD_FIFO_WR,
    _reserved4: [u8; 44usize],
    #[doc = "0x80 - Transmitter data FIFO control"]
    pub tx_data_fifo_ctl: TX_DATA_FIFO_CTL,
    #[doc = "0x84 - Transmitter data FIFO status"]
    pub tx_data_fifo_status: TX_DATA_FIFO_STATUS,
    _reserved6: [u8; 8usize],
    #[doc = "0x90 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr1: TX_DATA_FIFO_WR1,
    #[doc = "0x94 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr2: TX_DATA_FIFO_WR2,
    #[doc = "0x98 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr4: TX_DATA_FIFO_WR4,
    _reserved9: [u8; 36usize],
    #[doc = "0xc0 - Receiver data FIFO control"]
    pub rx_data_fifo_ctl: RX_DATA_FIFO_CTL,
    #[doc = "0xc4 - Receiver data FIFO status"]
    pub rx_data_fifo_status: RX_DATA_FIFO_STATUS,
    _reserved11: [u8; 8usize],
    #[doc = "0xd0 - Receiver data FIFO read"]
    pub rx_data_fifo_rd1: RX_DATA_FIFO_RD1,
    #[doc = "0xd4 - Receiver data FIFO read"]
    pub rx_data_fifo_rd2: RX_DATA_FIFO_RD2,
    #[doc = "0xd8 - Receiver data FIFO read"]
    pub rx_data_fifo_rd4: RX_DATA_FIFO_RD4,
    _reserved14: [u8; 4usize],
    #[doc = "0xe0 - Receiver data FIFO silent read"]
    pub rx_data_fifo_rd1_silent: RX_DATA_FIFO_RD1_SILENT,
    _reserved15: [u8; 28usize],
    #[doc = "0x100 - Slow cache control"]
    pub slow_ca_ctl: SLOW_CA_CTL,
    _reserved16: [u8; 4usize],
    #[doc = "0x108 - Slow cache command"]
    pub slow_ca_cmd: SLOW_CA_CMD,
    _reserved17: [u8; 116usize],
    #[doc = "0x180 - Fast cache control"]
    pub fast_ca_ctl: FAST_CA_CTL,
    _reserved18: [u8; 4usize],
    #[doc = "0x188 - Fast cache command"]
    pub fast_ca_cmd: FAST_CA_CMD,
    _reserved19: [u8; 116usize],
    #[doc = "0x200 - Cryptography Command"]
    pub crypto_cmd: CRYPTO_CMD,
    _reserved20: [u8; 28usize],
    #[doc = "0x220 - Cryptography input 0"]
    pub crypto_input0: CRYPTO_INPUT0,
    #[doc = "0x224 - Cryptography input 1"]
    pub crypto_input1: CRYPTO_INPUT1,
    #[doc = "0x228 - Cryptography input 2"]
    pub crypto_input2: CRYPTO_INPUT2,
    #[doc = "0x22c - Cryptography input 3"]
    pub crypto_input3: CRYPTO_INPUT3,
    _reserved24: [u8; 16usize],
    #[doc = "0x240 - Cryptography key 0"]
    pub crypto_key0: CRYPTO_KEY0,
    #[doc = "0x244 - Cryptography key 1"]
    pub crypto_key1: CRYPTO_KEY1,
    #[doc = "0x248 - Cryptography key 2"]
    pub crypto_key2: CRYPTO_KEY2,
    #[doc = "0x24c - Cryptography key 3"]
    pub crypto_key3: CRYPTO_KEY3,
    _reserved28: [u8; 16usize],
    #[doc = "0x260 - Cryptography output 0"]
    pub crypto_output0: CRYPTO_OUTPUT0,
    #[doc = "0x264 - Cryptography output 1"]
    pub crypto_output1: CRYPTO_OUTPUT1,
    #[doc = "0x268 - Cryptography output 2"]
    pub crypto_output2: CRYPTO_OUTPUT2,
    #[doc = "0x26c - Cryptography output 3"]
    pub crypto_output3: CRYPTO_OUTPUT3,
    _reserved32: [u8; 1360usize],
    #[doc = "0x7c0 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0x7c4 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0x7c8 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7cc - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
    _reserved36: [u8; 48usize],
    #[doc = "0x800 - Device (only used in XIP mode)"]
    pub device0: DEVICE,
    _reserved37: [u8; 12usize],
    #[doc = "0x880 - Device (only used in XIP mode)"]
    pub device1: DEVICE,
    _reserved38: [u8; 12usize],
    #[doc = "0x900 - Device (only used in XIP mode)"]
    pub device2: DEVICE,
    _reserved39: [u8; 12usize],
    #[doc = "0x980 - Device (only used in XIP mode)"]
    pub device3: DEVICE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control"]
    pub ctl: self::device::CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Device region base address"]
    pub addr: self::device::ADDR,
    #[doc = "0x0c - Device region mask"]
    pub mask: self::device::MASK,
    _reserved3: [u8; 16usize],
    #[doc = "0x20 - Address control"]
    pub addr_ctl: self::device::ADDR_CTL,
    _reserved4: [u8; 28usize],
    #[doc = "0x40 - Read command control"]
    pub rd_cmd_ctl: self::device::RD_CMD_CTL,
    #[doc = "0x44 - Read address control"]
    pub rd_addr_ctl: self::device::RD_ADDR_CTL,
    #[doc = "0x48 - Read mode control"]
    pub rd_mode_ctl: self::device::RD_MODE_CTL,
    #[doc = "0x4c - Read dummy control"]
    pub rd_dummy_ctl: self::device::RD_DUMMY_CTL,
    #[doc = "0x50 - Read data control"]
    pub rd_data_ctl: self::device::RD_DATA_CTL,
    _reserved9: [u8; 12usize],
    #[doc = "0x60 - Write command control"]
    pub wr_cmd_ctl: self::device::WR_CMD_CTL,
    #[doc = "0x64 - Write address control"]
    pub wr_addr_ctl: self::device::WR_ADDR_CTL,
    #[doc = "0x68 - Write mode control"]
    pub wr_mode_ctl: self::device::WR_MODE_CTL,
    #[doc = "0x6c - Write dummy control"]
    pub wr_dummy_ctl: self::device::WR_DUMMY_CTL,
    #[doc = "0x70 - Write data control"]
    pub wr_data_ctl: self::device::WR_DATA_CTL,
}
#[doc = r"Register block"]
#[doc = "Device (only used in XIP mode)"]
pub mod device;
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
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Transmitter command FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cmd_fifo_status](tx_cmd_fifo_status) module"]
pub type TX_CMD_FIFO_STATUS = crate::Reg<u32, _TX_CMD_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CMD_FIFO_STATUS;
#[doc = "`read()` method returns [tx_cmd_fifo_status::R](tx_cmd_fifo_status::R) reader structure"]
impl crate::Readable for TX_CMD_FIFO_STATUS {}
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "Transmitter command FIFO write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cmd_fifo_wr](tx_cmd_fifo_wr) module"]
pub type TX_CMD_FIFO_WR = crate::Reg<u32, _TX_CMD_FIFO_WR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CMD_FIFO_WR;
#[doc = "`write(|w| ..)` method takes [tx_cmd_fifo_wr::W](tx_cmd_fifo_wr::W) writer structure"]
impl crate::Writable for TX_CMD_FIFO_WR {}
#[doc = "Transmitter command FIFO write"]
pub mod tx_cmd_fifo_wr;
#[doc = "Transmitter data FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_ctl](tx_data_fifo_ctl) module"]
pub type TX_DATA_FIFO_CTL = crate::Reg<u32, _TX_DATA_FIFO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DATA_FIFO_CTL;
#[doc = "`read()` method returns [tx_data_fifo_ctl::R](tx_data_fifo_ctl::R) reader structure"]
impl crate::Readable for TX_DATA_FIFO_CTL {}
#[doc = "`write(|w| ..)` method takes [tx_data_fifo_ctl::W](tx_data_fifo_ctl::W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_CTL {}
#[doc = "Transmitter data FIFO control"]
pub mod tx_data_fifo_ctl;
#[doc = "Transmitter data FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_status](tx_data_fifo_status) module"]
pub type TX_DATA_FIFO_STATUS = crate::Reg<u32, _TX_DATA_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DATA_FIFO_STATUS;
#[doc = "`read()` method returns [tx_data_fifo_status::R](tx_data_fifo_status::R) reader structure"]
impl crate::Readable for TX_DATA_FIFO_STATUS {}
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "Transmitter data FIFO write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_wr1](tx_data_fifo_wr1) module"]
pub type TX_DATA_FIFO_WR1 = crate::Reg<u32, _TX_DATA_FIFO_WR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DATA_FIFO_WR1;
#[doc = "`write(|w| ..)` method takes [tx_data_fifo_wr1::W](tx_data_fifo_wr1::W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR1 {}
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1;
#[doc = "Transmitter data FIFO write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_wr2](tx_data_fifo_wr2) module"]
pub type TX_DATA_FIFO_WR2 = crate::Reg<u32, _TX_DATA_FIFO_WR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DATA_FIFO_WR2;
#[doc = "`write(|w| ..)` method takes [tx_data_fifo_wr2::W](tx_data_fifo_wr2::W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR2 {}
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr2;
#[doc = "Transmitter data FIFO write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_data_fifo_wr4](tx_data_fifo_wr4) module"]
pub type TX_DATA_FIFO_WR4 = crate::Reg<u32, _TX_DATA_FIFO_WR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_DATA_FIFO_WR4;
#[doc = "`write(|w| ..)` method takes [tx_data_fifo_wr4::W](tx_data_fifo_wr4::W) writer structure"]
impl crate::Writable for TX_DATA_FIFO_WR4 {}
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr4;
#[doc = "Receiver data FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_ctl](rx_data_fifo_ctl) module"]
pub type RX_DATA_FIFO_CTL = crate::Reg<u32, _RX_DATA_FIFO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_CTL;
#[doc = "`read()` method returns [rx_data_fifo_ctl::R](rx_data_fifo_ctl::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_CTL {}
#[doc = "`write(|w| ..)` method takes [rx_data_fifo_ctl::W](rx_data_fifo_ctl::W) writer structure"]
impl crate::Writable for RX_DATA_FIFO_CTL {}
#[doc = "Receiver data FIFO control"]
pub mod rx_data_fifo_ctl;
#[doc = "Receiver data FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_status](rx_data_fifo_status) module"]
pub type RX_DATA_FIFO_STATUS = crate::Reg<u32, _RX_DATA_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_STATUS;
#[doc = "`read()` method returns [rx_data_fifo_status::R](rx_data_fifo_status::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_STATUS {}
#[doc = "Receiver data FIFO status"]
pub mod rx_data_fifo_status;
#[doc = "Receiver data FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd1](rx_data_fifo_rd1) module"]
pub type RX_DATA_FIFO_RD1 = crate::Reg<u32, _RX_DATA_FIFO_RD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_RD1;
#[doc = "`read()` method returns [rx_data_fifo_rd1::R](rx_data_fifo_rd1::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD1 {}
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd1;
#[doc = "Receiver data FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd2](rx_data_fifo_rd2) module"]
pub type RX_DATA_FIFO_RD2 = crate::Reg<u32, _RX_DATA_FIFO_RD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_RD2;
#[doc = "`read()` method returns [rx_data_fifo_rd2::R](rx_data_fifo_rd2::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD2 {}
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd2;
#[doc = "Receiver data FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd4](rx_data_fifo_rd4) module"]
pub type RX_DATA_FIFO_RD4 = crate::Reg<u32, _RX_DATA_FIFO_RD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_RD4;
#[doc = "`read()` method returns [rx_data_fifo_rd4::R](rx_data_fifo_rd4::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD4 {}
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd4;
#[doc = "Receiver data FIFO silent read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd1_silent](rx_data_fifo_rd1_silent) module"]
pub type RX_DATA_FIFO_RD1_SILENT = crate::Reg<u32, _RX_DATA_FIFO_RD1_SILENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_DATA_FIFO_RD1_SILENT;
#[doc = "`read()` method returns [rx_data_fifo_rd1_silent::R](rx_data_fifo_rd1_silent::R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD1_SILENT {}
#[doc = "Receiver data FIFO silent read"]
pub mod rx_data_fifo_rd1_silent;
#[doc = "Slow cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_ca_ctl](slow_ca_ctl) module"]
pub type SLOW_CA_CTL = crate::Reg<u32, _SLOW_CA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOW_CA_CTL;
#[doc = "`read()` method returns [slow_ca_ctl::R](slow_ca_ctl::R) reader structure"]
impl crate::Readable for SLOW_CA_CTL {}
#[doc = "`write(|w| ..)` method takes [slow_ca_ctl::W](slow_ca_ctl::W) writer structure"]
impl crate::Writable for SLOW_CA_CTL {}
#[doc = "Slow cache control"]
pub mod slow_ca_ctl;
#[doc = "Slow cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_ca_cmd](slow_ca_cmd) module"]
pub type SLOW_CA_CMD = crate::Reg<u32, _SLOW_CA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOW_CA_CMD;
#[doc = "`read()` method returns [slow_ca_cmd::R](slow_ca_cmd::R) reader structure"]
impl crate::Readable for SLOW_CA_CMD {}
#[doc = "`write(|w| ..)` method takes [slow_ca_cmd::W](slow_ca_cmd::W) writer structure"]
impl crate::Writable for SLOW_CA_CMD {}
#[doc = "Slow cache command"]
pub mod slow_ca_cmd;
#[doc = "Fast cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fast_ca_ctl](fast_ca_ctl) module"]
pub type FAST_CA_CTL = crate::Reg<u32, _FAST_CA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAST_CA_CTL;
#[doc = "`read()` method returns [fast_ca_ctl::R](fast_ca_ctl::R) reader structure"]
impl crate::Readable for FAST_CA_CTL {}
#[doc = "`write(|w| ..)` method takes [fast_ca_ctl::W](fast_ca_ctl::W) writer structure"]
impl crate::Writable for FAST_CA_CTL {}
#[doc = "Fast cache control"]
pub mod fast_ca_ctl;
#[doc = "Fast cache command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fast_ca_cmd](fast_ca_cmd) module"]
pub type FAST_CA_CMD = crate::Reg<u32, _FAST_CA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAST_CA_CMD;
#[doc = "`read()` method returns [fast_ca_cmd::R](fast_ca_cmd::R) reader structure"]
impl crate::Readable for FAST_CA_CMD {}
#[doc = "`write(|w| ..)` method takes [fast_ca_cmd::W](fast_ca_cmd::W) writer structure"]
impl crate::Writable for FAST_CA_CMD {}
#[doc = "Fast cache command"]
pub mod fast_ca_cmd;
#[doc = "Cryptography Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_cmd](crypto_cmd) module"]
pub type CRYPTO_CMD = crate::Reg<u32, _CRYPTO_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_CMD;
#[doc = "`read()` method returns [crypto_cmd::R](crypto_cmd::R) reader structure"]
impl crate::Readable for CRYPTO_CMD {}
#[doc = "`write(|w| ..)` method takes [crypto_cmd::W](crypto_cmd::W) writer structure"]
impl crate::Writable for CRYPTO_CMD {}
#[doc = "Cryptography Command"]
pub mod crypto_cmd;
#[doc = "Cryptography input 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input0](crypto_input0) module"]
pub type CRYPTO_INPUT0 = crate::Reg<u32, _CRYPTO_INPUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_INPUT0;
#[doc = "`read()` method returns [crypto_input0::R](crypto_input0::R) reader structure"]
impl crate::Readable for CRYPTO_INPUT0 {}
#[doc = "`write(|w| ..)` method takes [crypto_input0::W](crypto_input0::W) writer structure"]
impl crate::Writable for CRYPTO_INPUT0 {}
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "Cryptography input 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input1](crypto_input1) module"]
pub type CRYPTO_INPUT1 = crate::Reg<u32, _CRYPTO_INPUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_INPUT1;
#[doc = "`read()` method returns [crypto_input1::R](crypto_input1::R) reader structure"]
impl crate::Readable for CRYPTO_INPUT1 {}
#[doc = "`write(|w| ..)` method takes [crypto_input1::W](crypto_input1::W) writer structure"]
impl crate::Writable for CRYPTO_INPUT1 {}
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "Cryptography input 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input2](crypto_input2) module"]
pub type CRYPTO_INPUT2 = crate::Reg<u32, _CRYPTO_INPUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_INPUT2;
#[doc = "`read()` method returns [crypto_input2::R](crypto_input2::R) reader structure"]
impl crate::Readable for CRYPTO_INPUT2 {}
#[doc = "`write(|w| ..)` method takes [crypto_input2::W](crypto_input2::W) writer structure"]
impl crate::Writable for CRYPTO_INPUT2 {}
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "Cryptography input 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input3](crypto_input3) module"]
pub type CRYPTO_INPUT3 = crate::Reg<u32, _CRYPTO_INPUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_INPUT3;
#[doc = "`read()` method returns [crypto_input3::R](crypto_input3::R) reader structure"]
impl crate::Readable for CRYPTO_INPUT3 {}
#[doc = "`write(|w| ..)` method takes [crypto_input3::W](crypto_input3::W) writer structure"]
impl crate::Writable for CRYPTO_INPUT3 {}
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "Cryptography key 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_key0](crypto_key0) module"]
pub type CRYPTO_KEY0 = crate::Reg<u32, _CRYPTO_KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_KEY0;
#[doc = "`write(|w| ..)` method takes [crypto_key0::W](crypto_key0::W) writer structure"]
impl crate::Writable for CRYPTO_KEY0 {}
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "Cryptography key 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_key1](crypto_key1) module"]
pub type CRYPTO_KEY1 = crate::Reg<u32, _CRYPTO_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_KEY1;
#[doc = "`write(|w| ..)` method takes [crypto_key1::W](crypto_key1::W) writer structure"]
impl crate::Writable for CRYPTO_KEY1 {}
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "Cryptography key 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_key2](crypto_key2) module"]
pub type CRYPTO_KEY2 = crate::Reg<u32, _CRYPTO_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_KEY2;
#[doc = "`write(|w| ..)` method takes [crypto_key2::W](crypto_key2::W) writer structure"]
impl crate::Writable for CRYPTO_KEY2 {}
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "Cryptography key 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_key3](crypto_key3) module"]
pub type CRYPTO_KEY3 = crate::Reg<u32, _CRYPTO_KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_KEY3;
#[doc = "`write(|w| ..)` method takes [crypto_key3::W](crypto_key3::W) writer structure"]
impl crate::Writable for CRYPTO_KEY3 {}
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "Cryptography output 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_output0](crypto_output0) module"]
pub type CRYPTO_OUTPUT0 = crate::Reg<u32, _CRYPTO_OUTPUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_OUTPUT0;
#[doc = "`read()` method returns [crypto_output0::R](crypto_output0::R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT0 {}
#[doc = "`write(|w| ..)` method takes [crypto_output0::W](crypto_output0::W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT0 {}
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "Cryptography output 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_output1](crypto_output1) module"]
pub type CRYPTO_OUTPUT1 = crate::Reg<u32, _CRYPTO_OUTPUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_OUTPUT1;
#[doc = "`read()` method returns [crypto_output1::R](crypto_output1::R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT1 {}
#[doc = "`write(|w| ..)` method takes [crypto_output1::W](crypto_output1::W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT1 {}
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "Cryptography output 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_output2](crypto_output2) module"]
pub type CRYPTO_OUTPUT2 = crate::Reg<u32, _CRYPTO_OUTPUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_OUTPUT2;
#[doc = "`read()` method returns [crypto_output2::R](crypto_output2::R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT2 {}
#[doc = "`write(|w| ..)` method takes [crypto_output2::W](crypto_output2::W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT2 {}
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "Cryptography output 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_output3](crypto_output3) module"]
pub type CRYPTO_OUTPUT3 = crate::Reg<u32, _CRYPTO_OUTPUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTO_OUTPUT3;
#[doc = "`read()` method returns [crypto_output3::R](crypto_output3::R) reader structure"]
impl crate::Readable for CRYPTO_OUTPUT3 {}
#[doc = "`write(|w| ..)` method takes [crypto_output3::W](crypto_output3::W) writer structure"]
impl crate::Writable for CRYPTO_OUTPUT3 {}
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
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
