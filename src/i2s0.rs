#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Clock control"]
    pub clock_ctl: crate::Reg<clock_ctl::CLOCK_CTL_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Command"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - Trigger control"]
    pub tr_ctl: crate::Reg<tr_ctl::TR_CTL_SPEC>,
    _reserved4: [u8; 0x3c],
    #[doc = "0x80 - Transmitter control"]
    pub tx_ctl: crate::Reg<tx_ctl::TX_CTL_SPEC>,
    #[doc = "0x84 - Transmitter watchdog"]
    pub tx_watchdog: crate::Reg<tx_watchdog::TX_WATCHDOG_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0xa0 - Receiver control"]
    pub rx_ctl: crate::Reg<rx_ctl::RX_CTL_SPEC>,
    #[doc = "0xa4 - Receiver watchdog"]
    pub rx_watchdog: crate::Reg<rx_watchdog::RX_WATCHDOG_SPEC>,
    _reserved8: [u8; 0x0158],
    #[doc = "0x200 - TX FIFO control"]
    pub tx_fifo_ctl: crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>,
    #[doc = "0x204 - TX FIFO status"]
    pub tx_fifo_status: crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>,
    #[doc = "0x208 - TX FIFO write"]
    pub tx_fifo_wr: crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>,
    _reserved11: [u8; 0xf4],
    #[doc = "0x300 - RX FIFO control"]
    pub rx_fifo_ctl: crate::Reg<rx_fifo_ctl::RX_FIFO_CTL_SPEC>,
    #[doc = "0x304 - RX FIFO status"]
    pub rx_fifo_status: crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>,
    #[doc = "0x308 - RX FIFO read"]
    pub rx_fifo_rd: crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>,
    #[doc = "0x30c - RX FIFO silent read"]
    pub rx_fifo_rd_silent: crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>,
    _reserved15: [u8; 0x0bf0],
    #[doc = "0xf00 - Interrupt register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0xf04 - Interrupt set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0xf08 - Interrupt mask register"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0xf0c - Interrupt masked register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CLOCK_CTL register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TR_CTL register accessor: an alias for `Reg<TR_CTL_SPEC>`"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control"]
pub mod tr_ctl;
#[doc = "TX_CTL register accessor: an alias for `Reg<TX_CTL_SPEC>`"]
pub type TX_CTL = crate::Reg<tx_ctl::TX_CTL_SPEC>;
#[doc = "Transmitter control"]
pub mod tx_ctl;
#[doc = "TX_WATCHDOG register accessor: an alias for `Reg<TX_WATCHDOG_SPEC>`"]
pub type TX_WATCHDOG = crate::Reg<tx_watchdog::TX_WATCHDOG_SPEC>;
#[doc = "Transmitter watchdog"]
pub mod tx_watchdog;
#[doc = "RX_CTL register accessor: an alias for `Reg<RX_CTL_SPEC>`"]
pub type RX_CTL = crate::Reg<rx_ctl::RX_CTL_SPEC>;
#[doc = "Receiver control"]
pub mod rx_ctl;
#[doc = "RX_WATCHDOG register accessor: an alias for `Reg<RX_WATCHDOG_SPEC>`"]
pub type RX_WATCHDOG = crate::Reg<rx_watchdog::RX_WATCHDOG_SPEC>;
#[doc = "Receiver watchdog"]
pub mod rx_watchdog;
#[doc = "TX_FIFO_CTL register accessor: an alias for `Reg<TX_FIFO_CTL_SPEC>`"]
pub type TX_FIFO_CTL = crate::Reg<tx_fifo_ctl::TX_FIFO_CTL_SPEC>;
#[doc = "TX FIFO control"]
pub mod tx_fifo_ctl;
#[doc = "TX_FIFO_STATUS register accessor: an alias for `Reg<TX_FIFO_STATUS_SPEC>`"]
pub type TX_FIFO_STATUS = crate::Reg<tx_fifo_status::TX_FIFO_STATUS_SPEC>;
#[doc = "TX FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR register accessor: an alias for `Reg<TX_FIFO_WR_SPEC>`"]
pub type TX_FIFO_WR = crate::Reg<tx_fifo_wr::TX_FIFO_WR_SPEC>;
#[doc = "TX FIFO write"]
pub mod tx_fifo_wr;
#[doc = "RX_FIFO_CTL register accessor: an alias for `Reg<RX_FIFO_CTL_SPEC>`"]
pub type RX_FIFO_CTL = crate::Reg<rx_fifo_ctl::RX_FIFO_CTL_SPEC>;
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX_FIFO_STATUS register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
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
