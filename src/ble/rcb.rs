#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RCB control register."]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RCB status register."]
pub mod status;
#[doc = "TX_CTRL register accessor: an alias for `Reg<TX_CTRL_SPEC>`"]
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
#[doc = "Transmitter control register."]
pub mod tx_ctrl;
#[doc = "TX_FIFO_CTRL register accessor: an alias for `Reg<TX_FIFO_CTRL_SPEC>`"]
pub type TX_FIFO_CTRL = crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>;
#[doc = "Transmitter FIFO control register."]
pub mod tx_fifo_ctrl;
#[doc = "TX_FIFO_STATUS register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "Transmitter FIFO status register."]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "Transmitter FIFO write register."]
pub mod tx_fifo_wr;
#[doc = "RX_CTRL register accessor: an alias for `Reg<RX_CTRL_SPEC>`"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Receiver control register."]
pub mod rx_ctrl;
#[doc = "RX_FIFO_CTRL register accessor: an alias for `Reg<RX_FIFO_CTRL_SPEC>`"]
pub type RX_FIFO_CTRL = crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>;
#[doc = "Receiver FIFO control register."]
pub mod rx_fifo_ctrl;
#[doc = "RX_FIFO_STATUS register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "Receiver FIFO status register."]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "Receiver FIFO read register."]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "Receiver FIFO read register."]
pub mod rx_fifo_rd_silent;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Master interrupt request register."]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Master interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Master interrupt mask register."]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Master interrupt masked request register"]
pub mod intr_masked;
#[doc = r"Register block"]
#[repr(C)]
pub struct RCBLL {
    #[doc = "0x00 - RCB LL control register."]
    pub ctrl: crate::Reg<self::rcbll::ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Master interrupt request register."]
    pub intr: crate::Reg<self::rcbll::intr::INTR_SPEC>,
    #[doc = "0x14 - Master interrupt set request register"]
    pub intr_set: crate::Reg<self::rcbll::intr_set::INTR_SET_SPEC>,
    #[doc = "0x18 - Master interrupt mask register."]
    pub intr_mask: crate::Reg<self::rcbll::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x1c - Master interrupt masked request register"]
    pub intr_masked: crate::Reg<self::rcbll::intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x20 - Address of Register#1 in Radio (MDON)"]
    pub radio_reg1_addr: crate::Reg<self::rcbll::radio_reg1_addr::RADIO_REG1_ADDR_SPEC>,
    #[doc = "0x24 - Address of Register#2 in Radio (RSSI)"]
    pub radio_reg2_addr: crate::Reg<self::rcbll::radio_reg2_addr::RADIO_REG2_ADDR_SPEC>,
    #[doc = "0x28 - Address of Register#3 in Radio (ACCL)"]
    pub radio_reg3_addr: crate::Reg<self::rcbll::radio_reg3_addr::RADIO_REG3_ADDR_SPEC>,
    #[doc = "0x2c - Address of Register#4 in Radio (ACCH)"]
    pub radio_reg4_addr: crate::Reg<self::rcbll::radio_reg4_addr::RADIO_REG4_ADDR_SPEC>,
    #[doc = "0x30 - Address of Register#5 in Radio (RSSI ENERGY)"]
    pub radio_reg5_addr: crate::Reg<self::rcbll::radio_reg5_addr::RADIO_REG5_ADDR_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - N/A"]
    pub cpu_write_reg: crate::Reg<self::rcbll::cpu_write_reg::CPU_WRITE_REG_SPEC>,
    #[doc = "0x44 - N/A"]
    pub cpu_read_reg: crate::Reg<self::rcbll::cpu_read_reg::CPU_READ_REG_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Radio Control Bus (RCB) & Link Layer controller"]
pub mod rcbll;
