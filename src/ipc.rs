#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPC structure"]
    pub struct0: STRUCT,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - IPC structure"]
    pub struct1: STRUCT,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - IPC structure"]
    pub struct2: STRUCT,
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - IPC structure"]
    pub struct3: STRUCT,
    _reserved4: [u8; 12usize],
    #[doc = "0x80 - IPC structure"]
    pub struct4: STRUCT,
    _reserved5: [u8; 12usize],
    #[doc = "0xa0 - IPC structure"]
    pub struct5: STRUCT,
    _reserved6: [u8; 12usize],
    #[doc = "0xc0 - IPC structure"]
    pub struct6: STRUCT,
    _reserved7: [u8; 12usize],
    #[doc = "0xe0 - IPC structure"]
    pub struct7: STRUCT,
    _reserved8: [u8; 12usize],
    #[doc = "0x100 - IPC structure"]
    pub struct8: STRUCT,
    _reserved9: [u8; 12usize],
    #[doc = "0x120 - IPC structure"]
    pub struct9: STRUCT,
    _reserved10: [u8; 12usize],
    #[doc = "0x140 - IPC structure"]
    pub struct10: STRUCT,
    _reserved11: [u8; 12usize],
    #[doc = "0x160 - IPC structure"]
    pub struct11: STRUCT,
    _reserved12: [u8; 12usize],
    #[doc = "0x180 - IPC structure"]
    pub struct12: STRUCT,
    _reserved13: [u8; 12usize],
    #[doc = "0x1a0 - IPC structure"]
    pub struct13: STRUCT,
    _reserved14: [u8; 12usize],
    #[doc = "0x1c0 - IPC structure"]
    pub struct14: STRUCT,
    _reserved15: [u8; 12usize],
    #[doc = "0x1e0 - IPC structure"]
    pub struct15: STRUCT,
    _reserved16: [u8; 3596usize],
    #[doc = "0x1000 - IPC interrupt structure"]
    pub intr_struct0: INTR_STRUCT,
    _reserved17: [u8; 16usize],
    #[doc = "0x1020 - IPC interrupt structure"]
    pub intr_struct1: INTR_STRUCT,
    _reserved18: [u8; 16usize],
    #[doc = "0x1040 - IPC interrupt structure"]
    pub intr_struct2: INTR_STRUCT,
    _reserved19: [u8; 16usize],
    #[doc = "0x1060 - IPC interrupt structure"]
    pub intr_struct3: INTR_STRUCT,
    _reserved20: [u8; 16usize],
    #[doc = "0x1080 - IPC interrupt structure"]
    pub intr_struct4: INTR_STRUCT,
    _reserved21: [u8; 16usize],
    #[doc = "0x10a0 - IPC interrupt structure"]
    pub intr_struct5: INTR_STRUCT,
    _reserved22: [u8; 16usize],
    #[doc = "0x10c0 - IPC interrupt structure"]
    pub intr_struct6: INTR_STRUCT,
    _reserved23: [u8; 16usize],
    #[doc = "0x10e0 - IPC interrupt structure"]
    pub intr_struct7: INTR_STRUCT,
    _reserved24: [u8; 16usize],
    #[doc = "0x1100 - IPC interrupt structure"]
    pub intr_struct8: INTR_STRUCT,
    _reserved25: [u8; 16usize],
    #[doc = "0x1120 - IPC interrupt structure"]
    pub intr_struct9: INTR_STRUCT,
    _reserved26: [u8; 16usize],
    #[doc = "0x1140 - IPC interrupt structure"]
    pub intr_struct10: INTR_STRUCT,
    _reserved27: [u8; 16usize],
    #[doc = "0x1160 - IPC interrupt structure"]
    pub intr_struct11: INTR_STRUCT,
    _reserved28: [u8; 16usize],
    #[doc = "0x1180 - IPC interrupt structure"]
    pub intr_struct12: INTR_STRUCT,
    _reserved29: [u8; 16usize],
    #[doc = "0x11a0 - IPC interrupt structure"]
    pub intr_struct13: INTR_STRUCT,
    _reserved30: [u8; 16usize],
    #[doc = "0x11c0 - IPC interrupt structure"]
    pub intr_struct14: INTR_STRUCT,
    _reserved31: [u8; 16usize],
    #[doc = "0x11e0 - IPC interrupt structure"]
    pub intr_struct15: INTR_STRUCT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - IPC acquire"]
    pub acquire: self::struct_::ACQUIRE,
    #[doc = "0x04 - IPC release"]
    pub release: self::struct_::RELEASE,
    #[doc = "0x08 - IPC notification"]
    pub notify: self::struct_::NOTIFY,
    #[doc = "0x0c - IPC data"]
    pub data: self::struct_::DATA,
    #[doc = "0x10 - IPC lock status"]
    pub lock_status: self::struct_::LOCK_STATUS,
}
#[doc = r"Register block"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = r"Register block"]
#[repr(C)]
pub struct INTR_STRUCT {
    #[doc = "0x00 - Interrupt"]
    pub intr: self::intr_struct::INTR,
    #[doc = "0x04 - Interrupt set"]
    pub intr_set: self::intr_struct::INTR_SET,
    #[doc = "0x08 - Interrupt mask"]
    pub intr_mask: self::intr_struct::INTR_MASK,
    #[doc = "0x0c - Interrupt masked"]
    pub intr_masked: self::intr_struct::INTR_MASKED,
}
#[doc = r"Register block"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;
