#[doc = "EP0_DR register accessor: an alias for `Reg<EP0_DR_SPEC>`"]
pub type EP0_DR = crate::Reg<ep0_dr::EP0_DR_SPEC>;
#[doc = "Control End point EP0 Data Register"]
pub mod ep0_dr;
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "USB control 0 Register"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "USB control 1 Register"]
pub mod cr1;
#[doc = "SIE_EP_INT_EN register accessor: an alias for `Reg<SIE_EP_INT_EN_SPEC>`"]
pub type SIE_EP_INT_EN = crate::Reg<sie_ep_int_en::SIE_EP_INT_EN_SPEC>;
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
pub mod sie_ep_int_en;
#[doc = "SIE_EP_INT_SR register accessor: an alias for `Reg<SIE_EP_INT_SR_SPEC>`"]
pub type SIE_EP_INT_SR = crate::Reg<sie_ep_int_sr::SIE_EP_INT_SR_SPEC>;
#[doc = "USB SIE Data Endpoint Interrupt Status"]
pub mod sie_ep_int_sr;
#[doc = "SIE_EP1_CNT0 register accessor: an alias for `Reg<SIE_EP1_CNT0_SPEC>`"]
pub type SIE_EP1_CNT0 = crate::Reg<sie_ep1_cnt0::SIE_EP1_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt0;
#[doc = "SIE_EP1_CNT1 register accessor: an alias for `Reg<SIE_EP1_CNT1_SPEC>`"]
pub type SIE_EP1_CNT1 = crate::Reg<sie_ep1_cnt1::SIE_EP1_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt1;
#[doc = "SIE_EP1_CR0 register accessor: an alias for `Reg<SIE_EP1_CR0_SPEC>`"]
pub type SIE_EP1_CR0 = crate::Reg<sie_ep1_cr0::SIE_EP1_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep1_cr0;
#[doc = "USBIO_CR0 register accessor: an alias for `Reg<USBIO_CR0_SPEC>`"]
pub type USBIO_CR0 = crate::Reg<usbio_cr0::USBIO_CR0_SPEC>;
#[doc = "USBIO Control 0 Register"]
pub mod usbio_cr0;
#[doc = "USBIO_CR2 register accessor: an alias for `Reg<USBIO_CR2_SPEC>`"]
pub type USBIO_CR2 = crate::Reg<usbio_cr2::USBIO_CR2_SPEC>;
#[doc = "USBIO control 2 Register"]
pub mod usbio_cr2;
#[doc = "USBIO_CR1 register accessor: an alias for `Reg<USBIO_CR1_SPEC>`"]
pub type USBIO_CR1 = crate::Reg<usbio_cr1::USBIO_CR1_SPEC>;
#[doc = "USBIO control 1 Register"]
pub mod usbio_cr1;
#[doc = "DYN_RECONFIG register accessor: an alias for `Reg<DYN_RECONFIG_SPEC>`"]
pub type DYN_RECONFIG = crate::Reg<dyn_reconfig::DYN_RECONFIG_SPEC>;
#[doc = "USB Dynamic reconfiguration register"]
pub mod dyn_reconfig;
#[doc = "SOF0 register accessor: an alias for `Reg<SOF0_SPEC>`"]
pub type SOF0 = crate::Reg<sof0::SOF0_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof0;
#[doc = "SOF1 register accessor: an alias for `Reg<SOF1_SPEC>`"]
pub type SOF1 = crate::Reg<sof1::SOF1_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof1;
#[doc = "SIE_EP2_CNT0 register accessor: an alias for `Reg<SIE_EP2_CNT0_SPEC>`"]
pub type SIE_EP2_CNT0 = crate::Reg<sie_ep2_cnt0::SIE_EP2_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt0;
#[doc = "SIE_EP2_CNT1 register accessor: an alias for `Reg<SIE_EP2_CNT1_SPEC>`"]
pub type SIE_EP2_CNT1 = crate::Reg<sie_ep2_cnt1::SIE_EP2_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt1;
#[doc = "SIE_EP2_CR0 register accessor: an alias for `Reg<SIE_EP2_CR0_SPEC>`"]
pub type SIE_EP2_CR0 = crate::Reg<sie_ep2_cr0::SIE_EP2_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep2_cr0;
#[doc = "OSCLK_DR0 register accessor: an alias for `Reg<OSCLK_DR0_SPEC>`"]
pub type OSCLK_DR0 = crate::Reg<osclk_dr0::OSCLK_DR0_SPEC>;
#[doc = "Oscillator lock data register 0"]
pub mod osclk_dr0;
#[doc = "OSCLK_DR1 register accessor: an alias for `Reg<OSCLK_DR1_SPEC>`"]
pub type OSCLK_DR1 = crate::Reg<osclk_dr1::OSCLK_DR1_SPEC>;
#[doc = "Oscillator lock data register 1"]
pub mod osclk_dr1;
#[doc = "EP0_CR register accessor: an alias for `Reg<EP0_CR_SPEC>`"]
pub type EP0_CR = crate::Reg<ep0_cr::EP0_CR_SPEC>;
#[doc = "Endpoint0 control Register"]
pub mod ep0_cr;
#[doc = "EP0_CNT register accessor: an alias for `Reg<EP0_CNT_SPEC>`"]
pub type EP0_CNT = crate::Reg<ep0_cnt::EP0_CNT_SPEC>;
#[doc = "Endpoint0 count Register"]
pub mod ep0_cnt;
#[doc = "SIE_EP3_CNT0 register accessor: an alias for `Reg<SIE_EP3_CNT0_SPEC>`"]
pub type SIE_EP3_CNT0 = crate::Reg<sie_ep3_cnt0::SIE_EP3_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt0;
#[doc = "SIE_EP3_CNT1 register accessor: an alias for `Reg<SIE_EP3_CNT1_SPEC>`"]
pub type SIE_EP3_CNT1 = crate::Reg<sie_ep3_cnt1::SIE_EP3_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt1;
#[doc = "SIE_EP3_CR0 register accessor: an alias for `Reg<SIE_EP3_CR0_SPEC>`"]
pub type SIE_EP3_CR0 = crate::Reg<sie_ep3_cr0::SIE_EP3_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep3_cr0;
#[doc = "SIE_EP4_CNT0 register accessor: an alias for `Reg<SIE_EP4_CNT0_SPEC>`"]
pub type SIE_EP4_CNT0 = crate::Reg<sie_ep4_cnt0::SIE_EP4_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt0;
#[doc = "SIE_EP4_CNT1 register accessor: an alias for `Reg<SIE_EP4_CNT1_SPEC>`"]
pub type SIE_EP4_CNT1 = crate::Reg<sie_ep4_cnt1::SIE_EP4_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt1;
#[doc = "SIE_EP4_CR0 register accessor: an alias for `Reg<SIE_EP4_CR0_SPEC>`"]
pub type SIE_EP4_CR0 = crate::Reg<sie_ep4_cr0::SIE_EP4_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep4_cr0;
#[doc = "SIE_EP5_CNT0 register accessor: an alias for `Reg<SIE_EP5_CNT0_SPEC>`"]
pub type SIE_EP5_CNT0 = crate::Reg<sie_ep5_cnt0::SIE_EP5_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt0;
#[doc = "SIE_EP5_CNT1 register accessor: an alias for `Reg<SIE_EP5_CNT1_SPEC>`"]
pub type SIE_EP5_CNT1 = crate::Reg<sie_ep5_cnt1::SIE_EP5_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt1;
#[doc = "SIE_EP5_CR0 register accessor: an alias for `Reg<SIE_EP5_CR0_SPEC>`"]
pub type SIE_EP5_CR0 = crate::Reg<sie_ep5_cr0::SIE_EP5_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep5_cr0;
#[doc = "SIE_EP6_CNT0 register accessor: an alias for `Reg<SIE_EP6_CNT0_SPEC>`"]
pub type SIE_EP6_CNT0 = crate::Reg<sie_ep6_cnt0::SIE_EP6_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt0;
#[doc = "SIE_EP6_CNT1 register accessor: an alias for `Reg<SIE_EP6_CNT1_SPEC>`"]
pub type SIE_EP6_CNT1 = crate::Reg<sie_ep6_cnt1::SIE_EP6_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt1;
#[doc = "SIE_EP6_CR0 register accessor: an alias for `Reg<SIE_EP6_CR0_SPEC>`"]
pub type SIE_EP6_CR0 = crate::Reg<sie_ep6_cr0::SIE_EP6_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep6_cr0;
#[doc = "SIE_EP7_CNT0 register accessor: an alias for `Reg<SIE_EP7_CNT0_SPEC>`"]
pub type SIE_EP7_CNT0 = crate::Reg<sie_ep7_cnt0::SIE_EP7_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt0;
#[doc = "SIE_EP7_CNT1 register accessor: an alias for `Reg<SIE_EP7_CNT1_SPEC>`"]
pub type SIE_EP7_CNT1 = crate::Reg<sie_ep7_cnt1::SIE_EP7_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt1;
#[doc = "SIE_EP7_CR0 register accessor: an alias for `Reg<SIE_EP7_CR0_SPEC>`"]
pub type SIE_EP7_CR0 = crate::Reg<sie_ep7_cr0::SIE_EP7_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep7_cr0;
#[doc = "SIE_EP8_CNT0 register accessor: an alias for `Reg<SIE_EP8_CNT0_SPEC>`"]
pub type SIE_EP8_CNT0 = crate::Reg<sie_ep8_cnt0::SIE_EP8_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt0;
#[doc = "SIE_EP8_CNT1 register accessor: an alias for `Reg<SIE_EP8_CNT1_SPEC>`"]
pub type SIE_EP8_CNT1 = crate::Reg<sie_ep8_cnt1::SIE_EP8_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt1;
#[doc = "SIE_EP8_CR0 register accessor: an alias for `Reg<SIE_EP8_CR0_SPEC>`"]
pub type SIE_EP8_CR0 = crate::Reg<sie_ep8_cr0::SIE_EP8_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep8_cr0;
#[doc = "ARB_EP1_CFG register accessor: an alias for `Reg<ARB_EP1_CFG_SPEC>`"]
pub type ARB_EP1_CFG = crate::Reg<arb_ep1_cfg::ARB_EP1_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep1_cfg;
#[doc = "ARB_EP1_INT_EN register accessor: an alias for `Reg<ARB_EP1_INT_EN_SPEC>`"]
pub type ARB_EP1_INT_EN = crate::Reg<arb_ep1_int_en::ARB_EP1_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_int_en;
#[doc = "ARB_EP1_SR register accessor: an alias for `Reg<ARB_EP1_SR_SPEC>`"]
pub type ARB_EP1_SR = crate::Reg<arb_ep1_sr::ARB_EP1_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_sr;
#[doc = "ARB_RW1_WA register accessor: an alias for `Reg<ARB_RW1_WA_SPEC>`"]
pub type ARB_RW1_WA = crate::Reg<arb_rw1_wa::ARB_RW1_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa;
#[doc = "ARB_RW1_WA_MSB register accessor: an alias for `Reg<ARB_RW1_WA_MSB_SPEC>`"]
pub type ARB_RW1_WA_MSB = crate::Reg<arb_rw1_wa_msb::ARB_RW1_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw1_wa_msb;
#[doc = "ARB_RW1_RA register accessor: an alias for `Reg<ARB_RW1_RA_SPEC>`"]
pub type ARB_RW1_RA = crate::Reg<arb_rw1_ra::ARB_RW1_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra;
#[doc = "ARB_RW1_RA_MSB register accessor: an alias for `Reg<ARB_RW1_RA_MSB_SPEC>`"]
pub type ARB_RW1_RA_MSB = crate::Reg<arb_rw1_ra_msb::ARB_RW1_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw1_ra_msb;
#[doc = "ARB_RW1_DR register accessor: an alias for `Reg<ARB_RW1_DR_SPEC>`"]
pub type ARB_RW1_DR = crate::Reg<arb_rw1_dr::ARB_RW1_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr;
#[doc = "BUF_SIZE register accessor: an alias for `Reg<BUF_SIZE_SPEC>`"]
pub type BUF_SIZE = crate::Reg<buf_size::BUF_SIZE_SPEC>;
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
pub mod buf_size;
#[doc = "EP_ACTIVE register accessor: an alias for `Reg<EP_ACTIVE_SPEC>`"]
pub type EP_ACTIVE = crate::Reg<ep_active::EP_ACTIVE_SPEC>;
#[doc = "Endpoint Active Indication Register *1"]
pub mod ep_active;
#[doc = "EP_TYPE register accessor: an alias for `Reg<EP_TYPE_SPEC>`"]
pub type EP_TYPE = crate::Reg<ep_type::EP_TYPE_SPEC>;
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
pub mod ep_type;
#[doc = "ARB_EP2_CFG register accessor: an alias for `Reg<ARB_EP2_CFG_SPEC>`"]
pub type ARB_EP2_CFG = crate::Reg<arb_ep2_cfg::ARB_EP2_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep2_cfg;
#[doc = "ARB_EP2_INT_EN register accessor: an alias for `Reg<ARB_EP2_INT_EN_SPEC>`"]
pub type ARB_EP2_INT_EN = crate::Reg<arb_ep2_int_en::ARB_EP2_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_int_en;
#[doc = "ARB_EP2_SR register accessor: an alias for `Reg<ARB_EP2_SR_SPEC>`"]
pub type ARB_EP2_SR = crate::Reg<arb_ep2_sr::ARB_EP2_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_sr;
#[doc = "ARB_RW2_WA register accessor: an alias for `Reg<ARB_RW2_WA_SPEC>`"]
pub type ARB_RW2_WA = crate::Reg<arb_rw2_wa::ARB_RW2_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa;
#[doc = "ARB_RW2_WA_MSB register accessor: an alias for `Reg<ARB_RW2_WA_MSB_SPEC>`"]
pub type ARB_RW2_WA_MSB = crate::Reg<arb_rw2_wa_msb::ARB_RW2_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw2_wa_msb;
#[doc = "ARB_RW2_RA register accessor: an alias for `Reg<ARB_RW2_RA_SPEC>`"]
pub type ARB_RW2_RA = crate::Reg<arb_rw2_ra::ARB_RW2_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra;
#[doc = "ARB_RW2_RA_MSB register accessor: an alias for `Reg<ARB_RW2_RA_MSB_SPEC>`"]
pub type ARB_RW2_RA_MSB = crate::Reg<arb_rw2_ra_msb::ARB_RW2_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw2_ra_msb;
#[doc = "ARB_RW2_DR register accessor: an alias for `Reg<ARB_RW2_DR_SPEC>`"]
pub type ARB_RW2_DR = crate::Reg<arb_rw2_dr::ARB_RW2_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr;
#[doc = "ARB_CFG register accessor: an alias for `Reg<ARB_CFG_SPEC>`"]
pub type ARB_CFG = crate::Reg<arb_cfg::ARB_CFG_SPEC>;
#[doc = "Arbiter Configuration Register *1"]
pub mod arb_cfg;
#[doc = "USB_CLK_EN register accessor: an alias for `Reg<USB_CLK_EN_SPEC>`"]
pub type USB_CLK_EN = crate::Reg<usb_clk_en::USB_CLK_EN_SPEC>;
#[doc = "USB Block Clock Enable Register"]
pub mod usb_clk_en;
#[doc = "ARB_INT_EN register accessor: an alias for `Reg<ARB_INT_EN_SPEC>`"]
pub type ARB_INT_EN = crate::Reg<arb_int_en::ARB_INT_EN_SPEC>;
#[doc = "Arbiter Interrupt Enable *1"]
pub mod arb_int_en;
#[doc = "ARB_INT_SR register accessor: an alias for `Reg<ARB_INT_SR_SPEC>`"]
pub type ARB_INT_SR = crate::Reg<arb_int_sr::ARB_INT_SR_SPEC>;
#[doc = "Arbiter Interrupt Status *1"]
pub mod arb_int_sr;
#[doc = "ARB_EP3_CFG register accessor: an alias for `Reg<ARB_EP3_CFG_SPEC>`"]
pub type ARB_EP3_CFG = crate::Reg<arb_ep3_cfg::ARB_EP3_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep3_cfg;
#[doc = "ARB_EP3_INT_EN register accessor: an alias for `Reg<ARB_EP3_INT_EN_SPEC>`"]
pub type ARB_EP3_INT_EN = crate::Reg<arb_ep3_int_en::ARB_EP3_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_int_en;
#[doc = "ARB_EP3_SR register accessor: an alias for `Reg<ARB_EP3_SR_SPEC>`"]
pub type ARB_EP3_SR = crate::Reg<arb_ep3_sr::ARB_EP3_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_sr;
#[doc = "ARB_RW3_WA register accessor: an alias for `Reg<ARB_RW3_WA_SPEC>`"]
pub type ARB_RW3_WA = crate::Reg<arb_rw3_wa::ARB_RW3_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa;
#[doc = "ARB_RW3_WA_MSB register accessor: an alias for `Reg<ARB_RW3_WA_MSB_SPEC>`"]
pub type ARB_RW3_WA_MSB = crate::Reg<arb_rw3_wa_msb::ARB_RW3_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw3_wa_msb;
#[doc = "ARB_RW3_RA register accessor: an alias for `Reg<ARB_RW3_RA_SPEC>`"]
pub type ARB_RW3_RA = crate::Reg<arb_rw3_ra::ARB_RW3_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra;
#[doc = "ARB_RW3_RA_MSB register accessor: an alias for `Reg<ARB_RW3_RA_MSB_SPEC>`"]
pub type ARB_RW3_RA_MSB = crate::Reg<arb_rw3_ra_msb::ARB_RW3_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw3_ra_msb;
#[doc = "ARB_RW3_DR register accessor: an alias for `Reg<ARB_RW3_DR_SPEC>`"]
pub type ARB_RW3_DR = crate::Reg<arb_rw3_dr::ARB_RW3_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr;
#[doc = "CWA register accessor: an alias for `Reg<CWA_SPEC>`"]
pub type CWA = crate::Reg<cwa::CWA_SPEC>;
#[doc = "Common Area Write Address *1"]
pub mod cwa;
#[doc = "CWA_MSB register accessor: an alias for `Reg<CWA_MSB_SPEC>`"]
pub type CWA_MSB = crate::Reg<cwa_msb::CWA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod cwa_msb;
#[doc = "ARB_EP4_CFG register accessor: an alias for `Reg<ARB_EP4_CFG_SPEC>`"]
pub type ARB_EP4_CFG = crate::Reg<arb_ep4_cfg::ARB_EP4_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep4_cfg;
#[doc = "ARB_EP4_INT_EN register accessor: an alias for `Reg<ARB_EP4_INT_EN_SPEC>`"]
pub type ARB_EP4_INT_EN = crate::Reg<arb_ep4_int_en::ARB_EP4_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_int_en;
#[doc = "ARB_EP4_SR register accessor: an alias for `Reg<ARB_EP4_SR_SPEC>`"]
pub type ARB_EP4_SR = crate::Reg<arb_ep4_sr::ARB_EP4_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_sr;
#[doc = "ARB_RW4_WA register accessor: an alias for `Reg<ARB_RW4_WA_SPEC>`"]
pub type ARB_RW4_WA = crate::Reg<arb_rw4_wa::ARB_RW4_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa;
#[doc = "ARB_RW4_WA_MSB register accessor: an alias for `Reg<ARB_RW4_WA_MSB_SPEC>`"]
pub type ARB_RW4_WA_MSB = crate::Reg<arb_rw4_wa_msb::ARB_RW4_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw4_wa_msb;
#[doc = "ARB_RW4_RA register accessor: an alias for `Reg<ARB_RW4_RA_SPEC>`"]
pub type ARB_RW4_RA = crate::Reg<arb_rw4_ra::ARB_RW4_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra;
#[doc = "ARB_RW4_RA_MSB register accessor: an alias for `Reg<ARB_RW4_RA_MSB_SPEC>`"]
pub type ARB_RW4_RA_MSB = crate::Reg<arb_rw4_ra_msb::ARB_RW4_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw4_ra_msb;
#[doc = "ARB_RW4_DR register accessor: an alias for `Reg<ARB_RW4_DR_SPEC>`"]
pub type ARB_RW4_DR = crate::Reg<arb_rw4_dr::ARB_RW4_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr;
#[doc = "DMA_THRES register accessor: an alias for `Reg<DMA_THRES_SPEC>`"]
pub type DMA_THRES = crate::Reg<dma_thres::DMA_THRES_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres;
#[doc = "DMA_THRES_MSB register accessor: an alias for `Reg<DMA_THRES_MSB_SPEC>`"]
pub type DMA_THRES_MSB = crate::Reg<dma_thres_msb::DMA_THRES_MSB_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres_msb;
#[doc = "ARB_EP5_CFG register accessor: an alias for `Reg<ARB_EP5_CFG_SPEC>`"]
pub type ARB_EP5_CFG = crate::Reg<arb_ep5_cfg::ARB_EP5_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep5_cfg;
#[doc = "ARB_EP5_INT_EN register accessor: an alias for `Reg<ARB_EP5_INT_EN_SPEC>`"]
pub type ARB_EP5_INT_EN = crate::Reg<arb_ep5_int_en::ARB_EP5_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_int_en;
#[doc = "ARB_EP5_SR register accessor: an alias for `Reg<ARB_EP5_SR_SPEC>`"]
pub type ARB_EP5_SR = crate::Reg<arb_ep5_sr::ARB_EP5_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_sr;
#[doc = "ARB_RW5_WA register accessor: an alias for `Reg<ARB_RW5_WA_SPEC>`"]
pub type ARB_RW5_WA = crate::Reg<arb_rw5_wa::ARB_RW5_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa;
#[doc = "ARB_RW5_WA_MSB register accessor: an alias for `Reg<ARB_RW5_WA_MSB_SPEC>`"]
pub type ARB_RW5_WA_MSB = crate::Reg<arb_rw5_wa_msb::ARB_RW5_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw5_wa_msb;
#[doc = "ARB_RW5_RA register accessor: an alias for `Reg<ARB_RW5_RA_SPEC>`"]
pub type ARB_RW5_RA = crate::Reg<arb_rw5_ra::ARB_RW5_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra;
#[doc = "ARB_RW5_RA_MSB register accessor: an alias for `Reg<ARB_RW5_RA_MSB_SPEC>`"]
pub type ARB_RW5_RA_MSB = crate::Reg<arb_rw5_ra_msb::ARB_RW5_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw5_ra_msb;
#[doc = "ARB_RW5_DR register accessor: an alias for `Reg<ARB_RW5_DR_SPEC>`"]
pub type ARB_RW5_DR = crate::Reg<arb_rw5_dr::ARB_RW5_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr;
#[doc = "BUS_RST_CNT register accessor: an alias for `Reg<BUS_RST_CNT_SPEC>`"]
pub type BUS_RST_CNT = crate::Reg<bus_rst_cnt::BUS_RST_CNT_SPEC>;
#[doc = "Bus Reset Count Register"]
pub mod bus_rst_cnt;
#[doc = "ARB_EP6_CFG register accessor: an alias for `Reg<ARB_EP6_CFG_SPEC>`"]
pub type ARB_EP6_CFG = crate::Reg<arb_ep6_cfg::ARB_EP6_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep6_cfg;
#[doc = "ARB_EP6_INT_EN register accessor: an alias for `Reg<ARB_EP6_INT_EN_SPEC>`"]
pub type ARB_EP6_INT_EN = crate::Reg<arb_ep6_int_en::ARB_EP6_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_int_en;
#[doc = "ARB_EP6_SR register accessor: an alias for `Reg<ARB_EP6_SR_SPEC>`"]
pub type ARB_EP6_SR = crate::Reg<arb_ep6_sr::ARB_EP6_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_sr;
#[doc = "ARB_RW6_WA register accessor: an alias for `Reg<ARB_RW6_WA_SPEC>`"]
pub type ARB_RW6_WA = crate::Reg<arb_rw6_wa::ARB_RW6_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa;
#[doc = "ARB_RW6_WA_MSB register accessor: an alias for `Reg<ARB_RW6_WA_MSB_SPEC>`"]
pub type ARB_RW6_WA_MSB = crate::Reg<arb_rw6_wa_msb::ARB_RW6_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw6_wa_msb;
#[doc = "ARB_RW6_RA register accessor: an alias for `Reg<ARB_RW6_RA_SPEC>`"]
pub type ARB_RW6_RA = crate::Reg<arb_rw6_ra::ARB_RW6_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra;
#[doc = "ARB_RW6_RA_MSB register accessor: an alias for `Reg<ARB_RW6_RA_MSB_SPEC>`"]
pub type ARB_RW6_RA_MSB = crate::Reg<arb_rw6_ra_msb::ARB_RW6_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw6_ra_msb;
#[doc = "ARB_RW6_DR register accessor: an alias for `Reg<ARB_RW6_DR_SPEC>`"]
pub type ARB_RW6_DR = crate::Reg<arb_rw6_dr::ARB_RW6_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr;
#[doc = "ARB_EP7_CFG register accessor: an alias for `Reg<ARB_EP7_CFG_SPEC>`"]
pub type ARB_EP7_CFG = crate::Reg<arb_ep7_cfg::ARB_EP7_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep7_cfg;
#[doc = "ARB_EP7_INT_EN register accessor: an alias for `Reg<ARB_EP7_INT_EN_SPEC>`"]
pub type ARB_EP7_INT_EN = crate::Reg<arb_ep7_int_en::ARB_EP7_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_int_en;
#[doc = "ARB_EP7_SR register accessor: an alias for `Reg<ARB_EP7_SR_SPEC>`"]
pub type ARB_EP7_SR = crate::Reg<arb_ep7_sr::ARB_EP7_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_sr;
#[doc = "ARB_RW7_WA register accessor: an alias for `Reg<ARB_RW7_WA_SPEC>`"]
pub type ARB_RW7_WA = crate::Reg<arb_rw7_wa::ARB_RW7_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa;
#[doc = "ARB_RW7_WA_MSB register accessor: an alias for `Reg<ARB_RW7_WA_MSB_SPEC>`"]
pub type ARB_RW7_WA_MSB = crate::Reg<arb_rw7_wa_msb::ARB_RW7_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw7_wa_msb;
#[doc = "ARB_RW7_RA register accessor: an alias for `Reg<ARB_RW7_RA_SPEC>`"]
pub type ARB_RW7_RA = crate::Reg<arb_rw7_ra::ARB_RW7_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra;
#[doc = "ARB_RW7_RA_MSB register accessor: an alias for `Reg<ARB_RW7_RA_MSB_SPEC>`"]
pub type ARB_RW7_RA_MSB = crate::Reg<arb_rw7_ra_msb::ARB_RW7_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw7_ra_msb;
#[doc = "ARB_RW7_DR register accessor: an alias for `Reg<ARB_RW7_DR_SPEC>`"]
pub type ARB_RW7_DR = crate::Reg<arb_rw7_dr::ARB_RW7_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr;
#[doc = "ARB_EP8_CFG register accessor: an alias for `Reg<ARB_EP8_CFG_SPEC>`"]
pub type ARB_EP8_CFG = crate::Reg<arb_ep8_cfg::ARB_EP8_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep8_cfg;
#[doc = "ARB_EP8_INT_EN register accessor: an alias for `Reg<ARB_EP8_INT_EN_SPEC>`"]
pub type ARB_EP8_INT_EN = crate::Reg<arb_ep8_int_en::ARB_EP8_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_int_en;
#[doc = "ARB_EP8_SR register accessor: an alias for `Reg<ARB_EP8_SR_SPEC>`"]
pub type ARB_EP8_SR = crate::Reg<arb_ep8_sr::ARB_EP8_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_sr;
#[doc = "ARB_RW8_WA register accessor: an alias for `Reg<ARB_RW8_WA_SPEC>`"]
pub type ARB_RW8_WA = crate::Reg<arb_rw8_wa::ARB_RW8_WA_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa;
#[doc = "ARB_RW8_WA_MSB register accessor: an alias for `Reg<ARB_RW8_WA_MSB_SPEC>`"]
pub type ARB_RW8_WA_MSB = crate::Reg<arb_rw8_wa_msb::ARB_RW8_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1"]
pub mod arb_rw8_wa_msb;
#[doc = "ARB_RW8_RA register accessor: an alias for `Reg<ARB_RW8_RA_SPEC>`"]
pub type ARB_RW8_RA = crate::Reg<arb_rw8_ra::ARB_RW8_RA_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra;
#[doc = "ARB_RW8_RA_MSB register accessor: an alias for `Reg<ARB_RW8_RA_MSB_SPEC>`"]
pub type ARB_RW8_RA_MSB = crate::Reg<arb_rw8_ra_msb::ARB_RW8_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod arb_rw8_ra_msb;
#[doc = "ARB_RW8_DR register accessor: an alias for `Reg<ARB_RW8_DR_SPEC>`"]
pub type ARB_RW8_DR = crate::Reg<arb_rw8_dr::ARB_RW8_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr;
#[doc = "MEM_DATA register accessor: an alias for `Reg<MEM_DATA_SPEC>`"]
pub type MEM_DATA = crate::Reg<mem_data::MEM_DATA_SPEC>;
#[doc = "DATA"]
pub mod mem_data;
#[doc = "SOF16 register accessor: an alias for `Reg<SOF16_SPEC>`"]
pub type SOF16 = crate::Reg<sof16::SOF16_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof16;
#[doc = "OSCLK_DR16 register accessor: an alias for `Reg<OSCLK_DR16_SPEC>`"]
pub type OSCLK_DR16 = crate::Reg<osclk_dr16::OSCLK_DR16_SPEC>;
#[doc = "Oscillator lock data register"]
pub mod osclk_dr16;
#[doc = "ARB_RW1_WA16 register accessor: an alias for `Reg<ARB_RW1_WA16_SPEC>`"]
pub type ARB_RW1_WA16 = crate::Reg<arb_rw1_wa16::ARB_RW1_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw1_wa16;
#[doc = "ARB_RW1_RA16 register accessor: an alias for `Reg<ARB_RW1_RA16_SPEC>`"]
pub type ARB_RW1_RA16 = crate::Reg<arb_rw1_ra16::ARB_RW1_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw1_ra16;
#[doc = "ARB_RW1_DR16 register accessor: an alias for `Reg<ARB_RW1_DR16_SPEC>`"]
pub type ARB_RW1_DR16 = crate::Reg<arb_rw1_dr16::ARB_RW1_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr16;
#[doc = "ARB_RW2_WA16 register accessor: an alias for `Reg<ARB_RW2_WA16_SPEC>`"]
pub type ARB_RW2_WA16 = crate::Reg<arb_rw2_wa16::ARB_RW2_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw2_wa16;
#[doc = "ARB_RW2_RA16 register accessor: an alias for `Reg<ARB_RW2_RA16_SPEC>`"]
pub type ARB_RW2_RA16 = crate::Reg<arb_rw2_ra16::ARB_RW2_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw2_ra16;
#[doc = "ARB_RW2_DR16 register accessor: an alias for `Reg<ARB_RW2_DR16_SPEC>`"]
pub type ARB_RW2_DR16 = crate::Reg<arb_rw2_dr16::ARB_RW2_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr16;
#[doc = "ARB_RW3_WA16 register accessor: an alias for `Reg<ARB_RW3_WA16_SPEC>`"]
pub type ARB_RW3_WA16 = crate::Reg<arb_rw3_wa16::ARB_RW3_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw3_wa16;
#[doc = "ARB_RW3_RA16 register accessor: an alias for `Reg<ARB_RW3_RA16_SPEC>`"]
pub type ARB_RW3_RA16 = crate::Reg<arb_rw3_ra16::ARB_RW3_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw3_ra16;
#[doc = "ARB_RW3_DR16 register accessor: an alias for `Reg<ARB_RW3_DR16_SPEC>`"]
pub type ARB_RW3_DR16 = crate::Reg<arb_rw3_dr16::ARB_RW3_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr16;
#[doc = "CWA16 register accessor: an alias for `Reg<CWA16_SPEC>`"]
pub type CWA16 = crate::Reg<cwa16::CWA16_SPEC>;
#[doc = "Common Area Write Address"]
pub mod cwa16;
#[doc = "ARB_RW4_WA16 register accessor: an alias for `Reg<ARB_RW4_WA16_SPEC>`"]
pub type ARB_RW4_WA16 = crate::Reg<arb_rw4_wa16::ARB_RW4_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw4_wa16;
#[doc = "ARB_RW4_RA16 register accessor: an alias for `Reg<ARB_RW4_RA16_SPEC>`"]
pub type ARB_RW4_RA16 = crate::Reg<arb_rw4_ra16::ARB_RW4_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw4_ra16;
#[doc = "ARB_RW4_DR16 register accessor: an alias for `Reg<ARB_RW4_DR16_SPEC>`"]
pub type ARB_RW4_DR16 = crate::Reg<arb_rw4_dr16::ARB_RW4_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr16;
#[doc = "DMA_THRES16 register accessor: an alias for `Reg<DMA_THRES16_SPEC>`"]
pub type DMA_THRES16 = crate::Reg<dma_thres16::DMA_THRES16_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres16;
#[doc = "ARB_RW5_WA16 register accessor: an alias for `Reg<ARB_RW5_WA16_SPEC>`"]
pub type ARB_RW5_WA16 = crate::Reg<arb_rw5_wa16::ARB_RW5_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw5_wa16;
#[doc = "ARB_RW5_RA16 register accessor: an alias for `Reg<ARB_RW5_RA16_SPEC>`"]
pub type ARB_RW5_RA16 = crate::Reg<arb_rw5_ra16::ARB_RW5_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw5_ra16;
#[doc = "ARB_RW5_DR16 register accessor: an alias for `Reg<ARB_RW5_DR16_SPEC>`"]
pub type ARB_RW5_DR16 = crate::Reg<arb_rw5_dr16::ARB_RW5_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr16;
#[doc = "ARB_RW6_WA16 register accessor: an alias for `Reg<ARB_RW6_WA16_SPEC>`"]
pub type ARB_RW6_WA16 = crate::Reg<arb_rw6_wa16::ARB_RW6_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw6_wa16;
#[doc = "ARB_RW6_RA16 register accessor: an alias for `Reg<ARB_RW6_RA16_SPEC>`"]
pub type ARB_RW6_RA16 = crate::Reg<arb_rw6_ra16::ARB_RW6_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw6_ra16;
#[doc = "ARB_RW6_DR16 register accessor: an alias for `Reg<ARB_RW6_DR16_SPEC>`"]
pub type ARB_RW6_DR16 = crate::Reg<arb_rw6_dr16::ARB_RW6_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr16;
#[doc = "ARB_RW7_WA16 register accessor: an alias for `Reg<ARB_RW7_WA16_SPEC>`"]
pub type ARB_RW7_WA16 = crate::Reg<arb_rw7_wa16::ARB_RW7_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw7_wa16;
#[doc = "ARB_RW7_RA16 register accessor: an alias for `Reg<ARB_RW7_RA16_SPEC>`"]
pub type ARB_RW7_RA16 = crate::Reg<arb_rw7_ra16::ARB_RW7_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw7_ra16;
#[doc = "ARB_RW7_DR16 register accessor: an alias for `Reg<ARB_RW7_DR16_SPEC>`"]
pub type ARB_RW7_DR16 = crate::Reg<arb_rw7_dr16::ARB_RW7_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr16;
#[doc = "ARB_RW8_WA16 register accessor: an alias for `Reg<ARB_RW8_WA16_SPEC>`"]
pub type ARB_RW8_WA16 = crate::Reg<arb_rw8_wa16::ARB_RW8_WA16_SPEC>;
#[doc = "Endpoint Write Address value"]
pub mod arb_rw8_wa16;
#[doc = "ARB_RW8_RA16 register accessor: an alias for `Reg<ARB_RW8_RA16_SPEC>`"]
pub type ARB_RW8_RA16 = crate::Reg<arb_rw8_ra16::ARB_RW8_RA16_SPEC>;
#[doc = "Endpoint Read Address value"]
pub mod arb_rw8_ra16;
#[doc = "ARB_RW8_DR16 register accessor: an alias for `Reg<ARB_RW8_DR16_SPEC>`"]
pub type ARB_RW8_DR16 = crate::Reg<arb_rw8_dr16::ARB_RW8_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr16;
