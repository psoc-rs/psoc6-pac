#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xd0 - Fault structure"]
    pub struct0: STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x100..0x1d0 - Fault structure"]
    pub struct1: STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - Fault control"]
    pub ctl: crate::Reg<self::struct_::ctl::CTL_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - Fault status"]
    pub status: crate::Reg<self::struct_::status::STATUS_SPEC>,
    #[doc = "0x10..0x20 - Fault data"]
    pub data: [crate::Reg<self::struct_::data::DATA_SPEC>; 4],
    _reserved3: [u8; 0x20],
    #[doc = "0x40 - Fault pending 0"]
    pub pending0: crate::Reg<self::struct_::pending0::PENDING0_SPEC>,
    #[doc = "0x44 - Fault pending 1"]
    pub pending1: crate::Reg<self::struct_::pending1::PENDING1_SPEC>,
    #[doc = "0x48 - Fault pending 2"]
    pub pending2: crate::Reg<self::struct_::pending2::PENDING2_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x50 - Fault mask 0"]
    pub mask0: crate::Reg<self::struct_::mask0::MASK0_SPEC>,
    #[doc = "0x54 - Fault mask 1"]
    pub mask1: crate::Reg<self::struct_::mask1::MASK1_SPEC>,
    #[doc = "0x58 - Fault mask 2"]
    pub mask2: crate::Reg<self::struct_::mask2::MASK2_SPEC>,
    _reserved9: [u8; 0x64],
    #[doc = "0xc0 - Interrupt"]
    pub intr: crate::Reg<self::struct_::intr::INTR_SPEC>,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_set: crate::Reg<self::struct_::intr_set::INTR_SET_SPEC>,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_mask: crate::Reg<self::struct_::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_masked: crate::Reg<self::struct_::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Fault structure"]
pub mod struct_;
