#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Generic control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Generic status"]
    pub status: STATUS,
    #[doc = "0x08 - Command/response control"]
    pub cmd_resp_ctrl: CMD_RESP_CTRL,
    #[doc = "0x0c - Command/response status"]
    pub cmd_resp_status: CMD_RESP_STATUS,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - SPI control"]
    pub spi_ctrl: SPI_CTRL,
    #[doc = "0x24 - SPI status"]
    pub spi_status: SPI_STATUS,
    _reserved6: [u8; 24usize],
    #[doc = "0x40 - UART control"]
    pub uart_ctrl: UART_CTRL,
    #[doc = "0x44 - UART transmitter control"]
    pub uart_tx_ctrl: UART_TX_CTRL,
    #[doc = "0x48 - UART receiver control"]
    pub uart_rx_ctrl: UART_RX_CTRL,
    #[doc = "0x4c - UART receiver status"]
    pub uart_rx_status: UART_RX_STATUS,
    #[doc = "0x50 - UART flow control"]
    pub uart_flow_ctrl: UART_FLOW_CTRL,
    _reserved11: [u8; 12usize],
    #[doc = "0x60 - I2C control"]
    pub i2c_ctrl: I2C_CTRL,
    #[doc = "0x64 - I2C status"]
    pub i2c_status: I2C_STATUS,
    #[doc = "0x68 - I2C master command"]
    pub i2c_m_cmd: I2C_M_CMD,
    #[doc = "0x6c - I2C slave command"]
    pub i2c_s_cmd: I2C_S_CMD,
    #[doc = "0x70 - I2C configuration"]
    pub i2c_cfg: I2C_CFG,
    _reserved16: [u8; 140usize],
    #[doc = "0x100 - Digital DfT control"]
    pub ddft_ctrl: DDFT_CTRL,
    _reserved17: [u8; 252usize],
    #[doc = "0x200 - Transmitter control"]
    pub tx_ctrl: TX_CTRL,
    #[doc = "0x204 - Transmitter FIFO control"]
    pub tx_fifo_ctrl: TX_FIFO_CTRL,
    #[doc = "0x208 - Transmitter FIFO status"]
    pub tx_fifo_status: TX_FIFO_STATUS,
    _reserved20: [u8; 52usize],
    #[doc = "0x240 - Transmitter FIFO write"]
    pub tx_fifo_wr: TX_FIFO_WR,
    _reserved21: [u8; 188usize],
    #[doc = "0x300 - Receiver control"]
    pub rx_ctrl: RX_CTRL,
    #[doc = "0x304 - Receiver FIFO control"]
    pub rx_fifo_ctrl: RX_FIFO_CTRL,
    #[doc = "0x308 - Receiver FIFO status"]
    pub rx_fifo_status: RX_FIFO_STATUS,
    _reserved24: [u8; 4usize],
    #[doc = "0x310 - Slave address and mask"]
    pub rx_match: RX_MATCH,
    _reserved25: [u8; 44usize],
    #[doc = "0x340 - Receiver FIFO read"]
    pub rx_fifo_rd: RX_FIFO_RD,
    #[doc = "0x344 - Receiver FIFO read silent"]
    pub rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved27: [u8; 2744usize],
    #[doc = "0xe00 - Active clocked interrupt signal"]
    pub intr_cause: INTR_CAUSE,
    _reserved28: [u8; 124usize],
    #[doc = "0xe80 - Externally clocked I2C interrupt request"]
    pub intr_i2c_ec: INTR_I2C_EC,
    _reserved29: [u8; 4usize],
    #[doc = "0xe88 - Externally clocked I2C interrupt mask"]
    pub intr_i2c_ec_mask: INTR_I2C_EC_MASK,
    #[doc = "0xe8c - Externally clocked I2C interrupt masked"]
    pub intr_i2c_ec_masked: INTR_I2C_EC_MASKED,
    _reserved31: [u8; 48usize],
    #[doc = "0xec0 - Externally clocked SPI interrupt request"]
    pub intr_spi_ec: INTR_SPI_EC,
    _reserved32: [u8; 4usize],
    #[doc = "0xec8 - Externally clocked SPI interrupt mask"]
    pub intr_spi_ec_mask: INTR_SPI_EC_MASK,
    #[doc = "0xecc - Externally clocked SPI interrupt masked"]
    pub intr_spi_ec_masked: INTR_SPI_EC_MASKED,
    _reserved34: [u8; 48usize],
    #[doc = "0xf00 - Master interrupt request"]
    pub intr_m: INTR_M,
    #[doc = "0xf04 - Master interrupt set request"]
    pub intr_m_set: INTR_M_SET,
    #[doc = "0xf08 - Master interrupt mask"]
    pub intr_m_mask: INTR_M_MASK,
    #[doc = "0xf0c - Master interrupt masked request"]
    pub intr_m_masked: INTR_M_MASKED,
    _reserved38: [u8; 48usize],
    #[doc = "0xf40 - Slave interrupt request"]
    pub intr_s: INTR_S,
    #[doc = "0xf44 - Slave interrupt set request"]
    pub intr_s_set: INTR_S_SET,
    #[doc = "0xf48 - Slave interrupt mask"]
    pub intr_s_mask: INTR_S_MASK,
    #[doc = "0xf4c - Slave interrupt masked request"]
    pub intr_s_masked: INTR_S_MASKED,
    _reserved42: [u8; 48usize],
    #[doc = "0xf80 - Transmitter interrupt request"]
    pub intr_tx: INTR_TX,
    #[doc = "0xf84 - Transmitter interrupt set request"]
    pub intr_tx_set: INTR_TX_SET,
    #[doc = "0xf88 - Transmitter interrupt mask"]
    pub intr_tx_mask: INTR_TX_MASK,
    #[doc = "0xf8c - Transmitter interrupt masked request"]
    pub intr_tx_masked: INTR_TX_MASKED,
    _reserved46: [u8; 48usize],
    #[doc = "0xfc0 - Receiver interrupt request"]
    pub intr_rx: INTR_RX,
    #[doc = "0xfc4 - Receiver interrupt set request"]
    pub intr_rx_set: INTR_RX_SET,
    #[doc = "0xfc8 - Receiver interrupt mask"]
    pub intr_rx_mask: INTR_RX_MASK,
    #[doc = "0xfcc - Receiver interrupt masked request"]
    pub intr_rx_masked: INTR_RX_MASKED,
}
#[doc = "Generic control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Generic control"]
pub mod ctrl;
#[doc = "Generic status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Generic status"]
pub mod status;
#[doc = "Command/response control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd_resp_ctrl](cmd_resp_ctrl) module"]
pub type CMD_RESP_CTRL = crate::Reg<u32, _CMD_RESP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RESP_CTRL;
#[doc = "`read()` method returns [cmd_resp_ctrl::R](cmd_resp_ctrl::R) reader structure"]
impl crate::Readable for CMD_RESP_CTRL {}
#[doc = "`write(|w| ..)` method takes [cmd_resp_ctrl::W](cmd_resp_ctrl::W) writer structure"]
impl crate::Writable for CMD_RESP_CTRL {}
#[doc = "Command/response control"]
pub mod cmd_resp_ctrl;
#[doc = "Command/response status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd_resp_status](cmd_resp_status) module"]
pub type CMD_RESP_STATUS = crate::Reg<u32, _CMD_RESP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RESP_STATUS;
#[doc = "`read()` method returns [cmd_resp_status::R](cmd_resp_status::R) reader structure"]
impl crate::Readable for CMD_RESP_STATUS {}
#[doc = "Command/response status"]
pub mod cmd_resp_status;
#[doc = "SPI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ctrl](spi_ctrl) module"]
pub type SPI_CTRL = crate::Reg<u32, _SPI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRL;
#[doc = "`read()` method returns [spi_ctrl::R](spi_ctrl::R) reader structure"]
impl crate::Readable for SPI_CTRL {}
#[doc = "`write(|w| ..)` method takes [spi_ctrl::W](spi_ctrl::W) writer structure"]
impl crate::Writable for SPI_CTRL {}
#[doc = "SPI control"]
pub mod spi_ctrl;
#[doc = "SPI status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_status](spi_status) module"]
pub type SPI_STATUS = crate::Reg<u32, _SPI_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_STATUS;
#[doc = "`read()` method returns [spi_status::R](spi_status::R) reader structure"]
impl crate::Readable for SPI_STATUS {}
#[doc = "SPI status"]
pub mod spi_status;
#[doc = "UART control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_ctrl](uart_ctrl) module"]
pub type UART_CTRL = crate::Reg<u32, _UART_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CTRL;
#[doc = "`read()` method returns [uart_ctrl::R](uart_ctrl::R) reader structure"]
impl crate::Readable for UART_CTRL {}
#[doc = "`write(|w| ..)` method takes [uart_ctrl::W](uart_ctrl::W) writer structure"]
impl crate::Writable for UART_CTRL {}
#[doc = "UART control"]
pub mod uart_ctrl;
#[doc = "UART transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_tx_ctrl](uart_tx_ctrl) module"]
pub type UART_TX_CTRL = crate::Reg<u32, _UART_TX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_TX_CTRL;
#[doc = "`read()` method returns [uart_tx_ctrl::R](uart_tx_ctrl::R) reader structure"]
impl crate::Readable for UART_TX_CTRL {}
#[doc = "`write(|w| ..)` method takes [uart_tx_ctrl::W](uart_tx_ctrl::W) writer structure"]
impl crate::Writable for UART_TX_CTRL {}
#[doc = "UART transmitter control"]
pub mod uart_tx_ctrl;
#[doc = "UART receiver control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rx_ctrl](uart_rx_ctrl) module"]
pub type UART_RX_CTRL = crate::Reg<u32, _UART_RX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RX_CTRL;
#[doc = "`read()` method returns [uart_rx_ctrl::R](uart_rx_ctrl::R) reader structure"]
impl crate::Readable for UART_RX_CTRL {}
#[doc = "`write(|w| ..)` method takes [uart_rx_ctrl::W](uart_rx_ctrl::W) writer structure"]
impl crate::Writable for UART_RX_CTRL {}
#[doc = "UART receiver control"]
pub mod uart_rx_ctrl;
#[doc = "UART receiver status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_rx_status](uart_rx_status) module"]
pub type UART_RX_STATUS = crate::Reg<u32, _UART_RX_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RX_STATUS;
#[doc = "`read()` method returns [uart_rx_status::R](uart_rx_status::R) reader structure"]
impl crate::Readable for UART_RX_STATUS {}
#[doc = "UART receiver status"]
pub mod uart_rx_status;
#[doc = "UART flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uart_flow_ctrl](uart_flow_ctrl) module"]
pub type UART_FLOW_CTRL = crate::Reg<u32, _UART_FLOW_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_FLOW_CTRL;
#[doc = "`read()` method returns [uart_flow_ctrl::R](uart_flow_ctrl::R) reader structure"]
impl crate::Readable for UART_FLOW_CTRL {}
#[doc = "`write(|w| ..)` method takes [uart_flow_ctrl::W](uart_flow_ctrl::W) writer structure"]
impl crate::Writable for UART_FLOW_CTRL {}
#[doc = "UART flow control"]
pub mod uart_flow_ctrl;
#[doc = "I2C control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_ctrl](i2c_ctrl) module"]
pub type I2C_CTRL = crate::Reg<u32, _I2C_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTRL;
#[doc = "`read()` method returns [i2c_ctrl::R](i2c_ctrl::R) reader structure"]
impl crate::Readable for I2C_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl::W](i2c_ctrl::W) writer structure"]
impl crate::Writable for I2C_CTRL {}
#[doc = "I2C control"]
pub mod i2c_ctrl;
#[doc = "I2C status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_status](i2c_status) module"]
pub type I2C_STATUS = crate::Reg<u32, _I2C_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_STATUS;
#[doc = "`read()` method returns [i2c_status::R](i2c_status::R) reader structure"]
impl crate::Readable for I2C_STATUS {}
#[doc = "I2C status"]
pub mod i2c_status;
#[doc = "I2C master command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_m_cmd](i2c_m_cmd) module"]
pub type I2C_M_CMD = crate::Reg<u32, _I2C_M_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_M_CMD;
#[doc = "`read()` method returns [i2c_m_cmd::R](i2c_m_cmd::R) reader structure"]
impl crate::Readable for I2C_M_CMD {}
#[doc = "`write(|w| ..)` method takes [i2c_m_cmd::W](i2c_m_cmd::W) writer structure"]
impl crate::Writable for I2C_M_CMD {}
#[doc = "I2C master command"]
pub mod i2c_m_cmd;
#[doc = "I2C slave command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_s_cmd](i2c_s_cmd) module"]
pub type I2C_S_CMD = crate::Reg<u32, _I2C_S_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_S_CMD;
#[doc = "`read()` method returns [i2c_s_cmd::R](i2c_s_cmd::R) reader structure"]
impl crate::Readable for I2C_S_CMD {}
#[doc = "`write(|w| ..)` method takes [i2c_s_cmd::W](i2c_s_cmd::W) writer structure"]
impl crate::Writable for I2C_S_CMD {}
#[doc = "I2C slave command"]
pub mod i2c_s_cmd;
#[doc = "I2C configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2c_cfg](i2c_cfg) module"]
pub type I2C_CFG = crate::Reg<u32, _I2C_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CFG;
#[doc = "`read()` method returns [i2c_cfg::R](i2c_cfg::R) reader structure"]
impl crate::Readable for I2C_CFG {}
#[doc = "`write(|w| ..)` method takes [i2c_cfg::W](i2c_cfg::W) writer structure"]
impl crate::Writable for I2C_CFG {}
#[doc = "I2C configuration"]
pub mod i2c_cfg;
#[doc = "Digital DfT control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddft_ctrl](ddft_ctrl) module"]
pub type DDFT_CTRL = crate::Reg<u32, _DDFT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDFT_CTRL;
#[doc = "`read()` method returns [ddft_ctrl::R](ddft_ctrl::R) reader structure"]
impl crate::Readable for DDFT_CTRL {}
#[doc = "`write(|w| ..)` method takes [ddft_ctrl::W](ddft_ctrl::W) writer structure"]
impl crate::Writable for DDFT_CTRL {}
#[doc = "Digital DfT control"]
pub mod ddft_ctrl;
#[doc = "Transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_ctrl](tx_ctrl) module"]
pub type TX_CTRL = crate::Reg<u32, _TX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CTRL;
#[doc = "`read()` method returns [tx_ctrl::R](tx_ctrl::R) reader structure"]
impl crate::Readable for TX_CTRL {}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](tx_ctrl::W) writer structure"]
impl crate::Writable for TX_CTRL {}
#[doc = "Transmitter control"]
pub mod tx_ctrl;
#[doc = "Transmitter FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_fifo_ctrl](tx_fifo_ctrl) module"]
pub type TX_FIFO_CTRL = crate::Reg<u32, _TX_FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_CTRL;
#[doc = "`read()` method returns [tx_fifo_ctrl::R](tx_fifo_ctrl::R) reader structure"]
impl crate::Readable for TX_FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctrl::W](tx_fifo_ctrl::W) writer structure"]
impl crate::Writable for TX_FIFO_CTRL {}
#[doc = "Transmitter FIFO control"]
pub mod tx_fifo_ctrl;
#[doc = "Transmitter FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_fifo_status](tx_fifo_status) module"]
pub type TX_FIFO_STATUS = crate::Reg<u32, _TX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_STATUS;
#[doc = "`read()` method returns [tx_fifo_status::R](tx_fifo_status::R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS {}
#[doc = "Transmitter FIFO status"]
pub mod tx_fifo_status;
#[doc = "Transmitter FIFO write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_fifo_wr](tx_fifo_wr) module"]
pub type TX_FIFO_WR = crate::Reg<u32, _TX_FIFO_WR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_WR;
#[doc = "`write(|w| ..)` method takes [tx_fifo_wr::W](tx_fifo_wr::W) writer structure"]
impl crate::Writable for TX_FIFO_WR {}
#[doc = "Transmitter FIFO write"]
pub mod tx_fifo_wr;
#[doc = "Receiver control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_ctrl](rx_ctrl) module"]
pub type RX_CTRL = crate::Reg<u32, _RX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CTRL;
#[doc = "`read()` method returns [rx_ctrl::R](rx_ctrl::R) reader structure"]
impl crate::Readable for RX_CTRL {}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](rx_ctrl::W) writer structure"]
impl crate::Writable for RX_CTRL {}
#[doc = "Receiver control"]
pub mod rx_ctrl;
#[doc = "Receiver FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_fifo_ctrl](rx_fifo_ctrl) module"]
pub type RX_FIFO_CTRL = crate::Reg<u32, _RX_FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_CTRL;
#[doc = "`read()` method returns [rx_fifo_ctrl::R](rx_fifo_ctrl::R) reader structure"]
impl crate::Readable for RX_FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctrl::W](rx_fifo_ctrl::W) writer structure"]
impl crate::Writable for RX_FIFO_CTRL {}
#[doc = "Receiver FIFO control"]
pub mod rx_fifo_ctrl;
#[doc = "Receiver FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_fifo_status](rx_fifo_status) module"]
pub type RX_FIFO_STATUS = crate::Reg<u32, _RX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_STATUS;
#[doc = "`read()` method returns [rx_fifo_status::R](rx_fifo_status::R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS {}
#[doc = "Receiver FIFO status"]
pub mod rx_fifo_status;
#[doc = "Slave address and mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_match](rx_match) module"]
pub type RX_MATCH = crate::Reg<u32, _RX_MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_MATCH;
#[doc = "`read()` method returns [rx_match::R](rx_match::R) reader structure"]
impl crate::Readable for RX_MATCH {}
#[doc = "`write(|w| ..)` method takes [rx_match::W](rx_match::W) writer structure"]
impl crate::Writable for RX_MATCH {}
#[doc = "Slave address and mask"]
pub mod rx_match;
#[doc = "Receiver FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_fifo_rd](rx_fifo_rd) module"]
pub type RX_FIFO_RD = crate::Reg<u32, _RX_FIFO_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD;
#[doc = "`read()` method returns [rx_fifo_rd::R](rx_fifo_rd::R) reader structure"]
impl crate::Readable for RX_FIFO_RD {}
#[doc = "Receiver FIFO read"]
pub mod rx_fifo_rd;
#[doc = "Receiver FIFO read silent\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_fifo_rd_silent](rx_fifo_rd_silent) module"]
pub type RX_FIFO_RD_SILENT = crate::Reg<u32, _RX_FIFO_RD_SILENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD_SILENT;
#[doc = "`read()` method returns [rx_fifo_rd_silent::R](rx_fifo_rd_silent::R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT {}
#[doc = "Receiver FIFO read silent"]
pub mod rx_fifo_rd_silent;
#[doc = "Active clocked interrupt signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_cause](intr_cause) module"]
pub type INTR_CAUSE = crate::Reg<u32, _INTR_CAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CAUSE;
#[doc = "`read()` method returns [intr_cause::R](intr_cause::R) reader structure"]
impl crate::Readable for INTR_CAUSE {}
#[doc = "Active clocked interrupt signal"]
pub mod intr_cause;
#[doc = "Externally clocked I2C interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_i2c_ec](intr_i2c_ec) module"]
pub type INTR_I2C_EC = crate::Reg<u32, _INTR_I2C_EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_I2C_EC;
#[doc = "`read()` method returns [intr_i2c_ec::R](intr_i2c_ec::R) reader structure"]
impl crate::Readable for INTR_I2C_EC {}
#[doc = "`write(|w| ..)` method takes [intr_i2c_ec::W](intr_i2c_ec::W) writer structure"]
impl crate::Writable for INTR_I2C_EC {}
#[doc = "Externally clocked I2C interrupt request"]
pub mod intr_i2c_ec;
#[doc = "Externally clocked I2C interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_i2c_ec_mask](intr_i2c_ec_mask) module"]
pub type INTR_I2C_EC_MASK = crate::Reg<u32, _INTR_I2C_EC_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_I2C_EC_MASK;
#[doc = "`read()` method returns [intr_i2c_ec_mask::R](intr_i2c_ec_mask::R) reader structure"]
impl crate::Readable for INTR_I2C_EC_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_i2c_ec_mask::W](intr_i2c_ec_mask::W) writer structure"]
impl crate::Writable for INTR_I2C_EC_MASK {}
#[doc = "Externally clocked I2C interrupt mask"]
pub mod intr_i2c_ec_mask;
#[doc = "Externally clocked I2C interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_i2c_ec_masked](intr_i2c_ec_masked) module"]
pub type INTR_I2C_EC_MASKED = crate::Reg<u32, _INTR_I2C_EC_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_I2C_EC_MASKED;
#[doc = "`read()` method returns [intr_i2c_ec_masked::R](intr_i2c_ec_masked::R) reader structure"]
impl crate::Readable for INTR_I2C_EC_MASKED {}
#[doc = "Externally clocked I2C interrupt masked"]
pub mod intr_i2c_ec_masked;
#[doc = "Externally clocked SPI interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_spi_ec](intr_spi_ec) module"]
pub type INTR_SPI_EC = crate::Reg<u32, _INTR_SPI_EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SPI_EC;
#[doc = "`read()` method returns [intr_spi_ec::R](intr_spi_ec::R) reader structure"]
impl crate::Readable for INTR_SPI_EC {}
#[doc = "`write(|w| ..)` method takes [intr_spi_ec::W](intr_spi_ec::W) writer structure"]
impl crate::Writable for INTR_SPI_EC {}
#[doc = "Externally clocked SPI interrupt request"]
pub mod intr_spi_ec;
#[doc = "Externally clocked SPI interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_spi_ec_mask](intr_spi_ec_mask) module"]
pub type INTR_SPI_EC_MASK = crate::Reg<u32, _INTR_SPI_EC_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SPI_EC_MASK;
#[doc = "`read()` method returns [intr_spi_ec_mask::R](intr_spi_ec_mask::R) reader structure"]
impl crate::Readable for INTR_SPI_EC_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_spi_ec_mask::W](intr_spi_ec_mask::W) writer structure"]
impl crate::Writable for INTR_SPI_EC_MASK {}
#[doc = "Externally clocked SPI interrupt mask"]
pub mod intr_spi_ec_mask;
#[doc = "Externally clocked SPI interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_spi_ec_masked](intr_spi_ec_masked) module"]
pub type INTR_SPI_EC_MASKED = crate::Reg<u32, _INTR_SPI_EC_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SPI_EC_MASKED;
#[doc = "`read()` method returns [intr_spi_ec_masked::R](intr_spi_ec_masked::R) reader structure"]
impl crate::Readable for INTR_SPI_EC_MASKED {}
#[doc = "Externally clocked SPI interrupt masked"]
pub mod intr_spi_ec_masked;
#[doc = "Master interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_m](intr_m) module"]
pub type INTR_M = crate::Reg<u32, _INTR_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_M;
#[doc = "`read()` method returns [intr_m::R](intr_m::R) reader structure"]
impl crate::Readable for INTR_M {}
#[doc = "`write(|w| ..)` method takes [intr_m::W](intr_m::W) writer structure"]
impl crate::Writable for INTR_M {}
#[doc = "Master interrupt request"]
pub mod intr_m;
#[doc = "Master interrupt set request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_m_set](intr_m_set) module"]
pub type INTR_M_SET = crate::Reg<u32, _INTR_M_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_M_SET;
#[doc = "`read()` method returns [intr_m_set::R](intr_m_set::R) reader structure"]
impl crate::Readable for INTR_M_SET {}
#[doc = "`write(|w| ..)` method takes [intr_m_set::W](intr_m_set::W) writer structure"]
impl crate::Writable for INTR_M_SET {}
#[doc = "Master interrupt set request"]
pub mod intr_m_set;
#[doc = "Master interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_m_mask](intr_m_mask) module"]
pub type INTR_M_MASK = crate::Reg<u32, _INTR_M_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_M_MASK;
#[doc = "`read()` method returns [intr_m_mask::R](intr_m_mask::R) reader structure"]
impl crate::Readable for INTR_M_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_m_mask::W](intr_m_mask::W) writer structure"]
impl crate::Writable for INTR_M_MASK {}
#[doc = "Master interrupt mask"]
pub mod intr_m_mask;
#[doc = "Master interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_m_masked](intr_m_masked) module"]
pub type INTR_M_MASKED = crate::Reg<u32, _INTR_M_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_M_MASKED;
#[doc = "`read()` method returns [intr_m_masked::R](intr_m_masked::R) reader structure"]
impl crate::Readable for INTR_M_MASKED {}
#[doc = "Master interrupt masked request"]
pub mod intr_m_masked;
#[doc = "Slave interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_s](intr_s) module"]
pub type INTR_S = crate::Reg<u32, _INTR_S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_S;
#[doc = "`read()` method returns [intr_s::R](intr_s::R) reader structure"]
impl crate::Readable for INTR_S {}
#[doc = "`write(|w| ..)` method takes [intr_s::W](intr_s::W) writer structure"]
impl crate::Writable for INTR_S {}
#[doc = "Slave interrupt request"]
pub mod intr_s;
#[doc = "Slave interrupt set request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_s_set](intr_s_set) module"]
pub type INTR_S_SET = crate::Reg<u32, _INTR_S_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_S_SET;
#[doc = "`read()` method returns [intr_s_set::R](intr_s_set::R) reader structure"]
impl crate::Readable for INTR_S_SET {}
#[doc = "`write(|w| ..)` method takes [intr_s_set::W](intr_s_set::W) writer structure"]
impl crate::Writable for INTR_S_SET {}
#[doc = "Slave interrupt set request"]
pub mod intr_s_set;
#[doc = "Slave interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_s_mask](intr_s_mask) module"]
pub type INTR_S_MASK = crate::Reg<u32, _INTR_S_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_S_MASK;
#[doc = "`read()` method returns [intr_s_mask::R](intr_s_mask::R) reader structure"]
impl crate::Readable for INTR_S_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_s_mask::W](intr_s_mask::W) writer structure"]
impl crate::Writable for INTR_S_MASK {}
#[doc = "Slave interrupt mask"]
pub mod intr_s_mask;
#[doc = "Slave interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_s_masked](intr_s_masked) module"]
pub type INTR_S_MASKED = crate::Reg<u32, _INTR_S_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_S_MASKED;
#[doc = "`read()` method returns [intr_s_masked::R](intr_s_masked::R) reader structure"]
impl crate::Readable for INTR_S_MASKED {}
#[doc = "Slave interrupt masked request"]
pub mod intr_s_masked;
#[doc = "Transmitter interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_tx](intr_tx) module"]
pub type INTR_TX = crate::Reg<u32, _INTR_TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TX;
#[doc = "`read()` method returns [intr_tx::R](intr_tx::R) reader structure"]
impl crate::Readable for INTR_TX {}
#[doc = "`write(|w| ..)` method takes [intr_tx::W](intr_tx::W) writer structure"]
impl crate::Writable for INTR_TX {}
#[doc = "Transmitter interrupt request"]
pub mod intr_tx;
#[doc = "Transmitter interrupt set request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_tx_set](intr_tx_set) module"]
pub type INTR_TX_SET = crate::Reg<u32, _INTR_TX_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TX_SET;
#[doc = "`read()` method returns [intr_tx_set::R](intr_tx_set::R) reader structure"]
impl crate::Readable for INTR_TX_SET {}
#[doc = "`write(|w| ..)` method takes [intr_tx_set::W](intr_tx_set::W) writer structure"]
impl crate::Writable for INTR_TX_SET {}
#[doc = "Transmitter interrupt set request"]
pub mod intr_tx_set;
#[doc = "Transmitter interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_tx_mask](intr_tx_mask) module"]
pub type INTR_TX_MASK = crate::Reg<u32, _INTR_TX_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TX_MASK;
#[doc = "`read()` method returns [intr_tx_mask::R](intr_tx_mask::R) reader structure"]
impl crate::Readable for INTR_TX_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_tx_mask::W](intr_tx_mask::W) writer structure"]
impl crate::Writable for INTR_TX_MASK {}
#[doc = "Transmitter interrupt mask"]
pub mod intr_tx_mask;
#[doc = "Transmitter interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_tx_masked](intr_tx_masked) module"]
pub type INTR_TX_MASKED = crate::Reg<u32, _INTR_TX_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_TX_MASKED;
#[doc = "`read()` method returns [intr_tx_masked::R](intr_tx_masked::R) reader structure"]
impl crate::Readable for INTR_TX_MASKED {}
#[doc = "Transmitter interrupt masked request"]
pub mod intr_tx_masked;
#[doc = "Receiver interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_rx](intr_rx) module"]
pub type INTR_RX = crate::Reg<u32, _INTR_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_RX;
#[doc = "`read()` method returns [intr_rx::R](intr_rx::R) reader structure"]
impl crate::Readable for INTR_RX {}
#[doc = "`write(|w| ..)` method takes [intr_rx::W](intr_rx::W) writer structure"]
impl crate::Writable for INTR_RX {}
#[doc = "Receiver interrupt request"]
pub mod intr_rx;
#[doc = "Receiver interrupt set request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_rx_set](intr_rx_set) module"]
pub type INTR_RX_SET = crate::Reg<u32, _INTR_RX_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_RX_SET;
#[doc = "`read()` method returns [intr_rx_set::R](intr_rx_set::R) reader structure"]
impl crate::Readable for INTR_RX_SET {}
#[doc = "`write(|w| ..)` method takes [intr_rx_set::W](intr_rx_set::W) writer structure"]
impl crate::Writable for INTR_RX_SET {}
#[doc = "Receiver interrupt set request"]
pub mod intr_rx_set;
#[doc = "Receiver interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_rx_mask](intr_rx_mask) module"]
pub type INTR_RX_MASK = crate::Reg<u32, _INTR_RX_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_RX_MASK;
#[doc = "`read()` method returns [intr_rx_mask::R](intr_rx_mask::R) reader structure"]
impl crate::Readable for INTR_RX_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_rx_mask::W](intr_rx_mask::W) writer structure"]
impl crate::Writable for INTR_RX_MASK {}
#[doc = "Receiver interrupt mask"]
pub mod intr_rx_mask;
#[doc = "Receiver interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_rx_masked](intr_rx_masked) module"]
pub type INTR_RX_MASKED = crate::Reg<u32, _INTR_RX_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_RX_MASKED;
#[doc = "`read()` method returns [intr_rx_masked::R](intr_rx_masked::R) reader structure"]
impl crate::Readable for INTR_RX_MASKED {}
#[doc = "Receiver interrupt masked request"]
pub mod intr_rx_masked;
