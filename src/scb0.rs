#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Generic status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Command/response control"]
    pub cmd_resp_ctrl: crate::Reg<cmd_resp_ctrl::CMD_RESP_CTRL_SPEC>,
    #[doc = "0x0c - Command/response status"]
    pub cmd_resp_status: crate::Reg<cmd_resp_status::CMD_RESP_STATUS_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - SPI control"]
    pub spi_ctrl: crate::Reg<spi_ctrl::SPI_CTRL_SPEC>,
    #[doc = "0x24 - SPI status"]
    pub spi_status: crate::Reg<spi_status::SPI_STATUS_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - UART control"]
    pub uart_ctrl: crate::Reg<uart_ctrl::UART_CTRL_SPEC>,
    #[doc = "0x44 - UART transmitter control"]
    pub uart_tx_ctrl: crate::Reg<uart_tx_ctrl::UART_TX_CTRL_SPEC>,
    #[doc = "0x48 - UART receiver control"]
    pub uart_rx_ctrl: crate::Reg<uart_rx_ctrl::UART_RX_CTRL_SPEC>,
    #[doc = "0x4c - UART receiver status"]
    pub uart_rx_status: crate::Reg<uart_rx_status::UART_RX_STATUS_SPEC>,
    #[doc = "0x50 - UART flow control"]
    pub uart_flow_ctrl: crate::Reg<uart_flow_ctrl::UART_FLOW_CTRL_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - I2C control"]
    pub i2c_ctrl: crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>,
    #[doc = "0x64 - I2C status"]
    pub i2c_status: crate::Reg<i2c_status::I2C_STATUS_SPEC>,
    #[doc = "0x68 - I2C master command"]
    pub i2c_m_cmd: crate::Reg<i2c_m_cmd::I2C_M_CMD_SPEC>,
    #[doc = "0x6c - I2C slave command"]
    pub i2c_s_cmd: crate::Reg<i2c_s_cmd::I2C_S_CMD_SPEC>,
    #[doc = "0x70 - I2C configuration"]
    pub i2c_cfg: crate::Reg<i2c_cfg::I2C_CFG_SPEC>,
    _reserved16: [u8; 0x018c],
    #[doc = "0x200 - Transmitter control"]
    pub tx_ctrl: crate::Reg<tx_ctrl::TX_CTRL_SPEC>,
    #[doc = "0x204 - Transmitter FIFO control"]
    pub tx_fifo_ctrl: crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>,
    #[doc = "0x208 - Transmitter FIFO status"]
    pub tx_fifo_status: crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>,
    _reserved19: [u8; 0x34],
    #[doc = "0x240 - Transmitter FIFO write"]
    pub tx_fifo_wr: crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>,
    _reserved20: [u8; 0xbc],
    #[doc = "0x300 - Receiver control"]
    pub rx_ctrl: crate::Reg<rx_ctrl::RX_CTRL_SPEC>,
    #[doc = "0x304 - Receiver FIFO control"]
    pub rx_fifo_ctrl: crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>,
    #[doc = "0x308 - Receiver FIFO status"]
    pub rx_fifo_status: crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x310 - Slave address and mask"]
    pub rx_match: crate::Reg<rx_match::RX_MATCH_SPEC>,
    _reserved24: [u8; 0x2c],
    #[doc = "0x340 - Receiver FIFO read"]
    pub rx_fifo_rd: crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>,
    #[doc = "0x344 - Receiver FIFO read silent"]
    pub rx_fifo_rd_silent: crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>,
    _reserved26: [u8; 0x0ab8],
    #[doc = "0xe00 - Active clocked interrupt signal"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved27: [u8; 0x7c],
    #[doc = "0xe80 - Externally clocked I2C interrupt request"]
    pub intr_i2c_ec: crate::Reg<intr_i2c_ec::INTR_I2C_EC_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0xe88 - Externally clocked I2C interrupt mask"]
    pub intr_i2c_ec_mask: crate::Reg<intr_i2c_ec_mask::INTR_I2C_EC_MASK_SPEC>,
    #[doc = "0xe8c - Externally clocked I2C interrupt masked"]
    pub intr_i2c_ec_masked: crate::Reg<intr_i2c_ec_masked::INTR_I2C_EC_MASKED_SPEC>,
    _reserved30: [u8; 0x30],
    #[doc = "0xec0 - Externally clocked SPI interrupt request"]
    pub intr_spi_ec: crate::Reg<intr_spi_ec::INTR_SPI_EC_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0xec8 - Externally clocked SPI interrupt mask"]
    pub intr_spi_ec_mask: crate::Reg<intr_spi_ec_mask::INTR_SPI_EC_MASK_SPEC>,
    #[doc = "0xecc - Externally clocked SPI interrupt masked"]
    pub intr_spi_ec_masked: crate::Reg<intr_spi_ec_masked::INTR_SPI_EC_MASKED_SPEC>,
    _reserved33: [u8; 0x30],
    #[doc = "0xf00 - Master interrupt request"]
    pub intr_m: crate::Reg<intr_m::INTR_M_SPEC>,
    #[doc = "0xf04 - Master interrupt set request"]
    pub intr_m_set: crate::Reg<intr_m_set::INTR_M_SET_SPEC>,
    #[doc = "0xf08 - Master interrupt mask"]
    pub intr_m_mask: crate::Reg<intr_m_mask::INTR_M_MASK_SPEC>,
    #[doc = "0xf0c - Master interrupt masked request"]
    pub intr_m_masked: crate::Reg<intr_m_masked::INTR_M_MASKED_SPEC>,
    _reserved37: [u8; 0x30],
    #[doc = "0xf40 - Slave interrupt request"]
    pub intr_s: crate::Reg<intr_s::INTR_S_SPEC>,
    #[doc = "0xf44 - Slave interrupt set request"]
    pub intr_s_set: crate::Reg<intr_s_set::INTR_S_SET_SPEC>,
    #[doc = "0xf48 - Slave interrupt mask"]
    pub intr_s_mask: crate::Reg<intr_s_mask::INTR_S_MASK_SPEC>,
    #[doc = "0xf4c - Slave interrupt masked request"]
    pub intr_s_masked: crate::Reg<intr_s_masked::INTR_S_MASKED_SPEC>,
    _reserved41: [u8; 0x30],
    #[doc = "0xf80 - Transmitter interrupt request"]
    pub intr_tx: crate::Reg<intr_tx::INTR_TX_SPEC>,
    #[doc = "0xf84 - Transmitter interrupt set request"]
    pub intr_tx_set: crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>,
    #[doc = "0xf88 - Transmitter interrupt mask"]
    pub intr_tx_mask: crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>,
    #[doc = "0xf8c - Transmitter interrupt masked request"]
    pub intr_tx_masked: crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>,
    _reserved45: [u8; 0x30],
    #[doc = "0xfc0 - Receiver interrupt request"]
    pub intr_rx: crate::Reg<intr_rx::INTR_RX_SPEC>,
    #[doc = "0xfc4 - Receiver interrupt set request"]
    pub intr_rx_set: crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>,
    #[doc = "0xfc8 - Receiver interrupt mask"]
    pub intr_rx_mask: crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>,
    #[doc = "0xfcc - Receiver interrupt masked request"]
    pub intr_rx_masked: crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Generic control"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Generic status"]
pub mod status;
#[doc = "CMD_RESP_CTRL register accessor: an alias for `Reg<CMD_RESP_CTRL_SPEC>`"]
pub type CMD_RESP_CTRL = crate::Reg<cmd_resp_ctrl::CMD_RESP_CTRL_SPEC>;
#[doc = "Command/response control"]
pub mod cmd_resp_ctrl;
#[doc = "CMD_RESP_STATUS register accessor: an alias for `Reg<CMD_RESP_STATUS_SPEC>`"]
pub type CMD_RESP_STATUS = crate::Reg<cmd_resp_status::CMD_RESP_STATUS_SPEC>;
#[doc = "Command/response status"]
pub mod cmd_resp_status;
#[doc = "SPI_CTRL register accessor: an alias for `Reg<SPI_CTRL_SPEC>`"]
pub type SPI_CTRL = crate::Reg<spi_ctrl::SPI_CTRL_SPEC>;
#[doc = "SPI control"]
pub mod spi_ctrl;
#[doc = "SPI_STATUS register accessor: an alias for `Reg<SPI_STATUS_SPEC>`"]
pub type SPI_STATUS = crate::Reg<spi_status::SPI_STATUS_SPEC>;
#[doc = "SPI status"]
pub mod spi_status;
#[doc = "UART_CTRL register accessor: an alias for `Reg<UART_CTRL_SPEC>`"]
pub type UART_CTRL = crate::Reg<uart_ctrl::UART_CTRL_SPEC>;
#[doc = "UART control"]
pub mod uart_ctrl;
#[doc = "UART_TX_CTRL register accessor: an alias for `Reg<UART_TX_CTRL_SPEC>`"]
pub type UART_TX_CTRL = crate::Reg<uart_tx_ctrl::UART_TX_CTRL_SPEC>;
#[doc = "UART transmitter control"]
pub mod uart_tx_ctrl;
#[doc = "UART_RX_CTRL register accessor: an alias for `Reg<UART_RX_CTRL_SPEC>`"]
pub type UART_RX_CTRL = crate::Reg<uart_rx_ctrl::UART_RX_CTRL_SPEC>;
#[doc = "UART receiver control"]
pub mod uart_rx_ctrl;
#[doc = "UART_RX_STATUS register accessor: an alias for `Reg<UART_RX_STATUS_SPEC>`"]
pub type UART_RX_STATUS = crate::Reg<uart_rx_status::UART_RX_STATUS_SPEC>;
#[doc = "UART receiver status"]
pub mod uart_rx_status;
#[doc = "UART_FLOW_CTRL register accessor: an alias for `Reg<UART_FLOW_CTRL_SPEC>`"]
pub type UART_FLOW_CTRL = crate::Reg<uart_flow_ctrl::UART_FLOW_CTRL_SPEC>;
#[doc = "UART flow control"]
pub mod uart_flow_ctrl;
#[doc = "I2C_CTRL register accessor: an alias for `Reg<I2C_CTRL_SPEC>`"]
pub type I2C_CTRL = crate::Reg<i2c_ctrl::I2C_CTRL_SPEC>;
#[doc = "I2C control"]
pub mod i2c_ctrl;
#[doc = "I2C_STATUS register accessor: an alias for `Reg<I2C_STATUS_SPEC>`"]
pub type I2C_STATUS = crate::Reg<i2c_status::I2C_STATUS_SPEC>;
#[doc = "I2C status"]
pub mod i2c_status;
#[doc = "I2C_M_CMD register accessor: an alias for `Reg<I2C_M_CMD_SPEC>`"]
pub type I2C_M_CMD = crate::Reg<i2c_m_cmd::I2C_M_CMD_SPEC>;
#[doc = "I2C master command"]
pub mod i2c_m_cmd;
#[doc = "I2C_S_CMD register accessor: an alias for `Reg<I2C_S_CMD_SPEC>`"]
pub type I2C_S_CMD = crate::Reg<i2c_s_cmd::I2C_S_CMD_SPEC>;
#[doc = "I2C slave command"]
pub mod i2c_s_cmd;
#[doc = "I2C_CFG register accessor: an alias for `Reg<I2C_CFG_SPEC>`"]
pub type I2C_CFG = crate::Reg<i2c_cfg::I2C_CFG_SPEC>;
#[doc = "I2C configuration"]
pub mod i2c_cfg;
#[doc = "TX_CTRL register accessor: an alias for `Reg<TX_CTRL_SPEC>`"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = "Transmitter control"]
pub mod tx_ctrl;
#[doc = "TX_FIFO_CTRL register accessor: an alias for `Reg<TX_FIFO_CTRL_SPEC>`"]
pub type TX_FIFO_CTRL = crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>;
#[doc = "Transmitter FIFO control"]
pub mod tx_fifo_ctrl;
#[doc = "TX_FIFO_STATUS register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "Transmitter FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "Transmitter FIFO write"]
pub mod tx_fifo_wr;
#[doc = "RX_CTRL register accessor: an alias for `Reg<RX_CTRL_SPEC>`"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Receiver control"]
pub mod rx_ctrl;
#[doc = "RX_FIFO_CTRL register accessor: an alias for `Reg<RX_FIFO_CTRL_SPEC>`"]
pub type RX_FIFO_CTRL = crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>;
#[doc = "Receiver FIFO control"]
pub mod rx_fifo_ctrl;
#[doc = "RX_FIFO_STATUS register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "Receiver FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_MATCH register accessor: an alias for `Reg<RX_MATCH_SPEC>`"]
pub type RX_MATCH = crate::Reg<rx_match::RX_MATCH_SPEC>;
#[doc = "Slave address and mask"]
pub mod rx_match;
#[doc = "RX_FIFO_RD register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "Receiver FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "Receiver FIFO read silent"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Active clocked interrupt signal"]
pub mod intr_cause;
#[doc = "INTR_I2C_EC register accessor: an alias for `Reg<INTR_I2C_EC_SPEC>`"]
pub type INTR_I2C_EC = crate::Reg<intr_i2c_ec::INTR_I2C_EC_SPEC>;
#[doc = "Externally clocked I2C interrupt request"]
pub mod intr_i2c_ec;
#[doc = "INTR_I2C_EC_MASK register accessor: an alias for `Reg<INTR_I2C_EC_MASK_SPEC>`"]
pub type INTR_I2C_EC_MASK = crate::Reg<intr_i2c_ec_mask::INTR_I2C_EC_MASK_SPEC>;
#[doc = "Externally clocked I2C interrupt mask"]
pub mod intr_i2c_ec_mask;
#[doc = "INTR_I2C_EC_MASKED register accessor: an alias for `Reg<INTR_I2C_EC_MASKED_SPEC>`"]
pub type INTR_I2C_EC_MASKED = crate::Reg<intr_i2c_ec_masked::INTR_I2C_EC_MASKED_SPEC>;
#[doc = "Externally clocked I2C interrupt masked"]
pub mod intr_i2c_ec_masked;
#[doc = "INTR_SPI_EC register accessor: an alias for `Reg<INTR_SPI_EC_SPEC>`"]
pub type INTR_SPI_EC = crate::Reg<intr_spi_ec::INTR_SPI_EC_SPEC>;
#[doc = "Externally clocked SPI interrupt request"]
pub mod intr_spi_ec;
#[doc = "INTR_SPI_EC_MASK register accessor: an alias for `Reg<INTR_SPI_EC_MASK_SPEC>`"]
pub type INTR_SPI_EC_MASK = crate::Reg<intr_spi_ec_mask::INTR_SPI_EC_MASK_SPEC>;
#[doc = "Externally clocked SPI interrupt mask"]
pub mod intr_spi_ec_mask;
#[doc = "INTR_SPI_EC_MASKED register accessor: an alias for `Reg<INTR_SPI_EC_MASKED_SPEC>`"]
pub type INTR_SPI_EC_MASKED = crate::Reg<intr_spi_ec_masked::INTR_SPI_EC_MASKED_SPEC>;
#[doc = "Externally clocked SPI interrupt masked"]
pub mod intr_spi_ec_masked;
#[doc = "INTR_M register accessor: an alias for `Reg<INTR_M_SPEC>`"]
pub type INTR_M = crate::Reg<intr_m::INTR_M_SPEC>;
#[doc = "Master interrupt request"]
pub mod intr_m;
#[doc = "INTR_M_SET register accessor: an alias for `Reg<INTR_M_SET_SPEC>`"]
pub type INTR_M_SET = crate::Reg<intr_m_set::INTR_M_SET_SPEC>;
#[doc = "Master interrupt set request"]
pub mod intr_m_set;
#[doc = "INTR_M_MASK register accessor: an alias for `Reg<INTR_M_MASK_SPEC>`"]
pub type INTR_M_MASK = crate::Reg<intr_m_mask::INTR_M_MASK_SPEC>;
#[doc = "Master interrupt mask"]
pub mod intr_m_mask;
#[doc = "INTR_M_MASKED register accessor: an alias for `Reg<INTR_M_MASKED_SPEC>`"]
pub type INTR_M_MASKED = crate::Reg<intr_m_masked::INTR_M_MASKED_SPEC>;
#[doc = "Master interrupt masked request"]
pub mod intr_m_masked;
#[doc = "INTR_S register accessor: an alias for `Reg<INTR_S_SPEC>`"]
pub type INTR_S = crate::Reg<intr_s::INTR_S_SPEC>;
#[doc = "Slave interrupt request"]
pub mod intr_s;
#[doc = "INTR_S_SET register accessor: an alias for `Reg<INTR_S_SET_SPEC>`"]
pub type INTR_S_SET = crate::Reg<intr_s_set::INTR_S_SET_SPEC>;
#[doc = "Slave interrupt set request"]
pub mod intr_s_set;
#[doc = "INTR_S_MASK register accessor: an alias for `Reg<INTR_S_MASK_SPEC>`"]
pub type INTR_S_MASK = crate::Reg<intr_s_mask::INTR_S_MASK_SPEC>;
#[doc = "Slave interrupt mask"]
pub mod intr_s_mask;
#[doc = "INTR_S_MASKED register accessor: an alias for `Reg<INTR_S_MASKED_SPEC>`"]
pub type INTR_S_MASKED = crate::Reg<intr_s_masked::INTR_S_MASKED_SPEC>;
#[doc = "Slave interrupt masked request"]
pub mod intr_s_masked;
#[doc = "INTR_TX register accessor: an alias for `Reg<INTR_TX_SPEC>`"]
pub type INTR_TX = crate::Reg<intr_tx::INTR_TX_SPEC>;
#[doc = "Transmitter interrupt request"]
pub mod intr_tx;
#[doc = "INTR_TX_SET register accessor: an alias for `Reg<INTR_TX_SET_SPEC>`"]
pub type INTR_TX_SET = crate::Reg<intr_tx_set::INTR_TX_SET_SPEC>;
#[doc = "Transmitter interrupt set request"]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK register accessor: an alias for `Reg<INTR_TX_MASK_SPEC>`"]
pub type INTR_TX_MASK = crate::Reg<intr_tx_mask::INTR_TX_MASK_SPEC>;
#[doc = "Transmitter interrupt mask"]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED register accessor: an alias for `Reg<INTR_TX_MASKED_SPEC>`"]
pub type INTR_TX_MASKED = crate::Reg<intr_tx_masked::INTR_TX_MASKED_SPEC>;
#[doc = "Transmitter interrupt masked request"]
pub mod intr_tx_masked;
#[doc = "INTR_RX register accessor: an alias for `Reg<INTR_RX_SPEC>`"]
pub type INTR_RX = crate::Reg<intr_rx::INTR_RX_SPEC>;
#[doc = "Receiver interrupt request"]
pub mod intr_rx;
#[doc = "INTR_RX_SET register accessor: an alias for `Reg<INTR_RX_SET_SPEC>`"]
pub type INTR_RX_SET = crate::Reg<intr_rx_set::INTR_RX_SET_SPEC>;
#[doc = "Receiver interrupt set request"]
pub mod intr_rx_set;
#[doc = "INTR_RX_MASK register accessor: an alias for `Reg<INTR_RX_MASK_SPEC>`"]
pub type INTR_RX_MASK = crate::Reg<intr_rx_mask::INTR_RX_MASK_SPEC>;
#[doc = "Receiver interrupt mask"]
pub mod intr_rx_mask;
#[doc = "INTR_RX_MASKED register accessor: an alias for `Reg<INTR_RX_MASKED_SPEC>`"]
pub type INTR_RX_MASKED = crate::Reg<intr_rx_masked::INTR_RX_MASKED_SPEC>;
#[doc = "Receiver interrupt masked request"]
pub mod intr_rx_masked;
