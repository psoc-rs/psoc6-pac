#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM0+ control"]
    pub cm0_ctl: CM0_CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - CM0+ status"]
    pub cm0_status: CM0_STATUS,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - CM0+ clock control"]
    pub cm0_clock_ctl: CM0_CLOCK_CTL,
    _reserved3: [u8; 12usize],
    #[doc = "0x20 - CM0+ interrupt control 0"]
    pub cm0_int_ctl0: CM0_INT_CTL0,
    #[doc = "0x24 - CM0+ interrupt control 1"]
    pub cm0_int_ctl1: CM0_INT_CTL1,
    #[doc = "0x28 - CM0+ interrupt control 2"]
    pub cm0_int_ctl2: CM0_INT_CTL2,
    #[doc = "0x2c - CM0+ interrupt control 3"]
    pub cm0_int_ctl3: CM0_INT_CTL3,
    #[doc = "0x30 - CM0+ interrupt control 4"]
    pub cm0_int_ctl4: CM0_INT_CTL4,
    #[doc = "0x34 - CM0+ interrupt control 5"]
    pub cm0_int_ctl5: CM0_INT_CTL5,
    #[doc = "0x38 - CM0+ interrupt control 6"]
    pub cm0_int_ctl6: CM0_INT_CTL6,
    #[doc = "0x3c - CM0+ interrupt control 7"]
    pub cm0_int_ctl7: CM0_INT_CTL7,
    _reserved11: [u8; 64usize],
    #[doc = "0x80 - CM4 power control"]
    pub cm4_pwr_ctl: CM4_PWR_CTL,
    #[doc = "0x84 - CM4 power control"]
    pub cm4_pwr_delay_ctl: CM4_PWR_DELAY_CTL,
    #[doc = "0x88 - CM4 status"]
    pub cm4_status: CM4_STATUS,
    _reserved14: [u8; 4usize],
    #[doc = "0x90 - CM4 clock control"]
    pub cm4_clock_ctl: CM4_CLOCK_CTL,
    _reserved15: [u8; 12usize],
    #[doc = "0xa0 - CM4 NMI control"]
    pub cm4_nmi_ctl: CM4_NMI_CTL,
    _reserved16: [u8; 92usize],
    #[doc = "0x100 - RAM 0 control 0"]
    pub ram0_ctl0: RAM0_CTL0,
    _reserved17: [u8; 60usize],
    #[doc = "0x140 - RAM 0 power control"]
    pub ram0_pwr_macro_ctl: [RAM0_PWR_MACRO_CTL; 16],
    #[doc = "0x180 - RAM 1 control 0"]
    pub ram1_ctl0: RAM1_CTL0,
    _reserved19: [u8; 12usize],
    #[doc = "0x190 - RAM1 power control"]
    pub ram1_pwr_ctl: RAM1_PWR_CTL,
    _reserved20: [u8; 12usize],
    #[doc = "0x1a0 - RAM 2 control 0"]
    pub ram2_ctl0: RAM2_CTL0,
    _reserved21: [u8; 12usize],
    #[doc = "0x1b0 - RAM2 power control"]
    pub ram2_pwr_ctl: RAM2_PWR_CTL,
    _reserved22: [u8; 12usize],
    #[doc = "0x1c0 - Power up delay used for all SRAM power domains"]
    pub ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    _reserved23: [u8; 12usize],
    #[doc = "0x1d0 - ROM control"]
    pub rom_ctl: ROM_CTL,
    _reserved24: [u8; 28usize],
    #[doc = "0x1f0 - UDB power control"]
    pub udb_pwr_ctl: UDB_PWR_CTL,
    #[doc = "0x1f4 - UDB power control"]
    pub udb_pwr_delay_ctl: UDB_PWR_DELAY_CTL,
    _reserved26: [u8; 16usize],
    #[doc = "0x208 - Debug port status"]
    pub dp_status: DP_STATUS,
    _reserved27: [u8; 20usize],
    #[doc = "0x220 - Buffer control"]
    pub buff_ctl: BUFF_CTL,
    _reserved28: [u8; 12usize],
    #[doc = "0x230 - DDFT control"]
    pub ddft_ctl: DDFT_CTL,
    _reserved29: [u8; 12usize],
    #[doc = "0x240 - SysTick timer control"]
    pub systick_ctl: SYSTICK_CTL,
    _reserved30: [u8; 108usize],
    #[doc = "0x2b0 - CM0+ vector table base"]
    pub cm0_vector_table_base: CM0_VECTOR_TABLE_BASE,
    _reserved31: [u8; 12usize],
    #[doc = "0x2c0 - CM4 vector table base"]
    pub cm4_vector_table_base: CM4_VECTOR_TABLE_BASE,
    _reserved32: [u8; 92usize],
    #[doc = "0x320 - CM0+ protection context 0 handler"]
    pub cm0_pc0_handler: CM0_PC0_HANDLER,
    _reserved33: [u8; 220usize],
    #[doc = "0x400 - Identity"]
    pub identity: IDENTITY,
    _reserved34: [u8; 252usize],
    #[doc = "0x500 - Protection status"]
    pub protection: PROTECTION,
    _reserved35: [u8; 28usize],
    #[doc = "0x520 - CM0+ NMI control"]
    pub cm0_nmi_ctl: CM0_NMI_CTL,
    _reserved36: [u8; 124usize],
    #[doc = "0x5a0 - Memory BIST status"]
    pub mbist_stat: MBIST_STAT,
    _reserved37: [u8; 59996usize],
    #[doc = "0xf000 - ROM trim control"]
    pub trim_rom_ctl: TRIM_ROM_CTL,
    #[doc = "0xf004 - RAM trim control"]
    pub trim_ram_ctl: TRIM_RAM_CTL,
}
#[doc = "CM0+ control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ctl](cm0_ctl) module"]
pub type CM0_CTL = crate::Reg<u32, _CM0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CTL;
#[doc = "`read()` method returns [cm0_ctl::R](cm0_ctl::R) reader structure"]
impl crate::Readable for CM0_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_ctl::W](cm0_ctl::W) writer structure"]
impl crate::Writable for CM0_CTL {}
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0+ status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_status](cm0_status) module"]
pub type CM0_STATUS = crate::Reg<u32, _CM0_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_STATUS;
#[doc = "`read()` method returns [cm0_status::R](cm0_status::R) reader structure"]
impl crate::Readable for CM0_STATUS {}
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0+ clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_clock_ctl](cm0_clock_ctl) module"]
pub type CM0_CLOCK_CTL = crate::Reg<u32, _CM0_CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_CLOCK_CTL;
#[doc = "`read()` method returns [cm0_clock_ctl::R](cm0_clock_ctl::R) reader structure"]
impl crate::Readable for CM0_CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_clock_ctl::W](cm0_clock_ctl::W) writer structure"]
impl crate::Writable for CM0_CLOCK_CTL {}
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0+ interrupt control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl0](cm0_int_ctl0) module"]
pub type CM0_INT_CTL0 = crate::Reg<u32, _CM0_INT_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL0;
#[doc = "`read()` method returns [cm0_int_ctl0::R](cm0_int_ctl0::R) reader structure"]
impl crate::Readable for CM0_INT_CTL0 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl0::W](cm0_int_ctl0::W) writer structure"]
impl crate::Writable for CM0_INT_CTL0 {}
#[doc = "CM0+ interrupt control 0"]
pub mod cm0_int_ctl0;
#[doc = "CM0+ interrupt control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl1](cm0_int_ctl1) module"]
pub type CM0_INT_CTL1 = crate::Reg<u32, _CM0_INT_CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL1;
#[doc = "`read()` method returns [cm0_int_ctl1::R](cm0_int_ctl1::R) reader structure"]
impl crate::Readable for CM0_INT_CTL1 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl1::W](cm0_int_ctl1::W) writer structure"]
impl crate::Writable for CM0_INT_CTL1 {}
#[doc = "CM0+ interrupt control 1"]
pub mod cm0_int_ctl1;
#[doc = "CM0+ interrupt control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl2](cm0_int_ctl2) module"]
pub type CM0_INT_CTL2 = crate::Reg<u32, _CM0_INT_CTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL2;
#[doc = "`read()` method returns [cm0_int_ctl2::R](cm0_int_ctl2::R) reader structure"]
impl crate::Readable for CM0_INT_CTL2 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl2::W](cm0_int_ctl2::W) writer structure"]
impl crate::Writable for CM0_INT_CTL2 {}
#[doc = "CM0+ interrupt control 2"]
pub mod cm0_int_ctl2;
#[doc = "CM0+ interrupt control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl3](cm0_int_ctl3) module"]
pub type CM0_INT_CTL3 = crate::Reg<u32, _CM0_INT_CTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL3;
#[doc = "`read()` method returns [cm0_int_ctl3::R](cm0_int_ctl3::R) reader structure"]
impl crate::Readable for CM0_INT_CTL3 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl3::W](cm0_int_ctl3::W) writer structure"]
impl crate::Writable for CM0_INT_CTL3 {}
#[doc = "CM0+ interrupt control 3"]
pub mod cm0_int_ctl3;
#[doc = "CM0+ interrupt control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl4](cm0_int_ctl4) module"]
pub type CM0_INT_CTL4 = crate::Reg<u32, _CM0_INT_CTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL4;
#[doc = "`read()` method returns [cm0_int_ctl4::R](cm0_int_ctl4::R) reader structure"]
impl crate::Readable for CM0_INT_CTL4 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl4::W](cm0_int_ctl4::W) writer structure"]
impl crate::Writable for CM0_INT_CTL4 {}
#[doc = "CM0+ interrupt control 4"]
pub mod cm0_int_ctl4;
#[doc = "CM0+ interrupt control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl5](cm0_int_ctl5) module"]
pub type CM0_INT_CTL5 = crate::Reg<u32, _CM0_INT_CTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL5;
#[doc = "`read()` method returns [cm0_int_ctl5::R](cm0_int_ctl5::R) reader structure"]
impl crate::Readable for CM0_INT_CTL5 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl5::W](cm0_int_ctl5::W) writer structure"]
impl crate::Writable for CM0_INT_CTL5 {}
#[doc = "CM0+ interrupt control 5"]
pub mod cm0_int_ctl5;
#[doc = "CM0+ interrupt control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl6](cm0_int_ctl6) module"]
pub type CM0_INT_CTL6 = crate::Reg<u32, _CM0_INT_CTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL6;
#[doc = "`read()` method returns [cm0_int_ctl6::R](cm0_int_ctl6::R) reader structure"]
impl crate::Readable for CM0_INT_CTL6 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl6::W](cm0_int_ctl6::W) writer structure"]
impl crate::Writable for CM0_INT_CTL6 {}
#[doc = "CM0+ interrupt control 6"]
pub mod cm0_int_ctl6;
#[doc = "CM0+ interrupt control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int_ctl7](cm0_int_ctl7) module"]
pub type CM0_INT_CTL7 = crate::Reg<u32, _CM0_INT_CTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_INT_CTL7;
#[doc = "`read()` method returns [cm0_int_ctl7::R](cm0_int_ctl7::R) reader structure"]
impl crate::Readable for CM0_INT_CTL7 {}
#[doc = "`write(|w| ..)` method takes [cm0_int_ctl7::W](cm0_int_ctl7::W) writer structure"]
impl crate::Writable for CM0_INT_CTL7 {}
#[doc = "CM0+ interrupt control 7"]
pub mod cm0_int_ctl7;
#[doc = "CM4 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_pwr_ctl](cm4_pwr_ctl) module"]
pub type CM4_PWR_CTL = crate::Reg<u32, _CM4_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_PWR_CTL;
#[doc = "`read()` method returns [cm4_pwr_ctl::R](cm4_pwr_ctl::R) reader structure"]
impl crate::Readable for CM4_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_pwr_ctl::W](cm4_pwr_ctl::W) writer structure"]
impl crate::Writable for CM4_PWR_CTL {}
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_pwr_delay_ctl](cm4_pwr_delay_ctl) module"]
pub type CM4_PWR_DELAY_CTL = crate::Reg<u32, _CM4_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_PWR_DELAY_CTL;
#[doc = "`read()` method returns [cm4_pwr_delay_ctl::R](cm4_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for CM4_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_pwr_delay_ctl::W](cm4_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for CM4_PWR_DELAY_CTL {}
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "CM4 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_status](cm4_status) module"]
pub type CM4_STATUS = crate::Reg<u32, _CM4_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_STATUS;
#[doc = "`read()` method returns [cm4_status::R](cm4_status::R) reader structure"]
impl crate::Readable for CM4_STATUS {}
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_clock_ctl](cm4_clock_ctl) module"]
pub type CM4_CLOCK_CTL = crate::Reg<u32, _CM4_CLOCK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_CLOCK_CTL;
#[doc = "`read()` method returns [cm4_clock_ctl::R](cm4_clock_ctl::R) reader structure"]
impl crate::Readable for CM4_CLOCK_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_clock_ctl::W](cm4_clock_ctl::W) writer structure"]
impl crate::Writable for CM4_CLOCK_CTL {}
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4 NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_nmi_ctl](cm4_nmi_ctl) module"]
pub type CM4_NMI_CTL = crate::Reg<u32, _CM4_NMI_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_NMI_CTL;
#[doc = "`read()` method returns [cm4_nmi_ctl::R](cm4_nmi_ctl::R) reader structure"]
impl crate::Readable for CM4_NMI_CTL {}
#[doc = "`write(|w| ..)` method takes [cm4_nmi_ctl::W](cm4_nmi_ctl::W) writer structure"]
impl crate::Writable for CM4_NMI_CTL {}
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "RAM 0 control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_ctl0](ram0_ctl0) module"]
pub type RAM0_CTL0 = crate::Reg<u32, _RAM0_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0_CTL0;
#[doc = "`read()` method returns [ram0_ctl0::R](ram0_ctl0::R) reader structure"]
impl crate::Readable for RAM0_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram0_ctl0::W](ram0_ctl0::W) writer structure"]
impl crate::Writable for RAM0_CTL0 {}
#[doc = "RAM 0 control 0"]
pub mod ram0_ctl0;
#[doc = "RAM 0 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_pwr_macro_ctl](ram0_pwr_macro_ctl) module"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<u32, _RAM0_PWR_MACRO_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0_PWR_MACRO_CTL;
#[doc = "`read()` method returns [ram0_pwr_macro_ctl::R](ram0_pwr_macro_ctl::R) reader structure"]
impl crate::Readable for RAM0_PWR_MACRO_CTL {}
#[doc = "`write(|w| ..)` method takes [ram0_pwr_macro_ctl::W](ram0_pwr_macro_ctl::W) writer structure"]
impl crate::Writable for RAM0_PWR_MACRO_CTL {}
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM 1 control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_ctl0](ram1_ctl0) module"]
pub type RAM1_CTL0 = crate::Reg<u32, _RAM1_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1_CTL0;
#[doc = "`read()` method returns [ram1_ctl0::R](ram1_ctl0::R) reader structure"]
impl crate::Readable for RAM1_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram1_ctl0::W](ram1_ctl0::W) writer structure"]
impl crate::Writable for RAM1_CTL0 {}
#[doc = "RAM 1 control 0"]
pub mod ram1_ctl0;
#[doc = "RAM1 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_pwr_ctl](ram1_pwr_ctl) module"]
pub type RAM1_PWR_CTL = crate::Reg<u32, _RAM1_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1_PWR_CTL;
#[doc = "`read()` method returns [ram1_pwr_ctl::R](ram1_pwr_ctl::R) reader structure"]
impl crate::Readable for RAM1_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [ram1_pwr_ctl::W](ram1_pwr_ctl::W) writer structure"]
impl crate::Writable for RAM1_PWR_CTL {}
#[doc = "RAM1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM 2 control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_ctl0](ram2_ctl0) module"]
pub type RAM2_CTL0 = crate::Reg<u32, _RAM2_CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2_CTL0;
#[doc = "`read()` method returns [ram2_ctl0::R](ram2_ctl0::R) reader structure"]
impl crate::Readable for RAM2_CTL0 {}
#[doc = "`write(|w| ..)` method takes [ram2_ctl0::W](ram2_ctl0::W) writer structure"]
impl crate::Writable for RAM2_CTL0 {}
#[doc = "RAM 2 control 0"]
pub mod ram2_ctl0;
#[doc = "RAM2 power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_pwr_ctl](ram2_pwr_ctl) module"]
pub type RAM2_PWR_CTL = crate::Reg<u32, _RAM2_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2_PWR_CTL;
#[doc = "`read()` method returns [ram2_pwr_ctl::R](ram2_pwr_ctl::R) reader structure"]
impl crate::Readable for RAM2_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [ram2_pwr_ctl::W](ram2_pwr_ctl::W) writer structure"]
impl crate::Writable for RAM2_PWR_CTL {}
#[doc = "RAM2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "Power up delay used for all SRAM power domains\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_pwr_delay_ctl](ram_pwr_delay_ctl) module"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<u32, _RAM_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_PWR_DELAY_CTL;
#[doc = "`read()` method returns [ram_pwr_delay_ctl::R](ram_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for RAM_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [ram_pwr_delay_ctl::W](ram_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for RAM_PWR_DELAY_CTL {}
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctl](rom_ctl) module"]
pub type ROM_CTL = crate::Reg<u32, _ROM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_CTL;
#[doc = "`read()` method returns [rom_ctl::R](rom_ctl::R) reader structure"]
impl crate::Readable for ROM_CTL {}
#[doc = "`write(|w| ..)` method takes [rom_ctl::W](rom_ctl::W) writer structure"]
impl crate::Writable for ROM_CTL {}
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "UDB power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udb_pwr_ctl](udb_pwr_ctl) module"]
pub type UDB_PWR_CTL = crate::Reg<u32, _UDB_PWR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDB_PWR_CTL;
#[doc = "`read()` method returns [udb_pwr_ctl::R](udb_pwr_ctl::R) reader structure"]
impl crate::Readable for UDB_PWR_CTL {}
#[doc = "`write(|w| ..)` method takes [udb_pwr_ctl::W](udb_pwr_ctl::W) writer structure"]
impl crate::Writable for UDB_PWR_CTL {}
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udb_pwr_delay_ctl](udb_pwr_delay_ctl) module"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<u32, _UDB_PWR_DELAY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDB_PWR_DELAY_CTL;
#[doc = "`read()` method returns [udb_pwr_delay_ctl::R](udb_pwr_delay_ctl::R) reader structure"]
impl crate::Readable for UDB_PWR_DELAY_CTL {}
#[doc = "`write(|w| ..)` method takes [udb_pwr_delay_ctl::W](udb_pwr_delay_ctl::W) writer structure"]
impl crate::Writable for UDB_PWR_DELAY_CTL {}
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "Debug port status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dp_status](dp_status) module"]
pub type DP_STATUS = crate::Reg<u32, _DP_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DP_STATUS;
#[doc = "`read()` method returns [dp_status::R](dp_status::R) reader structure"]
impl crate::Readable for DP_STATUS {}
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "Buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buff_ctl](buff_ctl) module"]
pub type BUFF_CTL = crate::Reg<u32, _BUFF_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFF_CTL;
#[doc = "`read()` method returns [buff_ctl::R](buff_ctl::R) reader structure"]
impl crate::Readable for BUFF_CTL {}
#[doc = "`write(|w| ..)` method takes [buff_ctl::W](buff_ctl::W) writer structure"]
impl crate::Writable for BUFF_CTL {}
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "DDFT control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddft_ctl](ddft_ctl) module"]
pub type DDFT_CTL = crate::Reg<u32, _DDFT_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDFT_CTL;
#[doc = "`read()` method returns [ddft_ctl::R](ddft_ctl::R) reader structure"]
impl crate::Readable for DDFT_CTL {}
#[doc = "`write(|w| ..)` method takes [ddft_ctl::W](ddft_ctl::W) writer structure"]
impl crate::Writable for DDFT_CTL {}
#[doc = "DDFT control"]
pub mod ddft_ctl;
#[doc = "SysTick timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systick_ctl](systick_ctl) module"]
pub type SYSTICK_CTL = crate::Reg<u32, _SYSTICK_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICK_CTL;
#[doc = "`read()` method returns [systick_ctl::R](systick_ctl::R) reader structure"]
impl crate::Readable for SYSTICK_CTL {}
#[doc = "`write(|w| ..)` method takes [systick_ctl::W](systick_ctl::W) writer structure"]
impl crate::Writable for SYSTICK_CTL {}
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "CM0+ vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_vector_table_base](cm0_vector_table_base) module"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<u32, _CM0_VECTOR_TABLE_BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_VECTOR_TABLE_BASE;
#[doc = "`read()` method returns [cm0_vector_table_base::R](cm0_vector_table_base::R) reader structure"]
impl crate::Readable for CM0_VECTOR_TABLE_BASE {}
#[doc = "`write(|w| ..)` method takes [cm0_vector_table_base::W](cm0_vector_table_base::W) writer structure"]
impl crate::Writable for CM0_VECTOR_TABLE_BASE {}
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM4 vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_vector_table_base](cm4_vector_table_base) module"]
pub type CM4_VECTOR_TABLE_BASE = crate::Reg<u32, _CM4_VECTOR_TABLE_BASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM4_VECTOR_TABLE_BASE;
#[doc = "`read()` method returns [cm4_vector_table_base::R](cm4_vector_table_base::R) reader structure"]
impl crate::Readable for CM4_VECTOR_TABLE_BASE {}
#[doc = "`write(|w| ..)` method takes [cm4_vector_table_base::W](cm4_vector_table_base::W) writer structure"]
impl crate::Writable for CM4_VECTOR_TABLE_BASE {}
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM0+ protection context 0 handler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_pc0_handler](cm0_pc0_handler) module"]
pub type CM0_PC0_HANDLER = crate::Reg<u32, _CM0_PC0_HANDLER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_PC0_HANDLER;
#[doc = "`read()` method returns [cm0_pc0_handler::R](cm0_pc0_handler::R) reader structure"]
impl crate::Readable for CM0_PC0_HANDLER {}
#[doc = "`write(|w| ..)` method takes [cm0_pc0_handler::W](cm0_pc0_handler::W) writer structure"]
impl crate::Writable for CM0_PC0_HANDLER {}
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "Identity\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [identity](identity) module"]
pub type IDENTITY = crate::Reg<u32, _IDENTITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDENTITY;
#[doc = "`read()` method returns [identity::R](identity::R) reader structure"]
impl crate::Readable for IDENTITY {}
#[doc = "Identity"]
pub mod identity;
#[doc = "Protection status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](protection) module"]
pub type PROTECTION = crate::Reg<u32, _PROTECTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROTECTION;
#[doc = "`read()` method returns [protection::R](protection::R) reader structure"]
impl crate::Readable for PROTECTION {}
#[doc = "`write(|w| ..)` method takes [protection::W](protection::W) writer structure"]
impl crate::Writable for PROTECTION {}
#[doc = "Protection status"]
pub mod protection;
#[doc = "CM0+ NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_nmi_ctl](cm0_nmi_ctl) module"]
pub type CM0_NMI_CTL = crate::Reg<u32, _CM0_NMI_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM0_NMI_CTL;
#[doc = "`read()` method returns [cm0_nmi_ctl::R](cm0_nmi_ctl::R) reader structure"]
impl crate::Readable for CM0_NMI_CTL {}
#[doc = "`write(|w| ..)` method takes [cm0_nmi_ctl::W](cm0_nmi_ctl::W) writer structure"]
impl crate::Writable for CM0_NMI_CTL {}
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "Memory BIST status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](mbist_stat) module"]
pub type MBIST_STAT = crate::Reg<u32, _MBIST_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBIST_STAT;
#[doc = "`read()` method returns [mbist_stat::R](mbist_stat::R) reader structure"]
impl crate::Readable for MBIST_STAT {}
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "ROM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_rom_ctl](trim_rom_ctl) module"]
pub type TRIM_ROM_CTL = crate::Reg<u32, _TRIM_ROM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_ROM_CTL;
#[doc = "`read()` method returns [trim_rom_ctl::R](trim_rom_ctl::R) reader structure"]
impl crate::Readable for TRIM_ROM_CTL {}
#[doc = "`write(|w| ..)` method takes [trim_rom_ctl::W](trim_rom_ctl::W) writer structure"]
impl crate::Writable for TRIM_ROM_CTL {}
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "RAM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ram_ctl](trim_ram_ctl) module"]
pub type TRIM_RAM_CTL = crate::Reg<u32, _TRIM_RAM_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM_RAM_CTL;
#[doc = "`read()` method returns [trim_ram_ctl::R](trim_ram_ctl::R) reader structure"]
impl crate::Readable for TRIM_RAM_CTL {}
#[doc = "`write(|w| ..)` method takes [trim_ram_ctl::W](trim_ram_ctl::W) writer structure"]
impl crate::Writable for TRIM_RAM_CTL {}
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
