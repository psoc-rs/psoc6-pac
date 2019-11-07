#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Fault structure"]
    pub struct0: STRUCT,
    _reserved1: [u8; 48usize],
    #[doc = "0x100 - Fault structure"]
    pub struct1: STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - Fault control"]
    pub ctl: self::struct_::CTL,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - Fault status"]
    pub status: self::struct_::STATUS,
    #[doc = "0x10 - Fault data"]
    pub data: [self::struct_::DATA; 4],
    _reserved3: [u8; 32usize],
    #[doc = "0x40 - Fault pending 0"]
    pub pending0: self::struct_::PENDING0,
    #[doc = "0x44 - Fault pending 1"]
    pub pending1: self::struct_::PENDING1,
    #[doc = "0x48 - Fault pending 2"]
    pub pending2: self::struct_::PENDING2,
    _reserved6: [u8; 4usize],
    #[doc = "0x50 - Fault mask 0"]
    pub mask0: self::struct_::MASK0,
    #[doc = "0x54 - Fault mask 1"]
    pub mask1: self::struct_::MASK1,
    #[doc = "0x58 - Fault mask 2"]
    pub mask2: self::struct_::MASK2,
    _reserved9: [u8; 100usize],
    #[doc = "0xc0 - Interrupt"]
    pub intr: self::struct_::INTR,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_set: self::struct_::INTR_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_mask: self::struct_::INTR_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_masked: self::struct_::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "Fault structure"]
pub mod struct_;
