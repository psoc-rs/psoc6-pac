#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x13e4 - USB Device"]
    pub usbdev: USBDEV,
    _reserved1: [u8; 0x0c1c],
    #[doc = "0x2000..0x2074 - USB Device LPM and PHY Test"]
    pub usblpm: USBLPM,
    _reserved2: [u8; 0x1f8c],
    #[doc = "0x4000..0x4b34 - USB Host Controller"]
    pub usbhost: USBHOST,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBDEV {
    #[doc = "0x00..0x20 - Control End point EP0 Data Register"]
    pub ep0_dr: [crate::Reg<self::usbdev::ep0_dr::EP0_DR_SPEC>; 8],
    #[doc = "0x20 - USB control 0 Register"]
    pub cr0: crate::Reg<self::usbdev::cr0::CR0_SPEC>,
    #[doc = "0x24 - USB control 1 Register"]
    pub cr1: crate::Reg<self::usbdev::cr1::CR1_SPEC>,
    #[doc = "0x28 - USB SIE Data Endpoints Interrupt Enable Register"]
    pub sie_ep_int_en: crate::Reg<self::usbdev::sie_ep_int_en::SIE_EP_INT_EN_SPEC>,
    #[doc = "0x2c - USB SIE Data Endpoint Interrupt Status"]
    pub sie_ep_int_sr: crate::Reg<self::usbdev::sie_ep_int_sr::SIE_EP_INT_SR_SPEC>,
    #[doc = "0x30 - Non-control endpoint count register"]
    pub sie_ep1_cnt0: crate::Reg<self::usbdev::sie_ep1_cnt0::SIE_EP1_CNT0_SPEC>,
    #[doc = "0x34 - Non-control endpoint count register"]
    pub sie_ep1_cnt1: crate::Reg<self::usbdev::sie_ep1_cnt1::SIE_EP1_CNT1_SPEC>,
    #[doc = "0x38 - Non-control endpoint's control Register"]
    pub sie_ep1_cr0: crate::Reg<self::usbdev::sie_ep1_cr0::SIE_EP1_CR0_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - USBIO Control 0 Register"]
    pub usbio_cr0: crate::Reg<self::usbdev::usbio_cr0::USBIO_CR0_SPEC>,
    #[doc = "0x44 - USBIO control 2 Register"]
    pub usbio_cr2: crate::Reg<self::usbdev::usbio_cr2::USBIO_CR2_SPEC>,
    #[doc = "0x48 - USBIO control 1 Register"]
    pub usbio_cr1: crate::Reg<self::usbdev::usbio_cr1::USBIO_CR1_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x50 - USB Dynamic reconfiguration register"]
    pub dyn_reconfig: crate::Reg<self::usbdev::dyn_reconfig::DYN_RECONFIG_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x60 - Start Of Frame Register"]
    pub sof0: crate::Reg<self::usbdev::sof0::SOF0_SPEC>,
    #[doc = "0x64 - Start Of Frame Register"]
    pub sof1: crate::Reg<self::usbdev::sof1::SOF1_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x70 - Non-control endpoint count register"]
    pub sie_ep2_cnt0: crate::Reg<self::usbdev::sie_ep2_cnt0::SIE_EP2_CNT0_SPEC>,
    #[doc = "0x74 - Non-control endpoint count register"]
    pub sie_ep2_cnt1: crate::Reg<self::usbdev::sie_ep2_cnt1::SIE_EP2_CNT1_SPEC>,
    #[doc = "0x78 - Non-control endpoint's control Register"]
    pub sie_ep2_cr0: crate::Reg<self::usbdev::sie_ep2_cr0::SIE_EP2_CR0_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x80 - Oscillator lock data register 0"]
    pub osclk_dr0: crate::Reg<self::usbdev::osclk_dr0::OSCLK_DR0_SPEC>,
    #[doc = "0x84 - Oscillator lock data register 1"]
    pub osclk_dr1: crate::Reg<self::usbdev::osclk_dr1::OSCLK_DR1_SPEC>,
    _reserved19: [u8; 0x18],
    #[doc = "0xa0 - Endpoint0 control Register"]
    pub ep0_cr: crate::Reg<self::usbdev::ep0_cr::EP0_CR_SPEC>,
    #[doc = "0xa4 - Endpoint0 count Register"]
    pub ep0_cnt: crate::Reg<self::usbdev::ep0_cnt::EP0_CNT_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0xb0 - Non-control endpoint count register"]
    pub sie_ep3_cnt0: crate::Reg<self::usbdev::sie_ep3_cnt0::SIE_EP3_CNT0_SPEC>,
    #[doc = "0xb4 - Non-control endpoint count register"]
    pub sie_ep3_cnt1: crate::Reg<self::usbdev::sie_ep3_cnt1::SIE_EP3_CNT1_SPEC>,
    #[doc = "0xb8 - Non-control endpoint's control Register"]
    pub sie_ep3_cr0: crate::Reg<self::usbdev::sie_ep3_cr0::SIE_EP3_CR0_SPEC>,
    _reserved24: [u8; 0x34],
    #[doc = "0xf0 - Non-control endpoint count register"]
    pub sie_ep4_cnt0: crate::Reg<self::usbdev::sie_ep4_cnt0::SIE_EP4_CNT0_SPEC>,
    #[doc = "0xf4 - Non-control endpoint count register"]
    pub sie_ep4_cnt1: crate::Reg<self::usbdev::sie_ep4_cnt1::SIE_EP4_CNT1_SPEC>,
    #[doc = "0xf8 - Non-control endpoint's control Register"]
    pub sie_ep4_cr0: crate::Reg<self::usbdev::sie_ep4_cr0::SIE_EP4_CR0_SPEC>,
    _reserved27: [u8; 0x34],
    #[doc = "0x130 - Non-control endpoint count register"]
    pub sie_ep5_cnt0: crate::Reg<self::usbdev::sie_ep5_cnt0::SIE_EP5_CNT0_SPEC>,
    #[doc = "0x134 - Non-control endpoint count register"]
    pub sie_ep5_cnt1: crate::Reg<self::usbdev::sie_ep5_cnt1::SIE_EP5_CNT1_SPEC>,
    #[doc = "0x138 - Non-control endpoint's control Register"]
    pub sie_ep5_cr0: crate::Reg<self::usbdev::sie_ep5_cr0::SIE_EP5_CR0_SPEC>,
    _reserved30: [u8; 0x34],
    #[doc = "0x170 - Non-control endpoint count register"]
    pub sie_ep6_cnt0: crate::Reg<self::usbdev::sie_ep6_cnt0::SIE_EP6_CNT0_SPEC>,
    #[doc = "0x174 - Non-control endpoint count register"]
    pub sie_ep6_cnt1: crate::Reg<self::usbdev::sie_ep6_cnt1::SIE_EP6_CNT1_SPEC>,
    #[doc = "0x178 - Non-control endpoint's control Register"]
    pub sie_ep6_cr0: crate::Reg<self::usbdev::sie_ep6_cr0::SIE_EP6_CR0_SPEC>,
    _reserved33: [u8; 0x34],
    #[doc = "0x1b0 - Non-control endpoint count register"]
    pub sie_ep7_cnt0: crate::Reg<self::usbdev::sie_ep7_cnt0::SIE_EP7_CNT0_SPEC>,
    #[doc = "0x1b4 - Non-control endpoint count register"]
    pub sie_ep7_cnt1: crate::Reg<self::usbdev::sie_ep7_cnt1::SIE_EP7_CNT1_SPEC>,
    #[doc = "0x1b8 - Non-control endpoint's control Register"]
    pub sie_ep7_cr0: crate::Reg<self::usbdev::sie_ep7_cr0::SIE_EP7_CR0_SPEC>,
    _reserved36: [u8; 0x34],
    #[doc = "0x1f0 - Non-control endpoint count register"]
    pub sie_ep8_cnt0: crate::Reg<self::usbdev::sie_ep8_cnt0::SIE_EP8_CNT0_SPEC>,
    #[doc = "0x1f4 - Non-control endpoint count register"]
    pub sie_ep8_cnt1: crate::Reg<self::usbdev::sie_ep8_cnt1::SIE_EP8_CNT1_SPEC>,
    #[doc = "0x1f8 - Non-control endpoint's control Register"]
    pub sie_ep8_cr0: crate::Reg<self::usbdev::sie_ep8_cr0::SIE_EP8_CR0_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0x200 - Endpoint Configuration Register *1"]
    pub arb_ep1_cfg: crate::Reg<self::usbdev::arb_ep1_cfg::ARB_EP1_CFG_SPEC>,
    #[doc = "0x204 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_int_en: crate::Reg<self::usbdev::arb_ep1_int_en::ARB_EP1_INT_EN_SPEC>,
    #[doc = "0x208 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_sr: crate::Reg<self::usbdev::arb_ep1_sr::ARB_EP1_SR_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x210 - Endpoint Write Address value *1"]
    pub arb_rw1_wa: crate::Reg<self::usbdev::arb_rw1_wa::ARB_RW1_WA_SPEC>,
    #[doc = "0x214 - Endpoint Write Address value *1"]
    pub arb_rw1_wa_msb: crate::Reg<self::usbdev::arb_rw1_wa_msb::ARB_RW1_WA_MSB_SPEC>,
    #[doc = "0x218 - Endpoint Read Address value *1"]
    pub arb_rw1_ra: crate::Reg<self::usbdev::arb_rw1_ra::ARB_RW1_RA_SPEC>,
    #[doc = "0x21c - Endpoint Read Address value *1"]
    pub arb_rw1_ra_msb: crate::Reg<self::usbdev::arb_rw1_ra_msb::ARB_RW1_RA_MSB_SPEC>,
    #[doc = "0x220 - Endpoint Data Register"]
    pub arb_rw1_dr: crate::Reg<self::usbdev::arb_rw1_dr::ARB_RW1_DR_SPEC>,
    _reserved47: [u8; 0x0c],
    #[doc = "0x230 - Dedicated Endpoint Buffer Size Register *1"]
    pub buf_size: crate::Reg<self::usbdev::buf_size::BUF_SIZE_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0x238 - Endpoint Active Indication Register *1"]
    pub ep_active: crate::Reg<self::usbdev::ep_active::EP_ACTIVE_SPEC>,
    #[doc = "0x23c - Endpoint Type (IN/OUT) Indication *1"]
    pub ep_type: crate::Reg<self::usbdev::ep_type::EP_TYPE_SPEC>,
    #[doc = "0x240 - Endpoint Configuration Register *1"]
    pub arb_ep2_cfg: crate::Reg<self::usbdev::arb_ep2_cfg::ARB_EP2_CFG_SPEC>,
    #[doc = "0x244 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_int_en: crate::Reg<self::usbdev::arb_ep2_int_en::ARB_EP2_INT_EN_SPEC>,
    #[doc = "0x248 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_sr: crate::Reg<self::usbdev::arb_ep2_sr::ARB_EP2_SR_SPEC>,
    _reserved53: [u8; 0x04],
    #[doc = "0x250 - Endpoint Write Address value *1"]
    pub arb_rw2_wa: crate::Reg<self::usbdev::arb_rw2_wa::ARB_RW2_WA_SPEC>,
    #[doc = "0x254 - Endpoint Write Address value *1"]
    pub arb_rw2_wa_msb: crate::Reg<self::usbdev::arb_rw2_wa_msb::ARB_RW2_WA_MSB_SPEC>,
    #[doc = "0x258 - Endpoint Read Address value *1"]
    pub arb_rw2_ra: crate::Reg<self::usbdev::arb_rw2_ra::ARB_RW2_RA_SPEC>,
    #[doc = "0x25c - Endpoint Read Address value *1"]
    pub arb_rw2_ra_msb: crate::Reg<self::usbdev::arb_rw2_ra_msb::ARB_RW2_RA_MSB_SPEC>,
    #[doc = "0x260 - Endpoint Data Register"]
    pub arb_rw2_dr: crate::Reg<self::usbdev::arb_rw2_dr::ARB_RW2_DR_SPEC>,
    _reserved58: [u8; 0x0c],
    #[doc = "0x270 - Arbiter Configuration Register *1"]
    pub arb_cfg: crate::Reg<self::usbdev::arb_cfg::ARB_CFG_SPEC>,
    #[doc = "0x274 - USB Block Clock Enable Register"]
    pub usb_clk_en: crate::Reg<self::usbdev::usb_clk_en::USB_CLK_EN_SPEC>,
    #[doc = "0x278 - Arbiter Interrupt Enable *1"]
    pub arb_int_en: crate::Reg<self::usbdev::arb_int_en::ARB_INT_EN_SPEC>,
    #[doc = "0x27c - Arbiter Interrupt Status *1"]
    pub arb_int_sr: crate::Reg<self::usbdev::arb_int_sr::ARB_INT_SR_SPEC>,
    #[doc = "0x280 - Endpoint Configuration Register *1"]
    pub arb_ep3_cfg: crate::Reg<self::usbdev::arb_ep3_cfg::ARB_EP3_CFG_SPEC>,
    #[doc = "0x284 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_int_en: crate::Reg<self::usbdev::arb_ep3_int_en::ARB_EP3_INT_EN_SPEC>,
    #[doc = "0x288 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_sr: crate::Reg<self::usbdev::arb_ep3_sr::ARB_EP3_SR_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0x290 - Endpoint Write Address value *1"]
    pub arb_rw3_wa: crate::Reg<self::usbdev::arb_rw3_wa::ARB_RW3_WA_SPEC>,
    #[doc = "0x294 - Endpoint Write Address value *1"]
    pub arb_rw3_wa_msb: crate::Reg<self::usbdev::arb_rw3_wa_msb::ARB_RW3_WA_MSB_SPEC>,
    #[doc = "0x298 - Endpoint Read Address value *1"]
    pub arb_rw3_ra: crate::Reg<self::usbdev::arb_rw3_ra::ARB_RW3_RA_SPEC>,
    #[doc = "0x29c - Endpoint Read Address value *1"]
    pub arb_rw3_ra_msb: crate::Reg<self::usbdev::arb_rw3_ra_msb::ARB_RW3_RA_MSB_SPEC>,
    #[doc = "0x2a0 - Endpoint Data Register"]
    pub arb_rw3_dr: crate::Reg<self::usbdev::arb_rw3_dr::ARB_RW3_DR_SPEC>,
    _reserved70: [u8; 0x0c],
    #[doc = "0x2b0 - Common Area Write Address *1"]
    pub cwa: crate::Reg<self::usbdev::cwa::CWA_SPEC>,
    #[doc = "0x2b4 - Endpoint Read Address value *1"]
    pub cwa_msb: crate::Reg<self::usbdev::cwa_msb::CWA_MSB_SPEC>,
    _reserved72: [u8; 0x08],
    #[doc = "0x2c0 - Endpoint Configuration Register *1"]
    pub arb_ep4_cfg: crate::Reg<self::usbdev::arb_ep4_cfg::ARB_EP4_CFG_SPEC>,
    #[doc = "0x2c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_int_en: crate::Reg<self::usbdev::arb_ep4_int_en::ARB_EP4_INT_EN_SPEC>,
    #[doc = "0x2c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_sr: crate::Reg<self::usbdev::arb_ep4_sr::ARB_EP4_SR_SPEC>,
    _reserved75: [u8; 0x04],
    #[doc = "0x2d0 - Endpoint Write Address value *1"]
    pub arb_rw4_wa: crate::Reg<self::usbdev::arb_rw4_wa::ARB_RW4_WA_SPEC>,
    #[doc = "0x2d4 - Endpoint Write Address value *1"]
    pub arb_rw4_wa_msb: crate::Reg<self::usbdev::arb_rw4_wa_msb::ARB_RW4_WA_MSB_SPEC>,
    #[doc = "0x2d8 - Endpoint Read Address value *1"]
    pub arb_rw4_ra: crate::Reg<self::usbdev::arb_rw4_ra::ARB_RW4_RA_SPEC>,
    #[doc = "0x2dc - Endpoint Read Address value *1"]
    pub arb_rw4_ra_msb: crate::Reg<self::usbdev::arb_rw4_ra_msb::ARB_RW4_RA_MSB_SPEC>,
    #[doc = "0x2e0 - Endpoint Data Register"]
    pub arb_rw4_dr: crate::Reg<self::usbdev::arb_rw4_dr::ARB_RW4_DR_SPEC>,
    _reserved80: [u8; 0x0c],
    #[doc = "0x2f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres: crate::Reg<self::usbdev::dma_thres::DMA_THRES_SPEC>,
    #[doc = "0x2f4 - DMA Burst / Threshold Configuration"]
    pub dma_thres_msb: crate::Reg<self::usbdev::dma_thres_msb::DMA_THRES_MSB_SPEC>,
    _reserved82: [u8; 0x08],
    #[doc = "0x300 - Endpoint Configuration Register *1"]
    pub arb_ep5_cfg: crate::Reg<self::usbdev::arb_ep5_cfg::ARB_EP5_CFG_SPEC>,
    #[doc = "0x304 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_int_en: crate::Reg<self::usbdev::arb_ep5_int_en::ARB_EP5_INT_EN_SPEC>,
    #[doc = "0x308 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_sr: crate::Reg<self::usbdev::arb_ep5_sr::ARB_EP5_SR_SPEC>,
    _reserved85: [u8; 0x04],
    #[doc = "0x310 - Endpoint Write Address value *1"]
    pub arb_rw5_wa: crate::Reg<self::usbdev::arb_rw5_wa::ARB_RW5_WA_SPEC>,
    #[doc = "0x314 - Endpoint Write Address value *1"]
    pub arb_rw5_wa_msb: crate::Reg<self::usbdev::arb_rw5_wa_msb::ARB_RW5_WA_MSB_SPEC>,
    #[doc = "0x318 - Endpoint Read Address value *1"]
    pub arb_rw5_ra: crate::Reg<self::usbdev::arb_rw5_ra::ARB_RW5_RA_SPEC>,
    #[doc = "0x31c - Endpoint Read Address value *1"]
    pub arb_rw5_ra_msb: crate::Reg<self::usbdev::arb_rw5_ra_msb::ARB_RW5_RA_MSB_SPEC>,
    #[doc = "0x320 - Endpoint Data Register"]
    pub arb_rw5_dr: crate::Reg<self::usbdev::arb_rw5_dr::ARB_RW5_DR_SPEC>,
    _reserved90: [u8; 0x0c],
    #[doc = "0x330 - Bus Reset Count Register"]
    pub bus_rst_cnt: crate::Reg<self::usbdev::bus_rst_cnt::BUS_RST_CNT_SPEC>,
    _reserved91: [u8; 0x0c],
    #[doc = "0x340 - Endpoint Configuration Register *1"]
    pub arb_ep6_cfg: crate::Reg<self::usbdev::arb_ep6_cfg::ARB_EP6_CFG_SPEC>,
    #[doc = "0x344 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_int_en: crate::Reg<self::usbdev::arb_ep6_int_en::ARB_EP6_INT_EN_SPEC>,
    #[doc = "0x348 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_sr: crate::Reg<self::usbdev::arb_ep6_sr::ARB_EP6_SR_SPEC>,
    _reserved94: [u8; 0x04],
    #[doc = "0x350 - Endpoint Write Address value *1"]
    pub arb_rw6_wa: crate::Reg<self::usbdev::arb_rw6_wa::ARB_RW6_WA_SPEC>,
    #[doc = "0x354 - Endpoint Write Address value *1"]
    pub arb_rw6_wa_msb: crate::Reg<self::usbdev::arb_rw6_wa_msb::ARB_RW6_WA_MSB_SPEC>,
    #[doc = "0x358 - Endpoint Read Address value *1"]
    pub arb_rw6_ra: crate::Reg<self::usbdev::arb_rw6_ra::ARB_RW6_RA_SPEC>,
    #[doc = "0x35c - Endpoint Read Address value *1"]
    pub arb_rw6_ra_msb: crate::Reg<self::usbdev::arb_rw6_ra_msb::ARB_RW6_RA_MSB_SPEC>,
    #[doc = "0x360 - Endpoint Data Register"]
    pub arb_rw6_dr: crate::Reg<self::usbdev::arb_rw6_dr::ARB_RW6_DR_SPEC>,
    _reserved99: [u8; 0x1c],
    #[doc = "0x380 - Endpoint Configuration Register *1"]
    pub arb_ep7_cfg: crate::Reg<self::usbdev::arb_ep7_cfg::ARB_EP7_CFG_SPEC>,
    #[doc = "0x384 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_int_en: crate::Reg<self::usbdev::arb_ep7_int_en::ARB_EP7_INT_EN_SPEC>,
    #[doc = "0x388 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_sr: crate::Reg<self::usbdev::arb_ep7_sr::ARB_EP7_SR_SPEC>,
    _reserved102: [u8; 0x04],
    #[doc = "0x390 - Endpoint Write Address value *1"]
    pub arb_rw7_wa: crate::Reg<self::usbdev::arb_rw7_wa::ARB_RW7_WA_SPEC>,
    #[doc = "0x394 - Endpoint Write Address value *1"]
    pub arb_rw7_wa_msb: crate::Reg<self::usbdev::arb_rw7_wa_msb::ARB_RW7_WA_MSB_SPEC>,
    #[doc = "0x398 - Endpoint Read Address value *1"]
    pub arb_rw7_ra: crate::Reg<self::usbdev::arb_rw7_ra::ARB_RW7_RA_SPEC>,
    #[doc = "0x39c - Endpoint Read Address value *1"]
    pub arb_rw7_ra_msb: crate::Reg<self::usbdev::arb_rw7_ra_msb::ARB_RW7_RA_MSB_SPEC>,
    #[doc = "0x3a0 - Endpoint Data Register"]
    pub arb_rw7_dr: crate::Reg<self::usbdev::arb_rw7_dr::ARB_RW7_DR_SPEC>,
    _reserved107: [u8; 0x1c],
    #[doc = "0x3c0 - Endpoint Configuration Register *1"]
    pub arb_ep8_cfg: crate::Reg<self::usbdev::arb_ep8_cfg::ARB_EP8_CFG_SPEC>,
    #[doc = "0x3c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_int_en: crate::Reg<self::usbdev::arb_ep8_int_en::ARB_EP8_INT_EN_SPEC>,
    #[doc = "0x3c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_sr: crate::Reg<self::usbdev::arb_ep8_sr::ARB_EP8_SR_SPEC>,
    _reserved110: [u8; 0x04],
    #[doc = "0x3d0 - Endpoint Write Address value *1"]
    pub arb_rw8_wa: crate::Reg<self::usbdev::arb_rw8_wa::ARB_RW8_WA_SPEC>,
    #[doc = "0x3d4 - Endpoint Write Address value *1"]
    pub arb_rw8_wa_msb: crate::Reg<self::usbdev::arb_rw8_wa_msb::ARB_RW8_WA_MSB_SPEC>,
    #[doc = "0x3d8 - Endpoint Read Address value *1"]
    pub arb_rw8_ra: crate::Reg<self::usbdev::arb_rw8_ra::ARB_RW8_RA_SPEC>,
    #[doc = "0x3dc - Endpoint Read Address value *1"]
    pub arb_rw8_ra_msb: crate::Reg<self::usbdev::arb_rw8_ra_msb::ARB_RW8_RA_MSB_SPEC>,
    #[doc = "0x3e0 - Endpoint Data Register"]
    pub arb_rw8_dr: crate::Reg<self::usbdev::arb_rw8_dr::ARB_RW8_DR_SPEC>,
    _reserved115: [u8; 0x1c],
    #[doc = "0x400..0xc00 - DATA"]
    pub mem_data: [crate::Reg<self::usbdev::mem_data::MEM_DATA_SPEC>; 512],
    _reserved116: [u8; 0x0460],
    #[doc = "0x1060 - Start Of Frame Register"]
    pub sof16: crate::Reg<self::usbdev::sof16::SOF16_SPEC>,
    _reserved117: [u8; 0x1c],
    #[doc = "0x1080 - Oscillator lock data register"]
    pub osclk_dr16: crate::Reg<self::usbdev::osclk_dr16::OSCLK_DR16_SPEC>,
    _reserved118: [u8; 0x018c],
    #[doc = "0x1210 - Endpoint Write Address value"]
    pub arb_rw1_wa16: crate::Reg<self::usbdev::arb_rw1_wa16::ARB_RW1_WA16_SPEC>,
    _reserved119: [u8; 0x04],
    #[doc = "0x1218 - Endpoint Read Address value"]
    pub arb_rw1_ra16: crate::Reg<self::usbdev::arb_rw1_ra16::ARB_RW1_RA16_SPEC>,
    _reserved120: [u8; 0x04],
    #[doc = "0x1220 - Endpoint Data Register"]
    pub arb_rw1_dr16: crate::Reg<self::usbdev::arb_rw1_dr16::ARB_RW1_DR16_SPEC>,
    _reserved121: [u8; 0x2c],
    #[doc = "0x1250 - Endpoint Write Address value"]
    pub arb_rw2_wa16: crate::Reg<self::usbdev::arb_rw2_wa16::ARB_RW2_WA16_SPEC>,
    _reserved122: [u8; 0x04],
    #[doc = "0x1258 - Endpoint Read Address value"]
    pub arb_rw2_ra16: crate::Reg<self::usbdev::arb_rw2_ra16::ARB_RW2_RA16_SPEC>,
    _reserved123: [u8; 0x04],
    #[doc = "0x1260 - Endpoint Data Register"]
    pub arb_rw2_dr16: crate::Reg<self::usbdev::arb_rw2_dr16::ARB_RW2_DR16_SPEC>,
    _reserved124: [u8; 0x2c],
    #[doc = "0x1290 - Endpoint Write Address value"]
    pub arb_rw3_wa16: crate::Reg<self::usbdev::arb_rw3_wa16::ARB_RW3_WA16_SPEC>,
    _reserved125: [u8; 0x04],
    #[doc = "0x1298 - Endpoint Read Address value"]
    pub arb_rw3_ra16: crate::Reg<self::usbdev::arb_rw3_ra16::ARB_RW3_RA16_SPEC>,
    _reserved126: [u8; 0x04],
    #[doc = "0x12a0 - Endpoint Data Register"]
    pub arb_rw3_dr16: crate::Reg<self::usbdev::arb_rw3_dr16::ARB_RW3_DR16_SPEC>,
    _reserved127: [u8; 0x0c],
    #[doc = "0x12b0 - Common Area Write Address"]
    pub cwa16: crate::Reg<self::usbdev::cwa16::CWA16_SPEC>,
    _reserved128: [u8; 0x1c],
    #[doc = "0x12d0 - Endpoint Write Address value"]
    pub arb_rw4_wa16: crate::Reg<self::usbdev::arb_rw4_wa16::ARB_RW4_WA16_SPEC>,
    _reserved129: [u8; 0x04],
    #[doc = "0x12d8 - Endpoint Read Address value"]
    pub arb_rw4_ra16: crate::Reg<self::usbdev::arb_rw4_ra16::ARB_RW4_RA16_SPEC>,
    _reserved130: [u8; 0x04],
    #[doc = "0x12e0 - Endpoint Data Register"]
    pub arb_rw4_dr16: crate::Reg<self::usbdev::arb_rw4_dr16::ARB_RW4_DR16_SPEC>,
    _reserved131: [u8; 0x0c],
    #[doc = "0x12f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres16: crate::Reg<self::usbdev::dma_thres16::DMA_THRES16_SPEC>,
    _reserved132: [u8; 0x1c],
    #[doc = "0x1310 - Endpoint Write Address value"]
    pub arb_rw5_wa16: crate::Reg<self::usbdev::arb_rw5_wa16::ARB_RW5_WA16_SPEC>,
    _reserved133: [u8; 0x04],
    #[doc = "0x1318 - Endpoint Read Address value"]
    pub arb_rw5_ra16: crate::Reg<self::usbdev::arb_rw5_ra16::ARB_RW5_RA16_SPEC>,
    _reserved134: [u8; 0x04],
    #[doc = "0x1320 - Endpoint Data Register"]
    pub arb_rw5_dr16: crate::Reg<self::usbdev::arb_rw5_dr16::ARB_RW5_DR16_SPEC>,
    _reserved135: [u8; 0x2c],
    #[doc = "0x1350 - Endpoint Write Address value"]
    pub arb_rw6_wa16: crate::Reg<self::usbdev::arb_rw6_wa16::ARB_RW6_WA16_SPEC>,
    _reserved136: [u8; 0x04],
    #[doc = "0x1358 - Endpoint Read Address value"]
    pub arb_rw6_ra16: crate::Reg<self::usbdev::arb_rw6_ra16::ARB_RW6_RA16_SPEC>,
    _reserved137: [u8; 0x04],
    #[doc = "0x1360 - Endpoint Data Register"]
    pub arb_rw6_dr16: crate::Reg<self::usbdev::arb_rw6_dr16::ARB_RW6_DR16_SPEC>,
    _reserved138: [u8; 0x2c],
    #[doc = "0x1390 - Endpoint Write Address value"]
    pub arb_rw7_wa16: crate::Reg<self::usbdev::arb_rw7_wa16::ARB_RW7_WA16_SPEC>,
    _reserved139: [u8; 0x04],
    #[doc = "0x1398 - Endpoint Read Address value"]
    pub arb_rw7_ra16: crate::Reg<self::usbdev::arb_rw7_ra16::ARB_RW7_RA16_SPEC>,
    _reserved140: [u8; 0x04],
    #[doc = "0x13a0 - Endpoint Data Register"]
    pub arb_rw7_dr16: crate::Reg<self::usbdev::arb_rw7_dr16::ARB_RW7_DR16_SPEC>,
    _reserved141: [u8; 0x2c],
    #[doc = "0x13d0 - Endpoint Write Address value"]
    pub arb_rw8_wa16: crate::Reg<self::usbdev::arb_rw8_wa16::ARB_RW8_WA16_SPEC>,
    _reserved142: [u8; 0x04],
    #[doc = "0x13d8 - Endpoint Read Address value"]
    pub arb_rw8_ra16: crate::Reg<self::usbdev::arb_rw8_ra16::ARB_RW8_RA16_SPEC>,
    _reserved143: [u8; 0x04],
    #[doc = "0x13e0 - Endpoint Data Register"]
    pub arb_rw8_dr16: crate::Reg<self::usbdev::arb_rw8_dr16::ARB_RW8_DR16_SPEC>,
}
#[doc = r"Register block"]
#[doc = "USB Device"]
pub mod usbdev;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBLPM {
    #[doc = "0x00 - Power Control Register"]
    pub power_ctl: crate::Reg<self::usblpm::power_ctl::POWER_CTL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - USB IO Control Register"]
    pub usbio_ctl: crate::Reg<self::usblpm::usbio_ctl::USBIO_CTL_SPEC>,
    #[doc = "0x0c - Flow Control Register"]
    pub flow_ctl: crate::Reg<self::usblpm::flow_ctl::FLOW_CTL_SPEC>,
    #[doc = "0x10 - LPM Control Register"]
    pub lpm_ctl: crate::Reg<self::usblpm::lpm_ctl::LPM_CTL_SPEC>,
    #[doc = "0x14 - LPM Status register"]
    pub lpm_stat: crate::Reg<self::usblpm::lpm_stat::LPM_STAT_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - USB SOF, BUS RESET and EP0 Interrupt Status"]
    pub intr_sie: crate::Reg<self::usblpm::intr_sie::INTR_SIE_SPEC>,
    #[doc = "0x24 - USB SOF, BUS RESET and EP0 Interrupt Set"]
    pub intr_sie_set: crate::Reg<self::usblpm::intr_sie_set::INTR_SIE_SET_SPEC>,
    #[doc = "0x28 - USB SOF, BUS RESET and EP0 Interrupt Mask"]
    pub intr_sie_mask: crate::Reg<self::usblpm::intr_sie_mask::INTR_SIE_MASK_SPEC>,
    #[doc = "0x2c - USB SOF, BUS RESET and EP0 Interrupt Masked"]
    pub intr_sie_masked: crate::Reg<self::usblpm::intr_sie_masked::INTR_SIE_MASKED_SPEC>,
    #[doc = "0x30 - Select interrupt level for each interrupt source"]
    pub intr_lvl_sel: crate::Reg<self::usblpm::intr_lvl_sel::INTR_LVL_SEL_SPEC>,
    #[doc = "0x34 - High priority interrupt Cause register"]
    pub intr_cause_hi: crate::Reg<self::usblpm::intr_cause_hi::INTR_CAUSE_HI_SPEC>,
    #[doc = "0x38 - Medium priority interrupt Cause register"]
    pub intr_cause_med: crate::Reg<self::usblpm::intr_cause_med::INTR_CAUSE_MED_SPEC>,
    #[doc = "0x3c - Low priority interrupt Cause register"]
    pub intr_cause_lo: crate::Reg<self::usblpm::intr_cause_lo::INTR_CAUSE_LO_SPEC>,
    _reserved13: [u8; 0x30],
    #[doc = "0x70 - DFT control"]
    pub dft_ctl: crate::Reg<self::usblpm::dft_ctl::DFT_CTL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "USB Device LPM and PHY Test"]
pub mod usblpm;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHOST {
    #[doc = "0x00 - Host Control 0 Register."]
    pub host_ctl0: crate::Reg<self::usbhost::host_ctl0::HOST_CTL0_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Host Control 1 Register."]
    pub host_ctl1: crate::Reg<self::usbhost::host_ctl1::HOST_CTL1_SPEC>,
    _reserved2: [u8; 0xec],
    #[doc = "0x100 - Host Control 2 Register."]
    pub host_ctl2: crate::Reg<self::usbhost::host_ctl2::HOST_CTL2_SPEC>,
    #[doc = "0x104 - Host Error Status Register."]
    pub host_err: crate::Reg<self::usbhost::host_err::HOST_ERR_SPEC>,
    #[doc = "0x108 - Host Status Register."]
    pub host_status: crate::Reg<self::usbhost::host_status::HOST_STATUS_SPEC>,
    #[doc = "0x10c - Host SOF Interrupt Frame Compare Register"]
    pub host_fcomp: crate::Reg<self::usbhost::host_fcomp::HOST_FCOMP_SPEC>,
    #[doc = "0x110 - Host Retry Timer Setup Register"]
    pub host_rtimer: crate::Reg<self::usbhost::host_rtimer::HOST_RTIMER_SPEC>,
    #[doc = "0x114 - Host Address Register"]
    pub host_addr: crate::Reg<self::usbhost::host_addr::HOST_ADDR_SPEC>,
    #[doc = "0x118 - Host EOF Setup Register"]
    pub host_eof: crate::Reg<self::usbhost::host_eof::HOST_EOF_SPEC>,
    #[doc = "0x11c - Host Frame Setup Register"]
    pub host_frame: crate::Reg<self::usbhost::host_frame::HOST_FRAME_SPEC>,
    #[doc = "0x120 - Host Token Endpoint Register"]
    pub host_token: crate::Reg<self::usbhost::host_token::HOST_TOKEN_SPEC>,
    _reserved11: [u8; 0x02dc],
    #[doc = "0x400 - Host Endpoint 1 Control Register"]
    pub host_ep1_ctl: crate::Reg<self::usbhost::host_ep1_ctl::HOST_EP1_CTL_SPEC>,
    #[doc = "0x404 - Host Endpoint 1 Status Register"]
    pub host_ep1_status: crate::Reg<self::usbhost::host_ep1_status::HOST_EP1_STATUS_SPEC>,
    #[doc = "0x408 - Host Endpoint 1 Data 1-Byte Register"]
    pub host_ep1_rw1_dr: crate::Reg<self::usbhost::host_ep1_rw1_dr::HOST_EP1_RW1_DR_SPEC>,
    #[doc = "0x40c - Host Endpoint 1 Data 2-Byte Register"]
    pub host_ep1_rw2_dr: crate::Reg<self::usbhost::host_ep1_rw2_dr::HOST_EP1_RW2_DR_SPEC>,
    _reserved15: [u8; 0xf0],
    #[doc = "0x500 - Host Endpoint 2 Control Register"]
    pub host_ep2_ctl: crate::Reg<self::usbhost::host_ep2_ctl::HOST_EP2_CTL_SPEC>,
    #[doc = "0x504 - Host Endpoint 2 Status Register"]
    pub host_ep2_status: crate::Reg<self::usbhost::host_ep2_status::HOST_EP2_STATUS_SPEC>,
    #[doc = "0x508 - Host Endpoint 2 Data 1-Byte Register"]
    pub host_ep2_rw1_dr: crate::Reg<self::usbhost::host_ep2_rw1_dr::HOST_EP2_RW1_DR_SPEC>,
    #[doc = "0x50c - Host Endpoint 2 Data 2-Byte Register"]
    pub host_ep2_rw2_dr: crate::Reg<self::usbhost::host_ep2_rw2_dr::HOST_EP2_RW2_DR_SPEC>,
    _reserved19: [u8; 0x02f0],
    #[doc = "0x800 - Host Interrupt Level 1 Selection Register"]
    pub host_lvl1_sel: crate::Reg<self::usbhost::host_lvl1_sel::HOST_LVL1_SEL_SPEC>,
    #[doc = "0x804 - Host Interrupt Level 2 Selection Register"]
    pub host_lvl2_sel: crate::Reg<self::usbhost::host_lvl2_sel::HOST_LVL2_SEL_SPEC>,
    _reserved21: [u8; 0xf8],
    #[doc = "0x900 - Interrupt USB Host Cause High Register"]
    pub intr_usbhost_cause_hi:
        crate::Reg<self::usbhost::intr_usbhost_cause_hi::INTR_USBHOST_CAUSE_HI_SPEC>,
    #[doc = "0x904 - Interrupt USB Host Cause Medium Register"]
    pub intr_usbhost_cause_med:
        crate::Reg<self::usbhost::intr_usbhost_cause_med::INTR_USBHOST_CAUSE_MED_SPEC>,
    #[doc = "0x908 - Interrupt USB Host Cause Low Register"]
    pub intr_usbhost_cause_lo:
        crate::Reg<self::usbhost::intr_usbhost_cause_lo::INTR_USBHOST_CAUSE_LO_SPEC>,
    _reserved24: [u8; 0x14],
    #[doc = "0x920 - Interrupt USB Host Endpoint Cause High Register"]
    pub intr_host_ep_cause_hi:
        crate::Reg<self::usbhost::intr_host_ep_cause_hi::INTR_HOST_EP_CAUSE_HI_SPEC>,
    #[doc = "0x924 - Interrupt USB Host Endpoint Cause Medium Register"]
    pub intr_host_ep_cause_med:
        crate::Reg<self::usbhost::intr_host_ep_cause_med::INTR_HOST_EP_CAUSE_MED_SPEC>,
    #[doc = "0x928 - Interrupt USB Host Endpoint Cause Low Register"]
    pub intr_host_ep_cause_lo:
        crate::Reg<self::usbhost::intr_host_ep_cause_lo::INTR_HOST_EP_CAUSE_LO_SPEC>,
    _reserved27: [u8; 0x14],
    #[doc = "0x940 - Interrupt USB Host Register"]
    pub intr_usbhost: crate::Reg<self::usbhost::intr_usbhost::INTR_USBHOST_SPEC>,
    #[doc = "0x944 - Interrupt USB Host Set Register"]
    pub intr_usbhost_set: crate::Reg<self::usbhost::intr_usbhost_set::INTR_USBHOST_SET_SPEC>,
    #[doc = "0x948 - Interrupt USB Host Mask Register"]
    pub intr_usbhost_mask: crate::Reg<self::usbhost::intr_usbhost_mask::INTR_USBHOST_MASK_SPEC>,
    #[doc = "0x94c - Interrupt USB Host Masked Register"]
    pub intr_usbhost_masked:
        crate::Reg<self::usbhost::intr_usbhost_masked::INTR_USBHOST_MASKED_SPEC>,
    _reserved31: [u8; 0xb0],
    #[doc = "0xa00 - Interrupt USB Host Endpoint Register"]
    pub intr_host_ep: crate::Reg<self::usbhost::intr_host_ep::INTR_HOST_EP_SPEC>,
    #[doc = "0xa04 - Interrupt USB Host Endpoint Set Register"]
    pub intr_host_ep_set: crate::Reg<self::usbhost::intr_host_ep_set::INTR_HOST_EP_SET_SPEC>,
    #[doc = "0xa08 - Interrupt USB Host Endpoint Mask Register"]
    pub intr_host_ep_mask: crate::Reg<self::usbhost::intr_host_ep_mask::INTR_HOST_EP_MASK_SPEC>,
    #[doc = "0xa0c - Interrupt USB Host Endpoint Masked Register"]
    pub intr_host_ep_masked:
        crate::Reg<self::usbhost::intr_host_ep_masked::INTR_HOST_EP_MASKED_SPEC>,
    _reserved35: [u8; 0xf0],
    #[doc = "0xb00 - Host DMA Enable Register"]
    pub host_dma_enbl: crate::Reg<self::usbhost::host_dma_enbl::HOST_DMA_ENBL_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0xb20 - Host Endpoint 1 Block Register"]
    pub host_ep1_blk: crate::Reg<self::usbhost::host_ep1_blk::HOST_EP1_BLK_SPEC>,
    _reserved37: [u8; 0x0c],
    #[doc = "0xb30 - Host Endpoint 2 Block Register"]
    pub host_ep2_blk: crate::Reg<self::usbhost::host_ep2_blk::HOST_EP2_BLK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "USB Host Controller"]
pub mod usbhost;
