#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved1: [u8; 0x0dfc],
    #[doc = "0xe00 - AREF configuration"]
    pub aref: AREF,
    _reserved2: [u8; 0xfc],
    #[doc = "0xf00 - VREF Trim bits"]
    pub vref_trim0: crate::Reg<vref_trim0::VREF_TRIM0_SPEC>,
    #[doc = "0xf04 - VREF Trim bits"]
    pub vref_trim1: crate::Reg<vref_trim1::VREF_TRIM1_SPEC>,
    #[doc = "0xf08 - VREF Trim bits"]
    pub vref_trim2: crate::Reg<vref_trim2::VREF_TRIM2_SPEC>,
    #[doc = "0xf0c - VREF Trim bits"]
    pub vref_trim3: crate::Reg<vref_trim3::VREF_TRIM3_SPEC>,
    #[doc = "0xf10 - IZTAT Trim bits"]
    pub iztat_trim0: crate::Reg<iztat_trim0::IZTAT_TRIM0_SPEC>,
    #[doc = "0xf14 - IZTAT Trim bits"]
    pub iztat_trim1: crate::Reg<iztat_trim1::IZTAT_TRIM1_SPEC>,
    #[doc = "0xf18 - IPTAT Trim bits"]
    pub iptat_trim0: crate::Reg<iptat_trim0::IPTAT_TRIM0_SPEC>,
    #[doc = "0xf1c - ICTAT Trim bits"]
    pub ictat_trim0: crate::Reg<ictat_trim0::ICTAT_TRIM0_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct AREF {
    #[doc = "0x00 - global AREF control"]
    pub aref_ctrl: crate::Reg<self::aref::aref_ctrl::AREF_CTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "AREF configuration"]
pub mod aref;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "VREF_TRIM0 register accessor: an alias for `Reg<VREF_TRIM0_SPEC>`"]
pub type VREF_TRIM0 = crate::Reg<vref_trim0::VREF_TRIM0_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim0;
#[doc = "VREF_TRIM1 register accessor: an alias for `Reg<VREF_TRIM1_SPEC>`"]
pub type VREF_TRIM1 = crate::Reg<vref_trim1::VREF_TRIM1_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim1;
#[doc = "VREF_TRIM2 register accessor: an alias for `Reg<VREF_TRIM2_SPEC>`"]
pub type VREF_TRIM2 = crate::Reg<vref_trim2::VREF_TRIM2_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim2;
#[doc = "VREF_TRIM3 register accessor: an alias for `Reg<VREF_TRIM3_SPEC>`"]
pub type VREF_TRIM3 = crate::Reg<vref_trim3::VREF_TRIM3_SPEC>;
#[doc = "VREF Trim bits"]
pub mod vref_trim3;
#[doc = "IZTAT_TRIM0 register accessor: an alias for `Reg<IZTAT_TRIM0_SPEC>`"]
pub type IZTAT_TRIM0 = crate::Reg<iztat_trim0::IZTAT_TRIM0_SPEC>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim0;
#[doc = "IZTAT_TRIM1 register accessor: an alias for `Reg<IZTAT_TRIM1_SPEC>`"]
pub type IZTAT_TRIM1 = crate::Reg<iztat_trim1::IZTAT_TRIM1_SPEC>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim1;
#[doc = "IPTAT_TRIM0 register accessor: an alias for `Reg<IPTAT_TRIM0_SPEC>`"]
pub type IPTAT_TRIM0 = crate::Reg<iptat_trim0::IPTAT_TRIM0_SPEC>;
#[doc = "IPTAT Trim bits"]
pub mod iptat_trim0;
#[doc = "ICTAT_TRIM0 register accessor: an alias for `Reg<ICTAT_TRIM0_SPEC>`"]
pub type ICTAT_TRIM0 = crate::Reg<ictat_trim0::ICTAT_TRIM0_SPEC>;
#[doc = "ICTAT Trim bits"]
pub mod ictat_trim0;
