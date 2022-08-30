#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x14 - IPC structure"]
    pub struct0: STRUCT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20..0x34 - IPC structure"]
    pub struct1: STRUCT,
    _reserved2: [u8; 0x0c],
    #[doc = "0x40..0x54 - IPC structure"]
    pub struct2: STRUCT,
    _reserved3: [u8; 0x0c],
    #[doc = "0x60..0x74 - IPC structure"]
    pub struct3: STRUCT,
    _reserved4: [u8; 0x0c],
    #[doc = "0x80..0x94 - IPC structure"]
    pub struct4: STRUCT,
    _reserved5: [u8; 0x0c],
    #[doc = "0xa0..0xb4 - IPC structure"]
    pub struct5: STRUCT,
    _reserved6: [u8; 0x0c],
    #[doc = "0xc0..0xd4 - IPC structure"]
    pub struct6: STRUCT,
    _reserved7: [u8; 0x0c],
    #[doc = "0xe0..0xf4 - IPC structure"]
    pub struct7: STRUCT,
    _reserved8: [u8; 0x0c],
    #[doc = "0x100..0x114 - IPC structure"]
    pub struct8: STRUCT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x120..0x134 - IPC structure"]
    pub struct9: STRUCT,
    _reserved10: [u8; 0x0c],
    #[doc = "0x140..0x154 - IPC structure"]
    pub struct10: STRUCT,
    _reserved11: [u8; 0x0c],
    #[doc = "0x160..0x174 - IPC structure"]
    pub struct11: STRUCT,
    _reserved12: [u8; 0x0c],
    #[doc = "0x180..0x194 - IPC structure"]
    pub struct12: STRUCT,
    _reserved13: [u8; 0x0c],
    #[doc = "0x1a0..0x1b4 - IPC structure"]
    pub struct13: STRUCT,
    _reserved14: [u8; 0x0c],
    #[doc = "0x1c0..0x1d4 - IPC structure"]
    pub struct14: STRUCT,
    _reserved15: [u8; 0x0c],
    #[doc = "0x1e0..0x1f4 - IPC structure"]
    pub struct15: STRUCT,
    _reserved16: [u8; 0x0e0c],
    #[doc = "0x1000..0x1010 - IPC interrupt structure"]
    pub intr_struct0: INTR_STRUCT,
    _reserved17: [u8; 0x10],
    #[doc = "0x1020..0x1030 - IPC interrupt structure"]
    pub intr_struct1: INTR_STRUCT,
    _reserved18: [u8; 0x10],
    #[doc = "0x1040..0x1050 - IPC interrupt structure"]
    pub intr_struct2: INTR_STRUCT,
    _reserved19: [u8; 0x10],
    #[doc = "0x1060..0x1070 - IPC interrupt structure"]
    pub intr_struct3: INTR_STRUCT,
    _reserved20: [u8; 0x10],
    #[doc = "0x1080..0x1090 - IPC interrupt structure"]
    pub intr_struct4: INTR_STRUCT,
    _reserved21: [u8; 0x10],
    #[doc = "0x10a0..0x10b0 - IPC interrupt structure"]
    pub intr_struct5: INTR_STRUCT,
    _reserved22: [u8; 0x10],
    #[doc = "0x10c0..0x10d0 - IPC interrupt structure"]
    pub intr_struct6: INTR_STRUCT,
    _reserved23: [u8; 0x10],
    #[doc = "0x10e0..0x10f0 - IPC interrupt structure"]
    pub intr_struct7: INTR_STRUCT,
    _reserved24: [u8; 0x10],
    #[doc = "0x1100..0x1110 - IPC interrupt structure"]
    pub intr_struct8: INTR_STRUCT,
    _reserved25: [u8; 0x10],
    #[doc = "0x1120..0x1130 - IPC interrupt structure"]
    pub intr_struct9: INTR_STRUCT,
    _reserved26: [u8; 0x10],
    #[doc = "0x1140..0x1150 - IPC interrupt structure"]
    pub intr_struct10: INTR_STRUCT,
    _reserved27: [u8; 0x10],
    #[doc = "0x1160..0x1170 - IPC interrupt structure"]
    pub intr_struct11: INTR_STRUCT,
    _reserved28: [u8; 0x10],
    #[doc = "0x1180..0x1190 - IPC interrupt structure"]
    pub intr_struct12: INTR_STRUCT,
    _reserved29: [u8; 0x10],
    #[doc = "0x11a0..0x11b0 - IPC interrupt structure"]
    pub intr_struct13: INTR_STRUCT,
    _reserved30: [u8; 0x10],
    #[doc = "0x11c0..0x11d0 - IPC interrupt structure"]
    pub intr_struct14: INTR_STRUCT,
    _reserved31: [u8; 0x10],
    #[doc = "0x11e0..0x11f0 - IPC interrupt structure"]
    pub intr_struct15: INTR_STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - IPC acquire"]
    pub acquire: crate::Reg<self::struct_::acquire::ACQUIRE_SPEC>,
    #[doc = "0x04 - IPC release"]
    pub release: crate::Reg<self::struct_::release::RELEASE_SPEC>,
    #[doc = "0x08 - IPC notification"]
    pub notify: crate::Reg<self::struct_::notify::NOTIFY_SPEC>,
    #[doc = "0x0c - IPC data"]
    pub data: crate::Reg<self::struct_::data::DATA_SPEC>,
    #[doc = "0x10 - IPC lock status"]
    pub lock_status: crate::Reg<self::struct_::lock_status::LOCK_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTR_STRUCT {
    #[doc = "0x00 - Interrupt"]
    pub intr: crate::Reg<self::intr_struct::intr::INTR_SPEC>,
    #[doc = "0x04 - Interrupt set"]
    pub intr_set: crate::Reg<self::intr_struct::intr_set::INTR_SET_SPEC>,
    #[doc = "0x08 - Interrupt mask"]
    pub intr_mask: crate::Reg<self::intr_struct::intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x0c - Interrupt masked"]
    pub intr_masked: crate::Reg<self::intr_struct::intr_masked::INTR_MASKED_SPEC>,
}
#[doc = r"Register block"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;
