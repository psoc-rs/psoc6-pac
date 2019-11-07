#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device"]
    pub usbdev: USBDEV,
    _reserved1: [u8; 3100usize],
    #[doc = "0x2000 - USB Device LPM and PHY Test"]
    pub usblpm: USBLPM,
    _reserved2: [u8; 8076usize],
    #[doc = "0x4000 - USB Host Controller"]
    pub usbhost: USBHOST,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBDEV {
    #[doc = "0x00 - Control End point EP0 Data Register"]
    pub ep0_dr: [self::usbdev::EP0_DR; 8],
    #[doc = "0x20 - USB control 0 Register"]
    pub cr0: self::usbdev::CR0,
    #[doc = "0x24 - USB control 1 Register"]
    pub cr1: self::usbdev::CR1,
    #[doc = "0x28 - USB SIE Data Endpoints Interrupt Enable Register"]
    pub sie_ep_int_en: self::usbdev::SIE_EP_INT_EN,
    #[doc = "0x2c - USB SIE Data Endpoint Interrupt Status"]
    pub sie_ep_int_sr: self::usbdev::SIE_EP_INT_SR,
    #[doc = "0x30 - Non-control endpoint count register"]
    pub sie_ep1_cnt0: self::usbdev::SIE_EP1_CNT0,
    #[doc = "0x34 - Non-control endpoint count register"]
    pub sie_ep1_cnt1: self::usbdev::SIE_EP1_CNT1,
    #[doc = "0x38 - Non-control endpoint's control Register"]
    pub sie_ep1_cr0: self::usbdev::SIE_EP1_CR0,
    _reserved8: [u8; 4usize],
    #[doc = "0x40 - USBIO Control 0 Register"]
    pub usbio_cr0: self::usbdev::USBIO_CR0,
    #[doc = "0x44 - USBIO control 2 Register"]
    pub usbio_cr2: self::usbdev::USBIO_CR2,
    #[doc = "0x48 - USBIO control 1 Register"]
    pub usbio_cr1: self::usbdev::USBIO_CR1,
    _reserved11: [u8; 4usize],
    #[doc = "0x50 - USB Dynamic reconfiguration register"]
    pub dyn_reconfig: self::usbdev::DYN_RECONFIG,
    _reserved12: [u8; 12usize],
    #[doc = "0x60 - Start Of Frame Register"]
    pub sof0: self::usbdev::SOF0,
    #[doc = "0x64 - Start Of Frame Register"]
    pub sof1: self::usbdev::SOF1,
    _reserved14: [u8; 8usize],
    #[doc = "0x70 - Non-control endpoint count register"]
    pub sie_ep2_cnt0: self::usbdev::SIE_EP2_CNT0,
    #[doc = "0x74 - Non-control endpoint count register"]
    pub sie_ep2_cnt1: self::usbdev::SIE_EP2_CNT1,
    #[doc = "0x78 - Non-control endpoint's control Register"]
    pub sie_ep2_cr0: self::usbdev::SIE_EP2_CR0,
    _reserved17: [u8; 4usize],
    #[doc = "0x80 - Oscillator lock data register 0"]
    pub osclk_dr0: self::usbdev::OSCLK_DR0,
    #[doc = "0x84 - Oscillator lock data register 1"]
    pub osclk_dr1: self::usbdev::OSCLK_DR1,
    _reserved19: [u8; 24usize],
    #[doc = "0xa0 - Endpoint0 control Register"]
    pub ep0_cr: self::usbdev::EP0_CR,
    #[doc = "0xa4 - Endpoint0 count Register"]
    pub ep0_cnt: self::usbdev::EP0_CNT,
    _reserved21: [u8; 8usize],
    #[doc = "0xb0 - Non-control endpoint count register"]
    pub sie_ep3_cnt0: self::usbdev::SIE_EP3_CNT0,
    #[doc = "0xb4 - Non-control endpoint count register"]
    pub sie_ep3_cnt1: self::usbdev::SIE_EP3_CNT1,
    #[doc = "0xb8 - Non-control endpoint's control Register"]
    pub sie_ep3_cr0: self::usbdev::SIE_EP3_CR0,
    _reserved24: [u8; 52usize],
    #[doc = "0xf0 - Non-control endpoint count register"]
    pub sie_ep4_cnt0: self::usbdev::SIE_EP4_CNT0,
    #[doc = "0xf4 - Non-control endpoint count register"]
    pub sie_ep4_cnt1: self::usbdev::SIE_EP4_CNT1,
    #[doc = "0xf8 - Non-control endpoint's control Register"]
    pub sie_ep4_cr0: self::usbdev::SIE_EP4_CR0,
    _reserved27: [u8; 52usize],
    #[doc = "0x130 - Non-control endpoint count register"]
    pub sie_ep5_cnt0: self::usbdev::SIE_EP5_CNT0,
    #[doc = "0x134 - Non-control endpoint count register"]
    pub sie_ep5_cnt1: self::usbdev::SIE_EP5_CNT1,
    #[doc = "0x138 - Non-control endpoint's control Register"]
    pub sie_ep5_cr0: self::usbdev::SIE_EP5_CR0,
    _reserved30: [u8; 52usize],
    #[doc = "0x170 - Non-control endpoint count register"]
    pub sie_ep6_cnt0: self::usbdev::SIE_EP6_CNT0,
    #[doc = "0x174 - Non-control endpoint count register"]
    pub sie_ep6_cnt1: self::usbdev::SIE_EP6_CNT1,
    #[doc = "0x178 - Non-control endpoint's control Register"]
    pub sie_ep6_cr0: self::usbdev::SIE_EP6_CR0,
    _reserved33: [u8; 52usize],
    #[doc = "0x1b0 - Non-control endpoint count register"]
    pub sie_ep7_cnt0: self::usbdev::SIE_EP7_CNT0,
    #[doc = "0x1b4 - Non-control endpoint count register"]
    pub sie_ep7_cnt1: self::usbdev::SIE_EP7_CNT1,
    #[doc = "0x1b8 - Non-control endpoint's control Register"]
    pub sie_ep7_cr0: self::usbdev::SIE_EP7_CR0,
    _reserved36: [u8; 52usize],
    #[doc = "0x1f0 - Non-control endpoint count register"]
    pub sie_ep8_cnt0: self::usbdev::SIE_EP8_CNT0,
    #[doc = "0x1f4 - Non-control endpoint count register"]
    pub sie_ep8_cnt1: self::usbdev::SIE_EP8_CNT1,
    #[doc = "0x1f8 - Non-control endpoint's control Register"]
    pub sie_ep8_cr0: self::usbdev::SIE_EP8_CR0,
    _reserved39: [u8; 4usize],
    #[doc = "0x200 - Endpoint Configuration Register *1"]
    pub arb_ep1_cfg: self::usbdev::ARB_EP1_CFG,
    #[doc = "0x204 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_int_en: self::usbdev::ARB_EP1_INT_EN,
    #[doc = "0x208 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_sr: self::usbdev::ARB_EP1_SR,
    _reserved42: [u8; 4usize],
    #[doc = "0x210 - Endpoint Write Address value *1"]
    pub arb_rw1_wa: self::usbdev::ARB_RW1_WA,
    #[doc = "0x214 - Endpoint Write Address value *1"]
    pub arb_rw1_wa_msb: self::usbdev::ARB_RW1_WA_MSB,
    #[doc = "0x218 - Endpoint Read Address value *1"]
    pub arb_rw1_ra: self::usbdev::ARB_RW1_RA,
    #[doc = "0x21c - Endpoint Read Address value *1"]
    pub arb_rw1_ra_msb: self::usbdev::ARB_RW1_RA_MSB,
    #[doc = "0x220 - Endpoint Data Register"]
    pub arb_rw1_dr: self::usbdev::ARB_RW1_DR,
    _reserved47: [u8; 12usize],
    #[doc = "0x230 - Dedicated Endpoint Buffer Size Register *1"]
    pub buf_size: self::usbdev::BUF_SIZE,
    _reserved48: [u8; 4usize],
    #[doc = "0x238 - Endpoint Active Indication Register *1"]
    pub ep_active: self::usbdev::EP_ACTIVE,
    #[doc = "0x23c - Endpoint Type (IN/OUT) Indication *1"]
    pub ep_type: self::usbdev::EP_TYPE,
    #[doc = "0x240 - Endpoint Configuration Register *1"]
    pub arb_ep2_cfg: self::usbdev::ARB_EP2_CFG,
    #[doc = "0x244 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_int_en: self::usbdev::ARB_EP2_INT_EN,
    #[doc = "0x248 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_sr: self::usbdev::ARB_EP2_SR,
    _reserved53: [u8; 4usize],
    #[doc = "0x250 - Endpoint Write Address value *1"]
    pub arb_rw2_wa: self::usbdev::ARB_RW2_WA,
    #[doc = "0x254 - Endpoint Write Address value *1"]
    pub arb_rw2_wa_msb: self::usbdev::ARB_RW2_WA_MSB,
    #[doc = "0x258 - Endpoint Read Address value *1"]
    pub arb_rw2_ra: self::usbdev::ARB_RW2_RA,
    #[doc = "0x25c - Endpoint Read Address value *1"]
    pub arb_rw2_ra_msb: self::usbdev::ARB_RW2_RA_MSB,
    #[doc = "0x260 - Endpoint Data Register"]
    pub arb_rw2_dr: self::usbdev::ARB_RW2_DR,
    _reserved58: [u8; 12usize],
    #[doc = "0x270 - Arbiter Configuration Register *1"]
    pub arb_cfg: self::usbdev::ARB_CFG,
    #[doc = "0x274 - USB Block Clock Enable Register"]
    pub usb_clk_en: self::usbdev::USB_CLK_EN,
    #[doc = "0x278 - Arbiter Interrupt Enable *1"]
    pub arb_int_en: self::usbdev::ARB_INT_EN,
    #[doc = "0x27c - Arbiter Interrupt Status *1"]
    pub arb_int_sr: self::usbdev::ARB_INT_SR,
    #[doc = "0x280 - Endpoint Configuration Register *1"]
    pub arb_ep3_cfg: self::usbdev::ARB_EP3_CFG,
    #[doc = "0x284 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_int_en: self::usbdev::ARB_EP3_INT_EN,
    #[doc = "0x288 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_sr: self::usbdev::ARB_EP3_SR,
    _reserved65: [u8; 4usize],
    #[doc = "0x290 - Endpoint Write Address value *1"]
    pub arb_rw3_wa: self::usbdev::ARB_RW3_WA,
    #[doc = "0x294 - Endpoint Write Address value *1"]
    pub arb_rw3_wa_msb: self::usbdev::ARB_RW3_WA_MSB,
    #[doc = "0x298 - Endpoint Read Address value *1"]
    pub arb_rw3_ra: self::usbdev::ARB_RW3_RA,
    #[doc = "0x29c - Endpoint Read Address value *1"]
    pub arb_rw3_ra_msb: self::usbdev::ARB_RW3_RA_MSB,
    #[doc = "0x2a0 - Endpoint Data Register"]
    pub arb_rw3_dr: self::usbdev::ARB_RW3_DR,
    _reserved70: [u8; 12usize],
    #[doc = "0x2b0 - Common Area Write Address *1"]
    pub cwa: self::usbdev::CWA,
    #[doc = "0x2b4 - Endpoint Read Address value *1"]
    pub cwa_msb: self::usbdev::CWA_MSB,
    _reserved72: [u8; 8usize],
    #[doc = "0x2c0 - Endpoint Configuration Register *1"]
    pub arb_ep4_cfg: self::usbdev::ARB_EP4_CFG,
    #[doc = "0x2c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_int_en: self::usbdev::ARB_EP4_INT_EN,
    #[doc = "0x2c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_sr: self::usbdev::ARB_EP4_SR,
    _reserved75: [u8; 4usize],
    #[doc = "0x2d0 - Endpoint Write Address value *1"]
    pub arb_rw4_wa: self::usbdev::ARB_RW4_WA,
    #[doc = "0x2d4 - Endpoint Write Address value *1"]
    pub arb_rw4_wa_msb: self::usbdev::ARB_RW4_WA_MSB,
    #[doc = "0x2d8 - Endpoint Read Address value *1"]
    pub arb_rw4_ra: self::usbdev::ARB_RW4_RA,
    #[doc = "0x2dc - Endpoint Read Address value *1"]
    pub arb_rw4_ra_msb: self::usbdev::ARB_RW4_RA_MSB,
    #[doc = "0x2e0 - Endpoint Data Register"]
    pub arb_rw4_dr: self::usbdev::ARB_RW4_DR,
    _reserved80: [u8; 12usize],
    #[doc = "0x2f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres: self::usbdev::DMA_THRES,
    #[doc = "0x2f4 - DMA Burst / Threshold Configuration"]
    pub dma_thres_msb: self::usbdev::DMA_THRES_MSB,
    _reserved82: [u8; 8usize],
    #[doc = "0x300 - Endpoint Configuration Register *1"]
    pub arb_ep5_cfg: self::usbdev::ARB_EP5_CFG,
    #[doc = "0x304 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_int_en: self::usbdev::ARB_EP5_INT_EN,
    #[doc = "0x308 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_sr: self::usbdev::ARB_EP5_SR,
    _reserved85: [u8; 4usize],
    #[doc = "0x310 - Endpoint Write Address value *1"]
    pub arb_rw5_wa: self::usbdev::ARB_RW5_WA,
    #[doc = "0x314 - Endpoint Write Address value *1"]
    pub arb_rw5_wa_msb: self::usbdev::ARB_RW5_WA_MSB,
    #[doc = "0x318 - Endpoint Read Address value *1"]
    pub arb_rw5_ra: self::usbdev::ARB_RW5_RA,
    #[doc = "0x31c - Endpoint Read Address value *1"]
    pub arb_rw5_ra_msb: self::usbdev::ARB_RW5_RA_MSB,
    #[doc = "0x320 - Endpoint Data Register"]
    pub arb_rw5_dr: self::usbdev::ARB_RW5_DR,
    _reserved90: [u8; 12usize],
    #[doc = "0x330 - Bus Reset Count Register"]
    pub bus_rst_cnt: self::usbdev::BUS_RST_CNT,
    _reserved91: [u8; 12usize],
    #[doc = "0x340 - Endpoint Configuration Register *1"]
    pub arb_ep6_cfg: self::usbdev::ARB_EP6_CFG,
    #[doc = "0x344 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_int_en: self::usbdev::ARB_EP6_INT_EN,
    #[doc = "0x348 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_sr: self::usbdev::ARB_EP6_SR,
    _reserved94: [u8; 4usize],
    #[doc = "0x350 - Endpoint Write Address value *1"]
    pub arb_rw6_wa: self::usbdev::ARB_RW6_WA,
    #[doc = "0x354 - Endpoint Write Address value *1"]
    pub arb_rw6_wa_msb: self::usbdev::ARB_RW6_WA_MSB,
    #[doc = "0x358 - Endpoint Read Address value *1"]
    pub arb_rw6_ra: self::usbdev::ARB_RW6_RA,
    #[doc = "0x35c - Endpoint Read Address value *1"]
    pub arb_rw6_ra_msb: self::usbdev::ARB_RW6_RA_MSB,
    #[doc = "0x360 - Endpoint Data Register"]
    pub arb_rw6_dr: self::usbdev::ARB_RW6_DR,
    _reserved99: [u8; 28usize],
    #[doc = "0x380 - Endpoint Configuration Register *1"]
    pub arb_ep7_cfg: self::usbdev::ARB_EP7_CFG,
    #[doc = "0x384 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_int_en: self::usbdev::ARB_EP7_INT_EN,
    #[doc = "0x388 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_sr: self::usbdev::ARB_EP7_SR,
    _reserved102: [u8; 4usize],
    #[doc = "0x390 - Endpoint Write Address value *1"]
    pub arb_rw7_wa: self::usbdev::ARB_RW7_WA,
    #[doc = "0x394 - Endpoint Write Address value *1"]
    pub arb_rw7_wa_msb: self::usbdev::ARB_RW7_WA_MSB,
    #[doc = "0x398 - Endpoint Read Address value *1"]
    pub arb_rw7_ra: self::usbdev::ARB_RW7_RA,
    #[doc = "0x39c - Endpoint Read Address value *1"]
    pub arb_rw7_ra_msb: self::usbdev::ARB_RW7_RA_MSB,
    #[doc = "0x3a0 - Endpoint Data Register"]
    pub arb_rw7_dr: self::usbdev::ARB_RW7_DR,
    _reserved107: [u8; 28usize],
    #[doc = "0x3c0 - Endpoint Configuration Register *1"]
    pub arb_ep8_cfg: self::usbdev::ARB_EP8_CFG,
    #[doc = "0x3c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_int_en: self::usbdev::ARB_EP8_INT_EN,
    #[doc = "0x3c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_sr: self::usbdev::ARB_EP8_SR,
    _reserved110: [u8; 4usize],
    #[doc = "0x3d0 - Endpoint Write Address value *1"]
    pub arb_rw8_wa: self::usbdev::ARB_RW8_WA,
    #[doc = "0x3d4 - Endpoint Write Address value *1"]
    pub arb_rw8_wa_msb: self::usbdev::ARB_RW8_WA_MSB,
    #[doc = "0x3d8 - Endpoint Read Address value *1"]
    pub arb_rw8_ra: self::usbdev::ARB_RW8_RA,
    #[doc = "0x3dc - Endpoint Read Address value *1"]
    pub arb_rw8_ra_msb: self::usbdev::ARB_RW8_RA_MSB,
    #[doc = "0x3e0 - Endpoint Data Register"]
    pub arb_rw8_dr: self::usbdev::ARB_RW8_DR,
    _reserved115: [u8; 28usize],
    #[doc = "0x400 - DATA"]
    pub mem_data: [self::usbdev::MEM_DATA; 512],
    _reserved116: [u8; 1120usize],
    #[doc = "0x1060 - Start Of Frame Register"]
    pub sof16: self::usbdev::SOF16,
    _reserved117: [u8; 28usize],
    #[doc = "0x1080 - Oscillator lock data register"]
    pub osclk_dr16: self::usbdev::OSCLK_DR16,
    _reserved118: [u8; 396usize],
    #[doc = "0x1210 - Endpoint Write Address value"]
    pub arb_rw1_wa16: self::usbdev::ARB_RW1_WA16,
    _reserved119: [u8; 4usize],
    #[doc = "0x1218 - Endpoint Read Address value"]
    pub arb_rw1_ra16: self::usbdev::ARB_RW1_RA16,
    _reserved120: [u8; 4usize],
    #[doc = "0x1220 - Endpoint Data Register"]
    pub arb_rw1_dr16: self::usbdev::ARB_RW1_DR16,
    _reserved121: [u8; 44usize],
    #[doc = "0x1250 - Endpoint Write Address value"]
    pub arb_rw2_wa16: self::usbdev::ARB_RW2_WA16,
    _reserved122: [u8; 4usize],
    #[doc = "0x1258 - Endpoint Read Address value"]
    pub arb_rw2_ra16: self::usbdev::ARB_RW2_RA16,
    _reserved123: [u8; 4usize],
    #[doc = "0x1260 - Endpoint Data Register"]
    pub arb_rw2_dr16: self::usbdev::ARB_RW2_DR16,
    _reserved124: [u8; 44usize],
    #[doc = "0x1290 - Endpoint Write Address value"]
    pub arb_rw3_wa16: self::usbdev::ARB_RW3_WA16,
    _reserved125: [u8; 4usize],
    #[doc = "0x1298 - Endpoint Read Address value"]
    pub arb_rw3_ra16: self::usbdev::ARB_RW3_RA16,
    _reserved126: [u8; 4usize],
    #[doc = "0x12a0 - Endpoint Data Register"]
    pub arb_rw3_dr16: self::usbdev::ARB_RW3_DR16,
    _reserved127: [u8; 12usize],
    #[doc = "0x12b0 - Common Area Write Address"]
    pub cwa16: self::usbdev::CWA16,
    _reserved128: [u8; 28usize],
    #[doc = "0x12d0 - Endpoint Write Address value"]
    pub arb_rw4_wa16: self::usbdev::ARB_RW4_WA16,
    _reserved129: [u8; 4usize],
    #[doc = "0x12d8 - Endpoint Read Address value"]
    pub arb_rw4_ra16: self::usbdev::ARB_RW4_RA16,
    _reserved130: [u8; 4usize],
    #[doc = "0x12e0 - Endpoint Data Register"]
    pub arb_rw4_dr16: self::usbdev::ARB_RW4_DR16,
    _reserved131: [u8; 12usize],
    #[doc = "0x12f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres16: self::usbdev::DMA_THRES16,
    _reserved132: [u8; 28usize],
    #[doc = "0x1310 - Endpoint Write Address value"]
    pub arb_rw5_wa16: self::usbdev::ARB_RW5_WA16,
    _reserved133: [u8; 4usize],
    #[doc = "0x1318 - Endpoint Read Address value"]
    pub arb_rw5_ra16: self::usbdev::ARB_RW5_RA16,
    _reserved134: [u8; 4usize],
    #[doc = "0x1320 - Endpoint Data Register"]
    pub arb_rw5_dr16: self::usbdev::ARB_RW5_DR16,
    _reserved135: [u8; 44usize],
    #[doc = "0x1350 - Endpoint Write Address value"]
    pub arb_rw6_wa16: self::usbdev::ARB_RW6_WA16,
    _reserved136: [u8; 4usize],
    #[doc = "0x1358 - Endpoint Read Address value"]
    pub arb_rw6_ra16: self::usbdev::ARB_RW6_RA16,
    _reserved137: [u8; 4usize],
    #[doc = "0x1360 - Endpoint Data Register"]
    pub arb_rw6_dr16: self::usbdev::ARB_RW6_DR16,
    _reserved138: [u8; 44usize],
    #[doc = "0x1390 - Endpoint Write Address value"]
    pub arb_rw7_wa16: self::usbdev::ARB_RW7_WA16,
    _reserved139: [u8; 4usize],
    #[doc = "0x1398 - Endpoint Read Address value"]
    pub arb_rw7_ra16: self::usbdev::ARB_RW7_RA16,
    _reserved140: [u8; 4usize],
    #[doc = "0x13a0 - Endpoint Data Register"]
    pub arb_rw7_dr16: self::usbdev::ARB_RW7_DR16,
    _reserved141: [u8; 44usize],
    #[doc = "0x13d0 - Endpoint Write Address value"]
    pub arb_rw8_wa16: self::usbdev::ARB_RW8_WA16,
    _reserved142: [u8; 4usize],
    #[doc = "0x13d8 - Endpoint Read Address value"]
    pub arb_rw8_ra16: self::usbdev::ARB_RW8_RA16,
    _reserved143: [u8; 4usize],
    #[doc = "0x13e0 - Endpoint Data Register"]
    pub arb_rw8_dr16: self::usbdev::ARB_RW8_DR16,
}
#[doc = r"Register block"]
#[doc = "USB Device"]
pub mod usbdev;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBLPM {
    #[doc = "0x00 - Power Control Register"]
    pub power_ctl: self::usblpm::POWER_CTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - USB IO Control Register"]
    pub usbio_ctl: self::usblpm::USBIO_CTL,
    #[doc = "0x0c - Flow Control Register"]
    pub flow_ctl: self::usblpm::FLOW_CTL,
    #[doc = "0x10 - LPM Control Register"]
    pub lpm_ctl: self::usblpm::LPM_CTL,
    #[doc = "0x14 - LPM Status register"]
    pub lpm_stat: self::usblpm::LPM_STAT,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - USB SOF, BUS RESET and EP0 Interrupt Status"]
    pub intr_sie: self::usblpm::INTR_SIE,
    #[doc = "0x24 - USB SOF, BUS RESET and EP0 Interrupt Set"]
    pub intr_sie_set: self::usblpm::INTR_SIE_SET,
    #[doc = "0x28 - USB SOF, BUS RESET and EP0 Interrupt Mask"]
    pub intr_sie_mask: self::usblpm::INTR_SIE_MASK,
    #[doc = "0x2c - USB SOF, BUS RESET and EP0 Interrupt Masked"]
    pub intr_sie_masked: self::usblpm::INTR_SIE_MASKED,
    #[doc = "0x30 - Select interrupt level for each interrupt source"]
    pub intr_lvl_sel: self::usblpm::INTR_LVL_SEL,
    #[doc = "0x34 - High priority interrupt Cause register"]
    pub intr_cause_hi: self::usblpm::INTR_CAUSE_HI,
    #[doc = "0x38 - Medium priority interrupt Cause register"]
    pub intr_cause_med: self::usblpm::INTR_CAUSE_MED,
    #[doc = "0x3c - Low priority interrupt Cause register"]
    pub intr_cause_lo: self::usblpm::INTR_CAUSE_LO,
    _reserved13: [u8; 48usize],
    #[doc = "0x70 - DFT control"]
    pub dft_ctl: self::usblpm::DFT_CTL,
}
#[doc = r"Register block"]
#[doc = "USB Device LPM and PHY Test"]
pub mod usblpm;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHOST {
    #[doc = "0x00 - Host Control 0 Register."]
    pub host_ctl0: self::usbhost::HOST_CTL0,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Host Control 1 Register."]
    pub host_ctl1: self::usbhost::HOST_CTL1,
    _reserved2: [u8; 236usize],
    #[doc = "0x100 - Host Control 2 Register."]
    pub host_ctl2: self::usbhost::HOST_CTL2,
    #[doc = "0x104 - Host Error Status Register."]
    pub host_err: self::usbhost::HOST_ERR,
    #[doc = "0x108 - Host Status Register."]
    pub host_status: self::usbhost::HOST_STATUS,
    #[doc = "0x10c - Host SOF Interrupt Frame Compare Register"]
    pub host_fcomp: self::usbhost::HOST_FCOMP,
    #[doc = "0x110 - Host Retry Timer Setup Register"]
    pub host_rtimer: self::usbhost::HOST_RTIMER,
    #[doc = "0x114 - Host Address Register"]
    pub host_addr: self::usbhost::HOST_ADDR,
    #[doc = "0x118 - Host EOF Setup Register"]
    pub host_eof: self::usbhost::HOST_EOF,
    #[doc = "0x11c - Host Frame Setup Register"]
    pub host_frame: self::usbhost::HOST_FRAME,
    #[doc = "0x120 - Host Token Endpoint Register"]
    pub host_token: self::usbhost::HOST_TOKEN,
    _reserved11: [u8; 732usize],
    #[doc = "0x400 - Host Endpoint 1 Control Register"]
    pub host_ep1_ctl: self::usbhost::HOST_EP1_CTL,
    #[doc = "0x404 - Host Endpoint 1 Status Register"]
    pub host_ep1_status: self::usbhost::HOST_EP1_STATUS,
    #[doc = "0x408 - Host Endpoint 1 Data 1-Byte Register"]
    pub host_ep1_rw1_dr: self::usbhost::HOST_EP1_RW1_DR,
    #[doc = "0x40c - Host Endpoint 1 Data 2-Byte Register"]
    pub host_ep1_rw2_dr: self::usbhost::HOST_EP1_RW2_DR,
    _reserved15: [u8; 240usize],
    #[doc = "0x500 - Host Endpoint 2 Control Register"]
    pub host_ep2_ctl: self::usbhost::HOST_EP2_CTL,
    #[doc = "0x504 - Host Endpoint 2 Status Register"]
    pub host_ep2_status: self::usbhost::HOST_EP2_STATUS,
    #[doc = "0x508 - Host Endpoint 2 Data 1-Byte Register"]
    pub host_ep2_rw1_dr: self::usbhost::HOST_EP2_RW1_DR,
    #[doc = "0x50c - Host Endpoint 2 Data 2-Byte Register"]
    pub host_ep2_rw2_dr: self::usbhost::HOST_EP2_RW2_DR,
    _reserved19: [u8; 752usize],
    #[doc = "0x800 - Host Interrupt Level 1 Selection Register"]
    pub host_lvl1_sel: self::usbhost::HOST_LVL1_SEL,
    #[doc = "0x804 - Host Interrupt Level 2 Selection Register"]
    pub host_lvl2_sel: self::usbhost::HOST_LVL2_SEL,
    _reserved21: [u8; 248usize],
    #[doc = "0x900 - Interrupt USB Host Cause High Register"]
    pub intr_usbhost_cause_hi: self::usbhost::INTR_USBHOST_CAUSE_HI,
    #[doc = "0x904 - Interrupt USB Host Cause Medium Register"]
    pub intr_usbhost_cause_med: self::usbhost::INTR_USBHOST_CAUSE_MED,
    #[doc = "0x908 - Interrupt USB Host Cause Low Register"]
    pub intr_usbhost_cause_lo: self::usbhost::INTR_USBHOST_CAUSE_LO,
    _reserved24: [u8; 20usize],
    #[doc = "0x920 - Interrupt USB Host Endpoint Cause High Register"]
    pub intr_host_ep_cause_hi: self::usbhost::INTR_HOST_EP_CAUSE_HI,
    #[doc = "0x924 - Interrupt USB Host Endpoint Cause Medium Register"]
    pub intr_host_ep_cause_med: self::usbhost::INTR_HOST_EP_CAUSE_MED,
    #[doc = "0x928 - Interrupt USB Host Endpoint Cause Low Register"]
    pub intr_host_ep_cause_lo: self::usbhost::INTR_HOST_EP_CAUSE_LO,
    _reserved27: [u8; 20usize],
    #[doc = "0x940 - Interrupt USB Host Register"]
    pub intr_usbhost: self::usbhost::INTR_USBHOST,
    #[doc = "0x944 - Interrupt USB Host Set Register"]
    pub intr_usbhost_set: self::usbhost::INTR_USBHOST_SET,
    #[doc = "0x948 - Interrupt USB Host Mask Register"]
    pub intr_usbhost_mask: self::usbhost::INTR_USBHOST_MASK,
    #[doc = "0x94c - Interrupt USB Host Masked Register"]
    pub intr_usbhost_masked: self::usbhost::INTR_USBHOST_MASKED,
    _reserved31: [u8; 176usize],
    #[doc = "0xa00 - Interrupt USB Host Endpoint Register"]
    pub intr_host_ep: self::usbhost::INTR_HOST_EP,
    #[doc = "0xa04 - Interrupt USB Host Endpoint Set Register"]
    pub intr_host_ep_set: self::usbhost::INTR_HOST_EP_SET,
    #[doc = "0xa08 - Interrupt USB Host Endpoint Mask Register"]
    pub intr_host_ep_mask: self::usbhost::INTR_HOST_EP_MASK,
    #[doc = "0xa0c - Interrupt USB Host Endpoint Masked Register"]
    pub intr_host_ep_masked: self::usbhost::INTR_HOST_EP_MASKED,
    _reserved35: [u8; 240usize],
    #[doc = "0xb00 - Host DMA Enable Register"]
    pub host_dma_enbl: self::usbhost::HOST_DMA_ENBL,
    _reserved36: [u8; 28usize],
    #[doc = "0xb20 - Host Endpoint 1 Block Register"]
    pub host_ep1_blk: self::usbhost::HOST_EP1_BLK,
    _reserved37: [u8; 12usize],
    #[doc = "0xb30 - Host Endpoint 2 Block Register"]
    pub host_ep2_blk: self::usbhost::HOST_EP2_BLK,
}
#[doc = r"Register block"]
#[doc = "USB Host Controller"]
pub mod usbhost;
