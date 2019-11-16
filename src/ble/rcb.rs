#[doc = "RCB control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RCB control register."]
pub mod ctrl;
#[doc = "RCB status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "RCB status register."]
pub mod status;
#[doc = "Transmitter control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](tx_ctrl) module"]
pub type TX_CTRL = crate::Reg<u32, _TX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_CTRL;
#[doc = "`read()` method returns [tx_ctrl::R](tx_ctrl::R) reader structure"]
impl crate::Readable for TX_CTRL {}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](tx_ctrl::W) writer structure"]
impl crate::Writable for TX_CTRL {}
#[doc = "Transmitter control register."]
pub mod tx_ctrl;
#[doc = "Transmitter FIFO control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctrl](tx_fifo_ctrl) module"]
pub type TX_FIFO_CTRL = crate::Reg<u32, _TX_FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_CTRL;
#[doc = "`read()` method returns [tx_fifo_ctrl::R](tx_fifo_ctrl::R) reader structure"]
impl crate::Readable for TX_FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctrl::W](tx_fifo_ctrl::W) writer structure"]
impl crate::Writable for TX_FIFO_CTRL {}
#[doc = "Transmitter FIFO control register."]
pub mod tx_fifo_ctrl;
#[doc = "Transmitter FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_status](tx_fifo_status) module"]
pub type TX_FIFO_STATUS = crate::Reg<u32, _TX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_STATUS;
#[doc = "`read()` method returns [tx_fifo_status::R](tx_fifo_status::R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS {}
#[doc = "Transmitter FIFO status register."]
pub mod tx_fifo_status;
#[doc = "Transmitter FIFO write register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_wr](tx_fifo_wr) module"]
pub type TX_FIFO_WR = crate::Reg<u32, _TX_FIFO_WR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_FIFO_WR;
#[doc = "`write(|w| ..)` method takes [tx_fifo_wr::W](tx_fifo_wr::W) writer structure"]
impl crate::Writable for TX_FIFO_WR {}
#[doc = "Transmitter FIFO write register."]
pub mod tx_fifo_wr;
#[doc = "Receiver control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl](rx_ctrl) module"]
pub type RX_CTRL = crate::Reg<u32, _RX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_CTRL;
#[doc = "`read()` method returns [rx_ctrl::R](rx_ctrl::R) reader structure"]
impl crate::Readable for RX_CTRL {}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](rx_ctrl::W) writer structure"]
impl crate::Writable for RX_CTRL {}
#[doc = "Receiver control register."]
pub mod rx_ctrl;
#[doc = "Receiver FIFO control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_ctrl](rx_fifo_ctrl) module"]
pub type RX_FIFO_CTRL = crate::Reg<u32, _RX_FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_CTRL;
#[doc = "`read()` method returns [rx_fifo_ctrl::R](rx_fifo_ctrl::R) reader structure"]
impl crate::Readable for RX_FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctrl::W](rx_fifo_ctrl::W) writer structure"]
impl crate::Writable for RX_FIFO_CTRL {}
#[doc = "Receiver FIFO control register."]
pub mod rx_fifo_ctrl;
#[doc = "Receiver FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](rx_fifo_status) module"]
pub type RX_FIFO_STATUS = crate::Reg<u32, _RX_FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_STATUS;
#[doc = "`read()` method returns [rx_fifo_status::R](rx_fifo_status::R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS {}
#[doc = "Receiver FIFO status register."]
pub mod rx_fifo_status;
#[doc = "Receiver FIFO read register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd](rx_fifo_rd) module"]
pub type RX_FIFO_RD = crate::Reg<u32, _RX_FIFO_RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD;
#[doc = "`read()` method returns [rx_fifo_rd::R](rx_fifo_rd::R) reader structure"]
impl crate::Readable for RX_FIFO_RD {}
#[doc = "Receiver FIFO read register."]
pub mod rx_fifo_rd;
#[doc = "Receiver FIFO read register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd_silent](rx_fifo_rd_silent) module"]
pub type RX_FIFO_RD_SILENT = crate::Reg<u32, _RX_FIFO_RD_SILENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_FIFO_RD_SILENT;
#[doc = "`read()` method returns [rx_fifo_rd_silent::R](rx_fifo_rd_silent::R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT {}
#[doc = "Receiver FIFO read register."]
pub mod rx_fifo_rd_silent;
#[doc = "Master interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Master interrupt request register."]
pub mod intr;
#[doc = "Master interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Master interrupt set request register"]
pub mod intr_set;
#[doc = "Master interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Master interrupt mask register."]
pub mod intr_mask;
#[doc = "Master interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Master interrupt masked request register"]
pub mod intr_masked;
#[doc = r"Register block"]
#[repr(C)]
pub struct RCBLL {
    #[doc = "0x00 - RCB LL control register."]
    pub ctrl: self::rcbll::CTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Master interrupt request register."]
    pub intr: self::rcbll::INTR,
    #[doc = "0x14 - Master interrupt set request register"]
    pub intr_set: self::rcbll::INTR_SET,
    #[doc = "0x18 - Master interrupt mask register."]
    pub intr_mask: self::rcbll::INTR_MASK,
    #[doc = "0x1c - Master interrupt masked request register"]
    pub intr_masked: self::rcbll::INTR_MASKED,
    #[doc = "0x20 - Address of Register#1 in Radio (MDON)"]
    pub radio_reg1_addr: self::rcbll::RADIO_REG1_ADDR,
    #[doc = "0x24 - Address of Register#2 in Radio (RSSI)"]
    pub radio_reg2_addr: self::rcbll::RADIO_REG2_ADDR,
    #[doc = "0x28 - Address of Register#3 in Radio (ACCL)"]
    pub radio_reg3_addr: self::rcbll::RADIO_REG3_ADDR,
    #[doc = "0x2c - Address of Register#4 in Radio (ACCH)"]
    pub radio_reg4_addr: self::rcbll::RADIO_REG4_ADDR,
    #[doc = "0x30 - Address of Register#5 in Radio (RSSI ENERGY)"]
    pub radio_reg5_addr: self::rcbll::RADIO_REG5_ADDR,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - N/A"]
    pub cpu_write_reg: self::rcbll::CPU_WRITE_REG,
    #[doc = "0x44 - N/A"]
    pub cpu_read_reg: self::rcbll::CPU_READ_REG,
}
#[doc = r"Register block"]
#[doc = "Radio Control Bus (RCB) & Link Layer controller"]
pub mod rcbll;
