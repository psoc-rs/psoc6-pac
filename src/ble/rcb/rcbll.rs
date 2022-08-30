#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RCB LL control register."]
pub mod ctrl;
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
#[doc = "RADIO_REG1_ADDR register accessor: an alias for `Reg<RADIO_REG1_ADDR_SPEC>`"]
pub type RADIO_REG1_ADDR = crate::Reg<radio_reg1_addr::RADIO_REG1_ADDR_SPEC>;
#[doc = "Address of Register#1 in Radio (MDON)"]
pub mod radio_reg1_addr;
#[doc = "RADIO_REG2_ADDR register accessor: an alias for `Reg<RADIO_REG2_ADDR_SPEC>`"]
pub type RADIO_REG2_ADDR = crate::Reg<radio_reg2_addr::RADIO_REG2_ADDR_SPEC>;
#[doc = "Address of Register#2 in Radio (RSSI)"]
pub mod radio_reg2_addr;
#[doc = "RADIO_REG3_ADDR register accessor: an alias for `Reg<RADIO_REG3_ADDR_SPEC>`"]
pub type RADIO_REG3_ADDR = crate::Reg<radio_reg3_addr::RADIO_REG3_ADDR_SPEC>;
#[doc = "Address of Register#3 in Radio (ACCL)"]
pub mod radio_reg3_addr;
#[doc = "RADIO_REG4_ADDR register accessor: an alias for `Reg<RADIO_REG4_ADDR_SPEC>`"]
pub type RADIO_REG4_ADDR = crate::Reg<radio_reg4_addr::RADIO_REG4_ADDR_SPEC>;
#[doc = "Address of Register#4 in Radio (ACCH)"]
pub mod radio_reg4_addr;
#[doc = "RADIO_REG5_ADDR register accessor: an alias for `Reg<RADIO_REG5_ADDR_SPEC>`"]
pub type RADIO_REG5_ADDR = crate::Reg<radio_reg5_addr::RADIO_REG5_ADDR_SPEC>;
#[doc = "Address of Register#5 in Radio (RSSI ENERGY)"]
pub mod radio_reg5_addr;
#[doc = "CPU_WRITE_REG register accessor: an alias for `Reg<CPU_WRITE_REG_SPEC>`"]
pub type CPU_WRITE_REG = crate::Reg<cpu_write_reg::CPU_WRITE_REG_SPEC>;
#[doc = "N/A"]
pub mod cpu_write_reg;
#[doc = "CPU_READ_REG register accessor: an alias for `Reg<CPU_READ_REG_SPEC>`"]
pub type CPU_READ_REG = crate::Reg<cpu_read_reg::CPU_READ_REG_SPEC>;
#[doc = "N/A"]
pub mod cpu_read_reg;
