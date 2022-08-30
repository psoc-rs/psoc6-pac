#[doc = "HOST_CTL0 register accessor: an alias for `Reg<HOST_CTL0_SPEC>`"]
pub type HOST_CTL0 = crate::Reg<host_ctl0::HOST_CTL0_SPEC>;
#[doc = "Host Control 0 Register."]
pub mod host_ctl0;
#[doc = "HOST_CTL1 register accessor: an alias for `Reg<HOST_CTL1_SPEC>`"]
pub type HOST_CTL1 = crate::Reg<host_ctl1::HOST_CTL1_SPEC>;
#[doc = "Host Control 1 Register."]
pub mod host_ctl1;
#[doc = "HOST_CTL2 register accessor: an alias for `Reg<HOST_CTL2_SPEC>`"]
pub type HOST_CTL2 = crate::Reg<host_ctl2::HOST_CTL2_SPEC>;
#[doc = "Host Control 2 Register."]
pub mod host_ctl2;
#[doc = "HOST_ERR register accessor: an alias for `Reg<HOST_ERR_SPEC>`"]
pub type HOST_ERR = crate::Reg<host_err::HOST_ERR_SPEC>;
#[doc = "Host Error Status Register."]
pub mod host_err;
#[doc = "HOST_STATUS register accessor: an alias for `Reg<HOST_STATUS_SPEC>`"]
pub type HOST_STATUS = crate::Reg<host_status::HOST_STATUS_SPEC>;
#[doc = "Host Status Register."]
pub mod host_status;
#[doc = "HOST_FCOMP register accessor: an alias for `Reg<HOST_FCOMP_SPEC>`"]
pub type HOST_FCOMP = crate::Reg<host_fcomp::HOST_FCOMP_SPEC>;
#[doc = "Host SOF Interrupt Frame Compare Register"]
pub mod host_fcomp;
#[doc = "HOST_RTIMER register accessor: an alias for `Reg<HOST_RTIMER_SPEC>`"]
pub type HOST_RTIMER = crate::Reg<host_rtimer::HOST_RTIMER_SPEC>;
#[doc = "Host Retry Timer Setup Register"]
pub mod host_rtimer;
#[doc = "HOST_ADDR register accessor: an alias for `Reg<HOST_ADDR_SPEC>`"]
pub type HOST_ADDR = crate::Reg<host_addr::HOST_ADDR_SPEC>;
#[doc = "Host Address Register"]
pub mod host_addr;
#[doc = "HOST_EOF register accessor: an alias for `Reg<HOST_EOF_SPEC>`"]
pub type HOST_EOF = crate::Reg<host_eof::HOST_EOF_SPEC>;
#[doc = "Host EOF Setup Register"]
pub mod host_eof;
#[doc = "HOST_FRAME register accessor: an alias for `Reg<HOST_FRAME_SPEC>`"]
pub type HOST_FRAME = crate::Reg<host_frame::HOST_FRAME_SPEC>;
#[doc = "Host Frame Setup Register"]
pub mod host_frame;
#[doc = "HOST_TOKEN register accessor: an alias for `Reg<HOST_TOKEN_SPEC>`"]
pub type HOST_TOKEN = crate::Reg<host_token::HOST_TOKEN_SPEC>;
#[doc = "Host Token Endpoint Register"]
pub mod host_token;
#[doc = "HOST_EP1_CTL register accessor: an alias for `Reg<HOST_EP1_CTL_SPEC>`"]
pub type HOST_EP1_CTL = crate::Reg<host_ep1_ctl::HOST_EP1_CTL_SPEC>;
#[doc = "Host Endpoint 1 Control Register"]
pub mod host_ep1_ctl;
#[doc = "HOST_EP1_STATUS register accessor: an alias for `Reg<HOST_EP1_STATUS_SPEC>`"]
pub type HOST_EP1_STATUS = crate::Reg<host_ep1_status::HOST_EP1_STATUS_SPEC>;
#[doc = "Host Endpoint 1 Status Register"]
pub mod host_ep1_status;
#[doc = "HOST_EP1_RW1_DR register accessor: an alias for `Reg<HOST_EP1_RW1_DR_SPEC>`"]
pub type HOST_EP1_RW1_DR = crate::Reg<host_ep1_rw1_dr::HOST_EP1_RW1_DR_SPEC>;
#[doc = "Host Endpoint 1 Data 1-Byte Register"]
pub mod host_ep1_rw1_dr;
#[doc = "HOST_EP1_RW2_DR register accessor: an alias for `Reg<HOST_EP1_RW2_DR_SPEC>`"]
pub type HOST_EP1_RW2_DR = crate::Reg<host_ep1_rw2_dr::HOST_EP1_RW2_DR_SPEC>;
#[doc = "Host Endpoint 1 Data 2-Byte Register"]
pub mod host_ep1_rw2_dr;
#[doc = "HOST_EP2_CTL register accessor: an alias for `Reg<HOST_EP2_CTL_SPEC>`"]
pub type HOST_EP2_CTL = crate::Reg<host_ep2_ctl::HOST_EP2_CTL_SPEC>;
#[doc = "Host Endpoint 2 Control Register"]
pub mod host_ep2_ctl;
#[doc = "HOST_EP2_STATUS register accessor: an alias for `Reg<HOST_EP2_STATUS_SPEC>`"]
pub type HOST_EP2_STATUS = crate::Reg<host_ep2_status::HOST_EP2_STATUS_SPEC>;
#[doc = "Host Endpoint 2 Status Register"]
pub mod host_ep2_status;
#[doc = "HOST_EP2_RW1_DR register accessor: an alias for `Reg<HOST_EP2_RW1_DR_SPEC>`"]
pub type HOST_EP2_RW1_DR = crate::Reg<host_ep2_rw1_dr::HOST_EP2_RW1_DR_SPEC>;
#[doc = "Host Endpoint 2 Data 1-Byte Register"]
pub mod host_ep2_rw1_dr;
#[doc = "HOST_EP2_RW2_DR register accessor: an alias for `Reg<HOST_EP2_RW2_DR_SPEC>`"]
pub type HOST_EP2_RW2_DR = crate::Reg<host_ep2_rw2_dr::HOST_EP2_RW2_DR_SPEC>;
#[doc = "Host Endpoint 2 Data 2-Byte Register"]
pub mod host_ep2_rw2_dr;
#[doc = "HOST_LVL1_SEL register accessor: an alias for `Reg<HOST_LVL1_SEL_SPEC>`"]
pub type HOST_LVL1_SEL = crate::Reg<host_lvl1_sel::HOST_LVL1_SEL_SPEC>;
#[doc = "Host Interrupt Level 1 Selection Register"]
pub mod host_lvl1_sel;
#[doc = "HOST_LVL2_SEL register accessor: an alias for `Reg<HOST_LVL2_SEL_SPEC>`"]
pub type HOST_LVL2_SEL = crate::Reg<host_lvl2_sel::HOST_LVL2_SEL_SPEC>;
#[doc = "Host Interrupt Level 2 Selection Register"]
pub mod host_lvl2_sel;
#[doc = "INTR_USBHOST_CAUSE_HI register accessor: an alias for `Reg<INTR_USBHOST_CAUSE_HI_SPEC>`"]
pub type INTR_USBHOST_CAUSE_HI = crate::Reg<intr_usbhost_cause_hi::INTR_USBHOST_CAUSE_HI_SPEC>;
#[doc = "Interrupt USB Host Cause High Register"]
pub mod intr_usbhost_cause_hi;
#[doc = "INTR_USBHOST_CAUSE_MED register accessor: an alias for `Reg<INTR_USBHOST_CAUSE_MED_SPEC>`"]
pub type INTR_USBHOST_CAUSE_MED = crate::Reg<intr_usbhost_cause_med::INTR_USBHOST_CAUSE_MED_SPEC>;
#[doc = "Interrupt USB Host Cause Medium Register"]
pub mod intr_usbhost_cause_med;
#[doc = "INTR_USBHOST_CAUSE_LO register accessor: an alias for `Reg<INTR_USBHOST_CAUSE_LO_SPEC>`"]
pub type INTR_USBHOST_CAUSE_LO = crate::Reg<intr_usbhost_cause_lo::INTR_USBHOST_CAUSE_LO_SPEC>;
#[doc = "Interrupt USB Host Cause Low Register"]
pub mod intr_usbhost_cause_lo;
#[doc = "INTR_HOST_EP_CAUSE_HI register accessor: an alias for `Reg<INTR_HOST_EP_CAUSE_HI_SPEC>`"]
pub type INTR_HOST_EP_CAUSE_HI = crate::Reg<intr_host_ep_cause_hi::INTR_HOST_EP_CAUSE_HI_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause High Register"]
pub mod intr_host_ep_cause_hi;
#[doc = "INTR_HOST_EP_CAUSE_MED register accessor: an alias for `Reg<INTR_HOST_EP_CAUSE_MED_SPEC>`"]
pub type INTR_HOST_EP_CAUSE_MED = crate::Reg<intr_host_ep_cause_med::INTR_HOST_EP_CAUSE_MED_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
pub mod intr_host_ep_cause_med;
#[doc = "INTR_HOST_EP_CAUSE_LO register accessor: an alias for `Reg<INTR_HOST_EP_CAUSE_LO_SPEC>`"]
pub type INTR_HOST_EP_CAUSE_LO = crate::Reg<intr_host_ep_cause_lo::INTR_HOST_EP_CAUSE_LO_SPEC>;
#[doc = "Interrupt USB Host Endpoint Cause Low Register"]
pub mod intr_host_ep_cause_lo;
#[doc = "INTR_USBHOST register accessor: an alias for `Reg<INTR_USBHOST_SPEC>`"]
pub type INTR_USBHOST = crate::Reg<intr_usbhost::INTR_USBHOST_SPEC>;
#[doc = "Interrupt USB Host Register"]
pub mod intr_usbhost;
#[doc = "INTR_USBHOST_SET register accessor: an alias for `Reg<INTR_USBHOST_SET_SPEC>`"]
pub type INTR_USBHOST_SET = crate::Reg<intr_usbhost_set::INTR_USBHOST_SET_SPEC>;
#[doc = "Interrupt USB Host Set Register"]
pub mod intr_usbhost_set;
#[doc = "INTR_USBHOST_MASK register accessor: an alias for `Reg<INTR_USBHOST_MASK_SPEC>`"]
pub type INTR_USBHOST_MASK = crate::Reg<intr_usbhost_mask::INTR_USBHOST_MASK_SPEC>;
#[doc = "Interrupt USB Host Mask Register"]
pub mod intr_usbhost_mask;
#[doc = "INTR_USBHOST_MASKED register accessor: an alias for `Reg<INTR_USBHOST_MASKED_SPEC>`"]
pub type INTR_USBHOST_MASKED = crate::Reg<intr_usbhost_masked::INTR_USBHOST_MASKED_SPEC>;
#[doc = "Interrupt USB Host Masked Register"]
pub mod intr_usbhost_masked;
#[doc = "INTR_HOST_EP register accessor: an alias for `Reg<INTR_HOST_EP_SPEC>`"]
pub type INTR_HOST_EP = crate::Reg<intr_host_ep::INTR_HOST_EP_SPEC>;
#[doc = "Interrupt USB Host Endpoint Register"]
pub mod intr_host_ep;
#[doc = "INTR_HOST_EP_SET register accessor: an alias for `Reg<INTR_HOST_EP_SET_SPEC>`"]
pub type INTR_HOST_EP_SET = crate::Reg<intr_host_ep_set::INTR_HOST_EP_SET_SPEC>;
#[doc = "Interrupt USB Host Endpoint Set Register"]
pub mod intr_host_ep_set;
#[doc = "INTR_HOST_EP_MASK register accessor: an alias for `Reg<INTR_HOST_EP_MASK_SPEC>`"]
pub type INTR_HOST_EP_MASK = crate::Reg<intr_host_ep_mask::INTR_HOST_EP_MASK_SPEC>;
#[doc = "Interrupt USB Host Endpoint Mask Register"]
pub mod intr_host_ep_mask;
#[doc = "INTR_HOST_EP_MASKED register accessor: an alias for `Reg<INTR_HOST_EP_MASKED_SPEC>`"]
pub type INTR_HOST_EP_MASKED = crate::Reg<intr_host_ep_masked::INTR_HOST_EP_MASKED_SPEC>;
#[doc = "Interrupt USB Host Endpoint Masked Register"]
pub mod intr_host_ep_masked;
#[doc = "HOST_DMA_ENBL register accessor: an alias for `Reg<HOST_DMA_ENBL_SPEC>`"]
pub type HOST_DMA_ENBL = crate::Reg<host_dma_enbl::HOST_DMA_ENBL_SPEC>;
#[doc = "Host DMA Enable Register"]
pub mod host_dma_enbl;
#[doc = "HOST_EP1_BLK register accessor: an alias for `Reg<HOST_EP1_BLK_SPEC>`"]
pub type HOST_EP1_BLK = crate::Reg<host_ep1_blk::HOST_EP1_BLK_SPEC>;
#[doc = "Host Endpoint 1 Block Register"]
pub mod host_ep1_blk;
#[doc = "HOST_EP2_BLK register accessor: an alias for `Reg<HOST_EP2_BLK_SPEC>`"]
pub type HOST_EP2_BLK = crate::Reg<host_ep2_blk::HOST_EP2_BLK_SPEC>;
#[doc = "Host Endpoint 2 Block Register"]
pub mod host_ep2_blk;
