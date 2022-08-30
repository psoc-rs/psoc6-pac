#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved2: [u8; 0x3c],
    #[doc = "0x44 - Transmitter command FIFO status"]
    pub tx_cmd_fifo_status: crate::Reg<tx_cmd_fifo_status::TX_CMD_FIFO_STATUS_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x50 - Transmitter command FIFO write"]
    pub tx_cmd_fifo_wr: crate::Reg<tx_cmd_fifo_wr::TX_CMD_FIFO_WR_SPEC>,
    _reserved4: [u8; 0x2c],
    #[doc = "0x80 - Transmitter data FIFO control"]
    pub tx_data_fifo_ctl: crate::Reg<tx_data_fifo_ctl::TX_DATA_FIFO_CTL_SPEC>,
    #[doc = "0x84 - Transmitter data FIFO status"]
    pub tx_data_fifo_status: crate::Reg<tx_data_fifo_status::TX_DATA_FIFO_STATUS_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x90 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr1: crate::Reg<tx_data_fifo_wr1::TX_DATA_FIFO_WR1_SPEC>,
    #[doc = "0x94 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr2: crate::Reg<tx_data_fifo_wr2::TX_DATA_FIFO_WR2_SPEC>,
    #[doc = "0x98 - Transmitter data FIFO write"]
    pub tx_data_fifo_wr4: crate::Reg<tx_data_fifo_wr4::TX_DATA_FIFO_WR4_SPEC>,
    _reserved9: [u8; 0x24],
    #[doc = "0xc0 - Receiver data FIFO control"]
    pub rx_data_fifo_ctl: crate::Reg<rx_data_fifo_ctl::RX_DATA_FIFO_CTL_SPEC>,
    #[doc = "0xc4 - Receiver data FIFO status"]
    pub rx_data_fifo_status: crate::Reg<rx_data_fifo_status::RX_DATA_FIFO_STATUS_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0xd0 - Receiver data FIFO read"]
    pub rx_data_fifo_rd1: crate::Reg<rx_data_fifo_rd1::RX_DATA_FIFO_RD1_SPEC>,
    #[doc = "0xd4 - Receiver data FIFO read"]
    pub rx_data_fifo_rd2: crate::Reg<rx_data_fifo_rd2::RX_DATA_FIFO_RD2_SPEC>,
    #[doc = "0xd8 - Receiver data FIFO read"]
    pub rx_data_fifo_rd4: crate::Reg<rx_data_fifo_rd4::RX_DATA_FIFO_RD4_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0xe0 - Receiver data FIFO silent read"]
    pub rx_data_fifo_rd1_silent: crate::Reg<rx_data_fifo_rd1_silent::RX_DATA_FIFO_RD1_SILENT_SPEC>,
    _reserved15: [u8; 0x1c],
    #[doc = "0x100 - Slow cache control"]
    pub slow_ca_ctl: crate::Reg<slow_ca_ctl::SLOW_CA_CTL_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x108 - Slow cache command"]
    pub slow_ca_cmd: crate::Reg<slow_ca_cmd::SLOW_CA_CMD_SPEC>,
    _reserved17: [u8; 0x74],
    #[doc = "0x180 - Fast cache control"]
    pub fast_ca_ctl: crate::Reg<fast_ca_ctl::FAST_CA_CTL_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x188 - Fast cache command"]
    pub fast_ca_cmd: crate::Reg<fast_ca_cmd::FAST_CA_CMD_SPEC>,
    _reserved19: [u8; 0x74],
    #[doc = "0x200 - Cryptography Command"]
    pub crypto_cmd: crate::Reg<crypto_cmd::CRYPTO_CMD_SPEC>,
    _reserved20: [u8; 0x1c],
    #[doc = "0x220 - Cryptography input 0"]
    pub crypto_input0: crate::Reg<crypto_input0::CRYPTO_INPUT0_SPEC>,
    #[doc = "0x224 - Cryptography input 1"]
    pub crypto_input1: crate::Reg<crypto_input1::CRYPTO_INPUT1_SPEC>,
    #[doc = "0x228 - Cryptography input 2"]
    pub crypto_input2: crate::Reg<crypto_input2::CRYPTO_INPUT2_SPEC>,
    #[doc = "0x22c - Cryptography input 3"]
    pub crypto_input3: crate::Reg<crypto_input3::CRYPTO_INPUT3_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x240 - Cryptography key 0"]
    pub crypto_key0: crate::Reg<crypto_key0::CRYPTO_KEY0_SPEC>,
    #[doc = "0x244 - Cryptography key 1"]
    pub crypto_key1: crate::Reg<crypto_key1::CRYPTO_KEY1_SPEC>,
    #[doc = "0x248 - Cryptography key 2"]
    pub crypto_key2: crate::Reg<crypto_key2::CRYPTO_KEY2_SPEC>,
    #[doc = "0x24c - Cryptography key 3"]
    pub crypto_key3: crate::Reg<crypto_key3::CRYPTO_KEY3_SPEC>,
    _reserved28: [u8; 0x10],
    #[doc = "0x260 - Cryptography output 0"]
    pub crypto_output0: crate::Reg<crypto_output0::CRYPTO_OUTPUT0_SPEC>,
    #[doc = "0x264 - Cryptography output 1"]
    pub crypto_output1: crate::Reg<crypto_output1::CRYPTO_OUTPUT1_SPEC>,
    #[doc = "0x268 - Cryptography output 2"]
    pub crypto_output2: crate::Reg<crypto_output2::CRYPTO_OUTPUT2_SPEC>,
    #[doc = "0x26c - Cryptography output 3"]
    pub crypto_output3: crate::Reg<crypto_output3::CRYPTO_OUTPUT3_SPEC>,
    _reserved32: [u8; 0x0550],
    #[doc = "0x7c0 - Interrupt register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x7c4 - Interrupt set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x7c8 - Interrupt mask register"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x7cc - Interrupt masked register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    _reserved36: [u8; 0x30],
    #[doc = "0x800..0x874 - Device (only used in XIP mode)"]
    pub device0: DEVICE,
    _reserved37: [u8; 0x0c],
    #[doc = "0x880..0x8f4 - Device (only used in XIP mode)"]
    pub device1: DEVICE,
    _reserved38: [u8; 0x0c],
    #[doc = "0x900..0x974 - Device (only used in XIP mode)"]
    pub device2: DEVICE,
    _reserved39: [u8; 0x0c],
    #[doc = "0x980..0x9f4 - Device (only used in XIP mode)"]
    pub device3: DEVICE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<self::device::ctl::CTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Device region base address"]
    pub addr: crate::Reg<self::device::addr::ADDR_SPEC>,
    #[doc = "0x0c - Device region mask"]
    pub mask: crate::Reg<self::device::mask::MASK_SPEC>,
    _reserved3: [u8; 0x10],
    #[doc = "0x20 - Address control"]
    pub addr_ctl: crate::Reg<self::device::addr_ctl::ADDR_CTL_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x40 - Read command control"]
    pub rd_cmd_ctl: crate::Reg<self::device::rd_cmd_ctl::RD_CMD_CTL_SPEC>,
    #[doc = "0x44 - Read address control"]
    pub rd_addr_ctl: crate::Reg<self::device::rd_addr_ctl::RD_ADDR_CTL_SPEC>,
    #[doc = "0x48 - Read mode control"]
    pub rd_mode_ctl: crate::Reg<self::device::rd_mode_ctl::RD_MODE_CTL_SPEC>,
    #[doc = "0x4c - Read dummy control"]
    pub rd_dummy_ctl: crate::Reg<self::device::rd_dummy_ctl::RD_DUMMY_CTL_SPEC>,
    #[doc = "0x50 - Read data control"]
    pub rd_data_ctl: crate::Reg<self::device::rd_data_ctl::RD_DATA_CTL_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x60 - Write command control"]
    pub wr_cmd_ctl: crate::Reg<self::device::wr_cmd_ctl::WR_CMD_CTL_SPEC>,
    #[doc = "0x64 - Write address control"]
    pub wr_addr_ctl: crate::Reg<self::device::wr_addr_ctl::WR_ADDR_CTL_SPEC>,
    #[doc = "0x68 - Write mode control"]
    pub wr_mode_ctl: crate::Reg<self::device::wr_mode_ctl::WR_MODE_CTL_SPEC>,
    #[doc = "0x6c - Write dummy control"]
    pub wr_dummy_ctl: crate::Reg<self::device::wr_dummy_ctl::WR_DUMMY_CTL_SPEC>,
    #[doc = "0x70 - Write data control"]
    pub wr_data_ctl: crate::Reg<self::device::wr_data_ctl::WR_DATA_CTL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device (only used in XIP mode)"]
pub mod device;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TX_CMD_FIFO_STATUS register accessor: an alias for `Reg<TX_CMD_FIFO_STATUS_SPEC>`"]
pub type TX_CMD_FIFO_STATUS = crate::Reg<tx_cmd_fifo_status::TX_CMD_FIFO_STATUS_SPEC>;
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "TX_CMD_FIFO_WR register accessor: an alias for `Reg<TX_CMD_FIFO_WR_SPEC>`"]
pub type TX_CMD_FIFO_WR = crate::Reg<tx_cmd_fifo_wr::TX_CMD_FIFO_WR_SPEC>;
#[doc = "Transmitter command FIFO write"]
pub mod tx_cmd_fifo_wr;
#[doc = "TX_DATA_FIFO_CTL register accessor: an alias for `Reg<TX_DATA_FIFO_CTL_SPEC>`"]
pub type TX_DATA_FIFO_CTL = crate::Reg<tx_data_fifo_ctl::TX_DATA_FIFO_CTL_SPEC>;
#[doc = "Transmitter data FIFO control"]
pub mod tx_data_fifo_ctl;
#[doc = "TX_DATA_FIFO_STATUS register accessor: an alias for `Reg<TX_DATA_FIFO_STATUS_SPEC>`"]
pub type TX_DATA_FIFO_STATUS = crate::Reg<tx_data_fifo_status::TX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "TX_DATA_FIFO_WR1 register accessor: an alias for `Reg<TX_DATA_FIFO_WR1_SPEC>`"]
pub type TX_DATA_FIFO_WR1 = crate::Reg<tx_data_fifo_wr1::TX_DATA_FIFO_WR1_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1;
#[doc = "TX_DATA_FIFO_WR2 register accessor: an alias for `Reg<TX_DATA_FIFO_WR2_SPEC>`"]
pub type TX_DATA_FIFO_WR2 = crate::Reg<tx_data_fifo_wr2::TX_DATA_FIFO_WR2_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr2;
#[doc = "TX_DATA_FIFO_WR4 register accessor: an alias for `Reg<TX_DATA_FIFO_WR4_SPEC>`"]
pub type TX_DATA_FIFO_WR4 = crate::Reg<tx_data_fifo_wr4::TX_DATA_FIFO_WR4_SPEC>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr4;
#[doc = "RX_DATA_FIFO_CTL register accessor: an alias for `Reg<RX_DATA_FIFO_CTL_SPEC>`"]
pub type RX_DATA_FIFO_CTL = crate::Reg<rx_data_fifo_ctl::RX_DATA_FIFO_CTL_SPEC>;
#[doc = "Receiver data FIFO control"]
pub mod rx_data_fifo_ctl;
#[doc = "RX_DATA_FIFO_STATUS register accessor: an alias for `Reg<RX_DATA_FIFO_STATUS_SPEC>`"]
pub type RX_DATA_FIFO_STATUS = crate::Reg<rx_data_fifo_status::RX_DATA_FIFO_STATUS_SPEC>;
#[doc = "Receiver data FIFO status"]
pub mod rx_data_fifo_status;
#[doc = "RX_DATA_FIFO_RD1 register accessor: an alias for `Reg<RX_DATA_FIFO_RD1_SPEC>`"]
pub type RX_DATA_FIFO_RD1 = crate::Reg<rx_data_fifo_rd1::RX_DATA_FIFO_RD1_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd1;
#[doc = "RX_DATA_FIFO_RD2 register accessor: an alias for `Reg<RX_DATA_FIFO_RD2_SPEC>`"]
pub type RX_DATA_FIFO_RD2 = crate::Reg<rx_data_fifo_rd2::RX_DATA_FIFO_RD2_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd2;
#[doc = "RX_DATA_FIFO_RD4 register accessor: an alias for `Reg<RX_DATA_FIFO_RD4_SPEC>`"]
pub type RX_DATA_FIFO_RD4 = crate::Reg<rx_data_fifo_rd4::RX_DATA_FIFO_RD4_SPEC>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd4;
#[doc = "RX_DATA_FIFO_RD1_SILENT register accessor: an alias for `Reg<RX_DATA_FIFO_RD1_SILENT_SPEC>`"]
pub type RX_DATA_FIFO_RD1_SILENT =
    crate::Reg<rx_data_fifo_rd1_silent::RX_DATA_FIFO_RD1_SILENT_SPEC>;
#[doc = "Receiver data FIFO silent read"]
pub mod rx_data_fifo_rd1_silent;
#[doc = "SLOW_CA_CTL register accessor: an alias for `Reg<SLOW_CA_CTL_SPEC>`"]
pub type SLOW_CA_CTL = crate::Reg<slow_ca_ctl::SLOW_CA_CTL_SPEC>;
#[doc = "Slow cache control"]
pub mod slow_ca_ctl;
#[doc = "SLOW_CA_CMD register accessor: an alias for `Reg<SLOW_CA_CMD_SPEC>`"]
pub type SLOW_CA_CMD = crate::Reg<slow_ca_cmd::SLOW_CA_CMD_SPEC>;
#[doc = "Slow cache command"]
pub mod slow_ca_cmd;
#[doc = "FAST_CA_CTL register accessor: an alias for `Reg<FAST_CA_CTL_SPEC>`"]
pub type FAST_CA_CTL = crate::Reg<fast_ca_ctl::FAST_CA_CTL_SPEC>;
#[doc = "Fast cache control"]
pub mod fast_ca_ctl;
#[doc = "FAST_CA_CMD register accessor: an alias for `Reg<FAST_CA_CMD_SPEC>`"]
pub type FAST_CA_CMD = crate::Reg<fast_ca_cmd::FAST_CA_CMD_SPEC>;
#[doc = "Fast cache command"]
pub mod fast_ca_cmd;
#[doc = "CRYPTO_CMD register accessor: an alias for `Reg<CRYPTO_CMD_SPEC>`"]
pub type CRYPTO_CMD = crate::Reg<crypto_cmd::CRYPTO_CMD_SPEC>;
#[doc = "Cryptography Command"]
pub mod crypto_cmd;
#[doc = "CRYPTO_INPUT0 register accessor: an alias for `Reg<CRYPTO_INPUT0_SPEC>`"]
pub type CRYPTO_INPUT0 = crate::Reg<crypto_input0::CRYPTO_INPUT0_SPEC>;
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "CRYPTO_INPUT1 register accessor: an alias for `Reg<CRYPTO_INPUT1_SPEC>`"]
pub type CRYPTO_INPUT1 = crate::Reg<crypto_input1::CRYPTO_INPUT1_SPEC>;
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "CRYPTO_INPUT2 register accessor: an alias for `Reg<CRYPTO_INPUT2_SPEC>`"]
pub type CRYPTO_INPUT2 = crate::Reg<crypto_input2::CRYPTO_INPUT2_SPEC>;
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "CRYPTO_INPUT3 register accessor: an alias for `Reg<CRYPTO_INPUT3_SPEC>`"]
pub type CRYPTO_INPUT3 = crate::Reg<crypto_input3::CRYPTO_INPUT3_SPEC>;
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "CRYPTO_KEY0 register accessor: an alias for `Reg<CRYPTO_KEY0_SPEC>`"]
pub type CRYPTO_KEY0 = crate::Reg<crypto_key0::CRYPTO_KEY0_SPEC>;
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "CRYPTO_KEY1 register accessor: an alias for `Reg<CRYPTO_KEY1_SPEC>`"]
pub type CRYPTO_KEY1 = crate::Reg<crypto_key1::CRYPTO_KEY1_SPEC>;
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "CRYPTO_KEY2 register accessor: an alias for `Reg<CRYPTO_KEY2_SPEC>`"]
pub type CRYPTO_KEY2 = crate::Reg<crypto_key2::CRYPTO_KEY2_SPEC>;
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "CRYPTO_KEY3 register accessor: an alias for `Reg<CRYPTO_KEY3_SPEC>`"]
pub type CRYPTO_KEY3 = crate::Reg<crypto_key3::CRYPTO_KEY3_SPEC>;
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "CRYPTO_OUTPUT0 register accessor: an alias for `Reg<CRYPTO_OUTPUT0_SPEC>`"]
pub type CRYPTO_OUTPUT0 = crate::Reg<crypto_output0::CRYPTO_OUTPUT0_SPEC>;
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "CRYPTO_OUTPUT1 register accessor: an alias for `Reg<CRYPTO_OUTPUT1_SPEC>`"]
pub type CRYPTO_OUTPUT1 = crate::Reg<crypto_output1::CRYPTO_OUTPUT1_SPEC>;
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "CRYPTO_OUTPUT2 register accessor: an alias for `Reg<CRYPTO_OUTPUT2_SPEC>`"]
pub type CRYPTO_OUTPUT2 = crate::Reg<crypto_output2::CRYPTO_OUTPUT2_SPEC>;
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "CRYPTO_OUTPUT3 register accessor: an alias for `Reg<CRYPTO_OUTPUT3_SPEC>`"]
pub type CRYPTO_OUTPUT3 = crate::Reg<crypto_output3::CRYPTO_OUTPUT3_SPEC>;
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
