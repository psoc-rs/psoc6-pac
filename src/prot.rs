#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x23e8 - SMPU"]
    pub smpu: SMPU,
    _reserved1: [u8; 0x1c18],
    #[doc = "0x4000..0x42e8 - MPU"]
    pub mpu0: MPU,
    _reserved2: [u8; 0x0118],
    #[doc = "0x4400..0x46e8 - MPU"]
    pub mpu1: MPU,
    _reserved3: [u8; 0x0118],
    #[doc = "0x4800..0x4ae8 - MPU"]
    pub mpu2: MPU,
    _reserved4: [u8; 0x0118],
    #[doc = "0x4c00..0x4ee8 - MPU"]
    pub mpu3: MPU,
    _reserved5: [u8; 0x0118],
    #[doc = "0x5000..0x52e8 - MPU"]
    pub mpu4: MPU,
    _reserved6: [u8; 0x0118],
    #[doc = "0x5400..0x56e8 - MPU"]
    pub mpu5: MPU,
    _reserved7: [u8; 0x0118],
    #[doc = "0x5800..0x5ae8 - MPU"]
    pub mpu6: MPU,
    _reserved8: [u8; 0x0118],
    #[doc = "0x5c00..0x5ee8 - MPU"]
    pub mpu7: MPU,
    _reserved9: [u8; 0x0118],
    #[doc = "0x6000..0x62e8 - MPU"]
    pub mpu8: MPU,
    _reserved10: [u8; 0x0118],
    #[doc = "0x6400..0x66e8 - MPU"]
    pub mpu9: MPU,
    _reserved11: [u8; 0x0118],
    #[doc = "0x6800..0x6ae8 - MPU"]
    pub mpu10: MPU,
    _reserved12: [u8; 0x0118],
    #[doc = "0x6c00..0x6ee8 - MPU"]
    pub mpu11: MPU,
    _reserved13: [u8; 0x0118],
    #[doc = "0x7000..0x72e8 - MPU"]
    pub mpu12: MPU,
    _reserved14: [u8; 0x0118],
    #[doc = "0x7400..0x76e8 - MPU"]
    pub mpu13: MPU,
    _reserved15: [u8; 0x0118],
    #[doc = "0x7800..0x7ae8 - MPU"]
    pub mpu14: MPU,
    _reserved16: [u8; 0x0118],
    #[doc = "0x7c00..0x7ee8 - MPU"]
    pub mpu15: MPU,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU {
    #[doc = "0x00 - Master 0 protection context control"]
    pub ms0_ctl: crate::Reg<self::smpu::ms0_ctl::MS0_CTL_SPEC>,
    #[doc = "0x04 - Master 1 protection context control"]
    pub ms1_ctl: crate::Reg<self::smpu::ms1_ctl::MS1_CTL_SPEC>,
    #[doc = "0x08 - Master 2 protection context control"]
    pub ms2_ctl: crate::Reg<self::smpu::ms2_ctl::MS2_CTL_SPEC>,
    #[doc = "0x0c - Master 3 protection context control"]
    pub ms3_ctl: crate::Reg<self::smpu::ms3_ctl::MS3_CTL_SPEC>,
    #[doc = "0x10 - Master 4 protection context control"]
    pub ms4_ctl: crate::Reg<self::smpu::ms4_ctl::MS4_CTL_SPEC>,
    #[doc = "0x14 - Master 5 protection context control"]
    pub ms5_ctl: crate::Reg<self::smpu::ms5_ctl::MS5_CTL_SPEC>,
    #[doc = "0x18 - Master 6 protection context control"]
    pub ms6_ctl: crate::Reg<self::smpu::ms6_ctl::MS6_CTL_SPEC>,
    #[doc = "0x1c - Master 7 protection context control"]
    pub ms7_ctl: crate::Reg<self::smpu::ms7_ctl::MS7_CTL_SPEC>,
    #[doc = "0x20 - Master 8 protection context control"]
    pub ms8_ctl: crate::Reg<self::smpu::ms8_ctl::MS8_CTL_SPEC>,
    #[doc = "0x24 - Master 9 protection context control"]
    pub ms9_ctl: crate::Reg<self::smpu::ms9_ctl::MS9_CTL_SPEC>,
    #[doc = "0x28 - Master 10 protection context control"]
    pub ms10_ctl: crate::Reg<self::smpu::ms10_ctl::MS10_CTL_SPEC>,
    #[doc = "0x2c - Master 11 protection context control"]
    pub ms11_ctl: crate::Reg<self::smpu::ms11_ctl::MS11_CTL_SPEC>,
    #[doc = "0x30 - Master 12 protection context control"]
    pub ms12_ctl: crate::Reg<self::smpu::ms12_ctl::MS12_CTL_SPEC>,
    #[doc = "0x34 - Master 13 protection context control"]
    pub ms13_ctl: crate::Reg<self::smpu::ms13_ctl::MS13_CTL_SPEC>,
    #[doc = "0x38 - Master 14 protection context control"]
    pub ms14_ctl: crate::Reg<self::smpu::ms14_ctl::MS14_CTL_SPEC>,
    #[doc = "0x3c - Master 15 protection context control"]
    pub ms15_ctl: crate::Reg<self::smpu::ms15_ctl::MS15_CTL_SPEC>,
    _reserved16: [u8; 0x1fc0],
    #[doc = "0x2000..0x2028 - SMPU structure"]
    pub smpu_struct0: self::smpu::SMPU_STRUCT,
    _reserved17: [u8; 0x18],
    #[doc = "0x2040..0x2068 - SMPU structure"]
    pub smpu_struct1: self::smpu::SMPU_STRUCT,
    _reserved18: [u8; 0x18],
    #[doc = "0x2080..0x20a8 - SMPU structure"]
    pub smpu_struct2: self::smpu::SMPU_STRUCT,
    _reserved19: [u8; 0x18],
    #[doc = "0x20c0..0x20e8 - SMPU structure"]
    pub smpu_struct3: self::smpu::SMPU_STRUCT,
    _reserved20: [u8; 0x18],
    #[doc = "0x2100..0x2128 - SMPU structure"]
    pub smpu_struct4: self::smpu::SMPU_STRUCT,
    _reserved21: [u8; 0x18],
    #[doc = "0x2140..0x2168 - SMPU structure"]
    pub smpu_struct5: self::smpu::SMPU_STRUCT,
    _reserved22: [u8; 0x18],
    #[doc = "0x2180..0x21a8 - SMPU structure"]
    pub smpu_struct6: self::smpu::SMPU_STRUCT,
    _reserved23: [u8; 0x18],
    #[doc = "0x21c0..0x21e8 - SMPU structure"]
    pub smpu_struct7: self::smpu::SMPU_STRUCT,
    _reserved24: [u8; 0x18],
    #[doc = "0x2200..0x2228 - SMPU structure"]
    pub smpu_struct8: self::smpu::SMPU_STRUCT,
    _reserved25: [u8; 0x18],
    #[doc = "0x2240..0x2268 - SMPU structure"]
    pub smpu_struct9: self::smpu::SMPU_STRUCT,
    _reserved26: [u8; 0x18],
    #[doc = "0x2280..0x22a8 - SMPU structure"]
    pub smpu_struct10: self::smpu::SMPU_STRUCT,
    _reserved27: [u8; 0x18],
    #[doc = "0x22c0..0x22e8 - SMPU structure"]
    pub smpu_struct11: self::smpu::SMPU_STRUCT,
    _reserved28: [u8; 0x18],
    #[doc = "0x2300..0x2328 - SMPU structure"]
    pub smpu_struct12: self::smpu::SMPU_STRUCT,
    _reserved29: [u8; 0x18],
    #[doc = "0x2340..0x2368 - SMPU structure"]
    pub smpu_struct13: self::smpu::SMPU_STRUCT,
    _reserved30: [u8; 0x18],
    #[doc = "0x2380..0x23a8 - SMPU structure"]
    pub smpu_struct14: self::smpu::SMPU_STRUCT,
    _reserved31: [u8; 0x18],
    #[doc = "0x23c0..0x23e8 - SMPU structure"]
    pub smpu_struct15: self::smpu::SMPU_STRUCT,
}
#[doc = r"Register block"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    #[doc = "0x00 - Master control"]
    pub ms_ctl: crate::Reg<self::mpu::ms_ctl::MS_CTL_SPEC>,
    #[doc = "0x04..0x200 - Master control read mirror"]
    pub ms_ctl_read_mir: [crate::Reg<self::mpu::ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>; 127],
    #[doc = "0x200..0x208 - MPU structure"]
    pub mpu_struct0: self::mpu::MPU_STRUCT,
    _reserved3: [u8; 0x18],
    #[doc = "0x220..0x228 - MPU structure"]
    pub mpu_struct1: self::mpu::MPU_STRUCT,
    _reserved4: [u8; 0x18],
    #[doc = "0x240..0x248 - MPU structure"]
    pub mpu_struct2: self::mpu::MPU_STRUCT,
    _reserved5: [u8; 0x18],
    #[doc = "0x260..0x268 - MPU structure"]
    pub mpu_struct3: self::mpu::MPU_STRUCT,
    _reserved6: [u8; 0x18],
    #[doc = "0x280..0x288 - MPU structure"]
    pub mpu_struct4: self::mpu::MPU_STRUCT,
    _reserved7: [u8; 0x18],
    #[doc = "0x2a0..0x2a8 - MPU structure"]
    pub mpu_struct5: self::mpu::MPU_STRUCT,
    _reserved8: [u8; 0x18],
    #[doc = "0x2c0..0x2c8 - MPU structure"]
    pub mpu_struct6: self::mpu::MPU_STRUCT,
    _reserved9: [u8; 0x18],
    #[doc = "0x2e0..0x2e8 - MPU structure"]
    pub mpu_struct7: self::mpu::MPU_STRUCT,
}
#[doc = r"Register block"]
#[doc = "MPU"]
pub mod mpu;
