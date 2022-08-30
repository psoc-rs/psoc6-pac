#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID & Revision"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x04 - LCD Divider Register"]
    pub divider: crate::Reg<divider::DIVIDER_SPEC>,
    #[doc = "0x08 - LCD Configuration Register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100..0x120 - LCD Pin Data Registers"]
    pub data0: [crate::Reg<data0::DATA0_SPEC>; 8],
    _reserved4: [u8; 0xe0],
    #[doc = "0x200..0x220 - LCD Pin Data Registers"]
    pub data1: [crate::Reg<data1::DATA1_SPEC>; 8],
    _reserved5: [u8; 0xe0],
    #[doc = "0x300..0x320 - LCD Pin Data Registers"]
    pub data2: [crate::Reg<data2::DATA2_SPEC>; 8],
    _reserved6: [u8; 0xe0],
    #[doc = "0x400..0x420 - LCD Pin Data Registers"]
    pub data3: [crate::Reg<data3::DATA3_SPEC>; 8],
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID & Revision"]
pub mod id;
#[doc = "DIVIDER register accessor: an alias for `Reg<DIVIDER_SPEC>`"]
pub type DIVIDER = crate::Reg<divider::DIVIDER_SPEC>;
#[doc = "LCD Divider Register"]
pub mod divider;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "LCD Configuration Register"]
pub mod control;
#[doc = "DATA0 register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data0;
#[doc = "DATA1 register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data1;
#[doc = "DATA2 register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data2;
#[doc = "DATA3 register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "LCD Pin Data Registers"]
pub mod data3;
