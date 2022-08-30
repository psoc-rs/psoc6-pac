#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM0+ control"]
    pub cm0_ctl: crate::Reg<cm0_ctl::CM0_CTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - CM0+ status"]
    pub cm0_status: crate::Reg<cm0_status::CM0_STATUS_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - CM0+ clock control"]
    pub cm0_clock_ctl: crate::Reg<cm0_clock_ctl::CM0_CLOCK_CTL_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x20 - CM0+ interrupt control 0"]
    pub cm0_int_ctl0: crate::Reg<cm0_int_ctl0::CM0_INT_CTL0_SPEC>,
    #[doc = "0x24 - CM0+ interrupt control 1"]
    pub cm0_int_ctl1: crate::Reg<cm0_int_ctl1::CM0_INT_CTL1_SPEC>,
    #[doc = "0x28 - CM0+ interrupt control 2"]
    pub cm0_int_ctl2: crate::Reg<cm0_int_ctl2::CM0_INT_CTL2_SPEC>,
    #[doc = "0x2c - CM0+ interrupt control 3"]
    pub cm0_int_ctl3: crate::Reg<cm0_int_ctl3::CM0_INT_CTL3_SPEC>,
    #[doc = "0x30 - CM0+ interrupt control 4"]
    pub cm0_int_ctl4: crate::Reg<cm0_int_ctl4::CM0_INT_CTL4_SPEC>,
    #[doc = "0x34 - CM0+ interrupt control 5"]
    pub cm0_int_ctl5: crate::Reg<cm0_int_ctl5::CM0_INT_CTL5_SPEC>,
    #[doc = "0x38 - CM0+ interrupt control 6"]
    pub cm0_int_ctl6: crate::Reg<cm0_int_ctl6::CM0_INT_CTL6_SPEC>,
    #[doc = "0x3c - CM0+ interrupt control 7"]
    pub cm0_int_ctl7: crate::Reg<cm0_int_ctl7::CM0_INT_CTL7_SPEC>,
    _reserved11: [u8; 0x40],
    #[doc = "0x80 - CM4 power control"]
    pub cm4_pwr_ctl: crate::Reg<cm4_pwr_ctl::CM4_PWR_CTL_SPEC>,
    #[doc = "0x84 - CM4 power control"]
    pub cm4_pwr_delay_ctl: crate::Reg<cm4_pwr_delay_ctl::CM4_PWR_DELAY_CTL_SPEC>,
    #[doc = "0x88 - CM4 status"]
    pub cm4_status: crate::Reg<cm4_status::CM4_STATUS_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - CM4 clock control"]
    pub cm4_clock_ctl: crate::Reg<cm4_clock_ctl::CM4_CLOCK_CTL_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0xa0 - CM4 NMI control"]
    pub cm4_nmi_ctl: crate::Reg<cm4_nmi_ctl::CM4_NMI_CTL_SPEC>,
    _reserved16: [u8; 0x5c],
    #[doc = "0x100 - RAM 0 control 0"]
    pub ram0_ctl0: crate::Reg<ram0_ctl0::RAM0_CTL0_SPEC>,
    _reserved17: [u8; 0x3c],
    #[doc = "0x140..0x180 - RAM 0 power control"]
    pub ram0_pwr_macro_ctl: [crate::Reg<ram0_pwr_macro_ctl::RAM0_PWR_MACRO_CTL_SPEC>; 16],
    #[doc = "0x180 - RAM 1 control 0"]
    pub ram1_ctl0: crate::Reg<ram1_ctl0::RAM1_CTL0_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x190 - RAM1 power control"]
    pub ram1_pwr_ctl: crate::Reg<ram1_pwr_ctl::RAM1_PWR_CTL_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x1a0 - RAM 2 control 0"]
    pub ram2_ctl0: crate::Reg<ram2_ctl0::RAM2_CTL0_SPEC>,
    _reserved21: [u8; 0x0c],
    #[doc = "0x1b0 - RAM2 power control"]
    pub ram2_pwr_ctl: crate::Reg<ram2_pwr_ctl::RAM2_PWR_CTL_SPEC>,
    _reserved22: [u8; 0x0c],
    #[doc = "0x1c0 - Power up delay used for all SRAM power domains"]
    pub ram_pwr_delay_ctl: crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x1d0 - ROM control"]
    pub rom_ctl: crate::Reg<rom_ctl::ROM_CTL_SPEC>,
    _reserved24: [u8; 0x1c],
    #[doc = "0x1f0 - UDB power control"]
    pub udb_pwr_ctl: crate::Reg<udb_pwr_ctl::UDB_PWR_CTL_SPEC>,
    #[doc = "0x1f4 - UDB power control"]
    pub udb_pwr_delay_ctl: crate::Reg<udb_pwr_delay_ctl::UDB_PWR_DELAY_CTL_SPEC>,
    _reserved26: [u8; 0x10],
    #[doc = "0x208 - Debug port status"]
    pub dp_status: crate::Reg<dp_status::DP_STATUS_SPEC>,
    _reserved27: [u8; 0x14],
    #[doc = "0x220 - Buffer control"]
    pub buff_ctl: crate::Reg<buff_ctl::BUFF_CTL_SPEC>,
    _reserved28: [u8; 0x0c],
    #[doc = "0x230 - DDFT control"]
    pub ddft_ctl: crate::Reg<ddft_ctl::DDFT_CTL_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x240 - SysTick timer control"]
    pub systick_ctl: crate::Reg<systick_ctl::SYSTICK_CTL_SPEC>,
    _reserved30: [u8; 0x6c],
    #[doc = "0x2b0 - CM0+ vector table base"]
    pub cm0_vector_table_base: crate::Reg<cm0_vector_table_base::CM0_VECTOR_TABLE_BASE_SPEC>,
    _reserved31: [u8; 0x0c],
    #[doc = "0x2c0 - CM4 vector table base"]
    pub cm4_vector_table_base: crate::Reg<cm4_vector_table_base::CM4_VECTOR_TABLE_BASE_SPEC>,
    _reserved32: [u8; 0x5c],
    #[doc = "0x320 - CM0+ protection context 0 handler"]
    pub cm0_pc0_handler: crate::Reg<cm0_pc0_handler::CM0_PC0_HANDLER_SPEC>,
    _reserved33: [u8; 0xdc],
    #[doc = "0x400 - Identity"]
    pub identity: crate::Reg<identity::IDENTITY_SPEC>,
    _reserved34: [u8; 0xfc],
    #[doc = "0x500 - Protection status"]
    pub protection: crate::Reg<protection::PROTECTION_SPEC>,
    _reserved35: [u8; 0x1c],
    #[doc = "0x520 - CM0+ NMI control"]
    pub cm0_nmi_ctl: crate::Reg<cm0_nmi_ctl::CM0_NMI_CTL_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0x540 - Access port control"]
    pub ap_ctl: crate::Reg<ap_ctl::AP_CTL_SPEC>,
    _reserved37: [u8; 0x5c],
    #[doc = "0x5a0 - Memory BIST status"]
    pub mbist_stat: crate::Reg<mbist_stat::MBIST_STAT_SPEC>,
    _reserved38: [u8; 0xea5c],
    #[doc = "0xf000 - ROM trim control"]
    pub trim_rom_ctl: crate::Reg<trim_rom_ctl::TRIM_ROM_CTL_SPEC>,
    #[doc = "0xf004 - RAM trim control"]
    pub trim_ram_ctl: crate::Reg<trim_ram_ctl::TRIM_RAM_CTL_SPEC>,
}
#[doc = "CM0_CTL register accessor: an alias for `Reg<CM0_CTL_SPEC>`"]
pub type CM0_CTL = crate::Reg<cm0_ctl::CM0_CTL_SPEC>;
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0_STATUS register accessor: an alias for `Reg<CM0_STATUS_SPEC>`"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0_CLOCK_CTL register accessor: an alias for `Reg<CM0_CLOCK_CTL_SPEC>`"]
pub type CM0_CLOCK_CTL = crate::Reg<cm0_clock_ctl::CM0_CLOCK_CTL_SPEC>;
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0_INT_CTL0 register accessor: an alias for `Reg<CM0_INT_CTL0_SPEC>`"]
pub type CM0_INT_CTL0 = crate::Reg<cm0_int_ctl0::CM0_INT_CTL0_SPEC>;
#[doc = "CM0+ interrupt control 0"]
pub mod cm0_int_ctl0;
#[doc = "CM0_INT_CTL1 register accessor: an alias for `Reg<CM0_INT_CTL1_SPEC>`"]
pub type CM0_INT_CTL1 = crate::Reg<cm0_int_ctl1::CM0_INT_CTL1_SPEC>;
#[doc = "CM0+ interrupt control 1"]
pub mod cm0_int_ctl1;
#[doc = "CM0_INT_CTL2 register accessor: an alias for `Reg<CM0_INT_CTL2_SPEC>`"]
pub type CM0_INT_CTL2 = crate::Reg<cm0_int_ctl2::CM0_INT_CTL2_SPEC>;
#[doc = "CM0+ interrupt control 2"]
pub mod cm0_int_ctl2;
#[doc = "CM0_INT_CTL3 register accessor: an alias for `Reg<CM0_INT_CTL3_SPEC>`"]
pub type CM0_INT_CTL3 = crate::Reg<cm0_int_ctl3::CM0_INT_CTL3_SPEC>;
#[doc = "CM0+ interrupt control 3"]
pub mod cm0_int_ctl3;
#[doc = "CM0_INT_CTL4 register accessor: an alias for `Reg<CM0_INT_CTL4_SPEC>`"]
pub type CM0_INT_CTL4 = crate::Reg<cm0_int_ctl4::CM0_INT_CTL4_SPEC>;
#[doc = "CM0+ interrupt control 4"]
pub mod cm0_int_ctl4;
#[doc = "CM0_INT_CTL5 register accessor: an alias for `Reg<CM0_INT_CTL5_SPEC>`"]
pub type CM0_INT_CTL5 = crate::Reg<cm0_int_ctl5::CM0_INT_CTL5_SPEC>;
#[doc = "CM0+ interrupt control 5"]
pub mod cm0_int_ctl5;
#[doc = "CM0_INT_CTL6 register accessor: an alias for `Reg<CM0_INT_CTL6_SPEC>`"]
pub type CM0_INT_CTL6 = crate::Reg<cm0_int_ctl6::CM0_INT_CTL6_SPEC>;
#[doc = "CM0+ interrupt control 6"]
pub mod cm0_int_ctl6;
#[doc = "CM0_INT_CTL7 register accessor: an alias for `Reg<CM0_INT_CTL7_SPEC>`"]
pub type CM0_INT_CTL7 = crate::Reg<cm0_int_ctl7::CM0_INT_CTL7_SPEC>;
#[doc = "CM0+ interrupt control 7"]
pub mod cm0_int_ctl7;
#[doc = "CM4_PWR_CTL register accessor: an alias for `Reg<CM4_PWR_CTL_SPEC>`"]
pub type CM4_PWR_CTL = crate::Reg<cm4_pwr_ctl::CM4_PWR_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4_PWR_DELAY_CTL register accessor: an alias for `Reg<CM4_PWR_DELAY_CTL_SPEC>`"]
pub type CM4_PWR_DELAY_CTL = crate::Reg<cm4_pwr_delay_ctl::CM4_PWR_DELAY_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "CM4_STATUS register accessor: an alias for `Reg<CM4_STATUS_SPEC>`"]
pub type CM4_STATUS = crate::Reg<cm4_status::CM4_STATUS_SPEC>;
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4_CLOCK_CTL register accessor: an alias for `Reg<CM4_CLOCK_CTL_SPEC>`"]
pub type CM4_CLOCK_CTL = crate::Reg<cm4_clock_ctl::CM4_CLOCK_CTL_SPEC>;
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4_NMI_CTL register accessor: an alias for `Reg<CM4_NMI_CTL_SPEC>`"]
pub type CM4_NMI_CTL = crate::Reg<cm4_nmi_ctl::CM4_NMI_CTL_SPEC>;
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "RAM0_CTL0 register accessor: an alias for `Reg<RAM0_CTL0_SPEC>`"]
pub type RAM0_CTL0 = crate::Reg<ram0_ctl0::RAM0_CTL0_SPEC>;
#[doc = "RAM 0 control 0"]
pub mod ram0_ctl0;
#[doc = "RAM0_PWR_MACRO_CTL register accessor: an alias for `Reg<RAM0_PWR_MACRO_CTL_SPEC>`"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<ram0_pwr_macro_ctl::RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM1_CTL0 register accessor: an alias for `Reg<RAM1_CTL0_SPEC>`"]
pub type RAM1_CTL0 = crate::Reg<ram1_ctl0::RAM1_CTL0_SPEC>;
#[doc = "RAM 1 control 0"]
pub mod ram1_ctl0;
#[doc = "RAM1_PWR_CTL register accessor: an alias for `Reg<RAM1_PWR_CTL_SPEC>`"]
pub type RAM1_PWR_CTL = crate::Reg<ram1_pwr_ctl::RAM1_PWR_CTL_SPEC>;
#[doc = "RAM1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM2_CTL0 register accessor: an alias for `Reg<RAM2_CTL0_SPEC>`"]
pub type RAM2_CTL0 = crate::Reg<ram2_ctl0::RAM2_CTL0_SPEC>;
#[doc = "RAM 2 control 0"]
pub mod ram2_ctl0;
#[doc = "RAM2_PWR_CTL register accessor: an alias for `Reg<RAM2_PWR_CTL_SPEC>`"]
pub type RAM2_PWR_CTL = crate::Reg<ram2_pwr_ctl::RAM2_PWR_CTL_SPEC>;
#[doc = "RAM2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL register accessor: an alias for `Reg<RAM_PWR_DELAY_CTL_SPEC>`"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>;
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM_CTL register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "UDB_PWR_CTL register accessor: an alias for `Reg<UDB_PWR_CTL_SPEC>`"]
pub type UDB_PWR_CTL = crate::Reg<udb_pwr_ctl::UDB_PWR_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB_PWR_DELAY_CTL register accessor: an alias for `Reg<UDB_PWR_DELAY_CTL_SPEC>`"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<udb_pwr_delay_ctl::UDB_PWR_DELAY_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "DP_STATUS register accessor: an alias for `Reg<DP_STATUS_SPEC>`"]
pub type DP_STATUS = crate::Reg<dp_status::DP_STATUS_SPEC>;
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "BUFF_CTL register accessor: an alias for `Reg<BUFF_CTL_SPEC>`"]
pub type BUFF_CTL = crate::Reg<buff_ctl::BUFF_CTL_SPEC>;
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "DDFT_CTL register accessor: an alias for `Reg<DDFT_CTL_SPEC>`"]
pub type DDFT_CTL = crate::Reg<ddft_ctl::DDFT_CTL_SPEC>;
#[doc = "DDFT control"]
pub mod ddft_ctl;
#[doc = "SYSTICK_CTL register accessor: an alias for `Reg<SYSTICK_CTL_SPEC>`"]
pub type SYSTICK_CTL = crate::Reg<systick_ctl::SYSTICK_CTL_SPEC>;
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "CM0_VECTOR_TABLE_BASE register accessor: an alias for `Reg<CM0_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<cm0_vector_table_base::CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM4_VECTOR_TABLE_BASE register accessor: an alias for `Reg<CM4_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM4_VECTOR_TABLE_BASE = crate::Reg<cm4_vector_table_base::CM4_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM0_PC0_HANDLER register accessor: an alias for `Reg<CM0_PC0_HANDLER_SPEC>`"]
pub type CM0_PC0_HANDLER = crate::Reg<cm0_pc0_handler::CM0_PC0_HANDLER_SPEC>;
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "IDENTITY register accessor: an alias for `Reg<IDENTITY_SPEC>`"]
pub type IDENTITY = crate::Reg<identity::IDENTITY_SPEC>;
#[doc = "Identity"]
pub mod identity;
#[doc = "PROTECTION register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection status"]
pub mod protection;
#[doc = "CM0_NMI_CTL register accessor: an alias for `Reg<CM0_NMI_CTL_SPEC>`"]
pub type CM0_NMI_CTL = crate::Reg<cm0_nmi_ctl::CM0_NMI_CTL_SPEC>;
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "AP_CTL register accessor: an alias for `Reg<AP_CTL_SPEC>`"]
pub type AP_CTL = crate::Reg<ap_ctl::AP_CTL_SPEC>;
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "MBIST_STAT register accessor: an alias for `Reg<MBIST_STAT_SPEC>`"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "TRIM_ROM_CTL register accessor: an alias for `Reg<TRIM_ROM_CTL_SPEC>`"]
pub type TRIM_ROM_CTL = crate::Reg<trim_rom_ctl::TRIM_ROM_CTL_SPEC>;
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "TRIM_RAM_CTL register accessor: an alias for `Reg<TRIM_RAM_CTL_SPEC>`"]
pub type TRIM_RAM_CTL = crate::Reg<trim_ram_ctl::TRIM_RAM_CTL_SPEC>;
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
