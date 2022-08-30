#[doc = "MS0_CTL register accessor: an alias for `Reg<MS0_CTL_SPEC>`"]
pub type MS0_CTL = crate::Reg<ms0_ctl::MS0_CTL_SPEC>;
#[doc = "Master 0 protection context control"]
pub mod ms0_ctl;
#[doc = "MS1_CTL register accessor: an alias for `Reg<MS1_CTL_SPEC>`"]
pub type MS1_CTL = crate::Reg<ms1_ctl::MS1_CTL_SPEC>;
#[doc = "Master 1 protection context control"]
pub mod ms1_ctl;
#[doc = "MS2_CTL register accessor: an alias for `Reg<MS2_CTL_SPEC>`"]
pub type MS2_CTL = crate::Reg<ms2_ctl::MS2_CTL_SPEC>;
#[doc = "Master 2 protection context control"]
pub mod ms2_ctl;
#[doc = "MS3_CTL register accessor: an alias for `Reg<MS3_CTL_SPEC>`"]
pub type MS3_CTL = crate::Reg<ms3_ctl::MS3_CTL_SPEC>;
#[doc = "Master 3 protection context control"]
pub mod ms3_ctl;
#[doc = "MS4_CTL register accessor: an alias for `Reg<MS4_CTL_SPEC>`"]
pub type MS4_CTL = crate::Reg<ms4_ctl::MS4_CTL_SPEC>;
#[doc = "Master 4 protection context control"]
pub mod ms4_ctl;
#[doc = "MS5_CTL register accessor: an alias for `Reg<MS5_CTL_SPEC>`"]
pub type MS5_CTL = crate::Reg<ms5_ctl::MS5_CTL_SPEC>;
#[doc = "Master 5 protection context control"]
pub mod ms5_ctl;
#[doc = "MS6_CTL register accessor: an alias for `Reg<MS6_CTL_SPEC>`"]
pub type MS6_CTL = crate::Reg<ms6_ctl::MS6_CTL_SPEC>;
#[doc = "Master 6 protection context control"]
pub mod ms6_ctl;
#[doc = "MS7_CTL register accessor: an alias for `Reg<MS7_CTL_SPEC>`"]
pub type MS7_CTL = crate::Reg<ms7_ctl::MS7_CTL_SPEC>;
#[doc = "Master 7 protection context control"]
pub mod ms7_ctl;
#[doc = "MS8_CTL register accessor: an alias for `Reg<MS8_CTL_SPEC>`"]
pub type MS8_CTL = crate::Reg<ms8_ctl::MS8_CTL_SPEC>;
#[doc = "Master 8 protection context control"]
pub mod ms8_ctl;
#[doc = "MS9_CTL register accessor: an alias for `Reg<MS9_CTL_SPEC>`"]
pub type MS9_CTL = crate::Reg<ms9_ctl::MS9_CTL_SPEC>;
#[doc = "Master 9 protection context control"]
pub mod ms9_ctl;
#[doc = "MS10_CTL register accessor: an alias for `Reg<MS10_CTL_SPEC>`"]
pub type MS10_CTL = crate::Reg<ms10_ctl::MS10_CTL_SPEC>;
#[doc = "Master 10 protection context control"]
pub mod ms10_ctl;
#[doc = "MS11_CTL register accessor: an alias for `Reg<MS11_CTL_SPEC>`"]
pub type MS11_CTL = crate::Reg<ms11_ctl::MS11_CTL_SPEC>;
#[doc = "Master 11 protection context control"]
pub mod ms11_ctl;
#[doc = "MS12_CTL register accessor: an alias for `Reg<MS12_CTL_SPEC>`"]
pub type MS12_CTL = crate::Reg<ms12_ctl::MS12_CTL_SPEC>;
#[doc = "Master 12 protection context control"]
pub mod ms12_ctl;
#[doc = "MS13_CTL register accessor: an alias for `Reg<MS13_CTL_SPEC>`"]
pub type MS13_CTL = crate::Reg<ms13_ctl::MS13_CTL_SPEC>;
#[doc = "Master 13 protection context control"]
pub mod ms13_ctl;
#[doc = "MS14_CTL register accessor: an alias for `Reg<MS14_CTL_SPEC>`"]
pub type MS14_CTL = crate::Reg<ms14_ctl::MS14_CTL_SPEC>;
#[doc = "Master 14 protection context control"]
pub mod ms14_ctl;
#[doc = "MS15_CTL register accessor: an alias for `Reg<MS15_CTL_SPEC>`"]
pub type MS15_CTL = crate::Reg<ms15_ctl::MS15_CTL_SPEC>;
#[doc = "Master 15 protection context control"]
pub mod ms15_ctl;
#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU_STRUCT {
    #[doc = "0x00 - SMPU region address 0 (slave structure)"]
    pub addr0: crate::Reg<self::smpu_struct::addr0::ADDR0_SPEC>,
    #[doc = "0x04 - SMPU region attributes 0 (slave structure)"]
    pub att0: crate::Reg<self::smpu_struct::att0::ATT0_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - SMPU region address 1 (master structure)"]
    pub addr1: crate::Reg<self::smpu_struct::addr1::ADDR1_SPEC>,
    #[doc = "0x24 - SMPU region attributes 1 (master structure)"]
    pub att1: crate::Reg<self::smpu_struct::att1::ATT1_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SMPU structure"]
pub mod smpu_struct;
