use crate::prot::mpu::MPU_STRUCT;
use crate::prot::smpu::SMPU_STRUCT;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMPU"]
    pub smpu: SMPU,
    _reserved1: [u8; 8152usize],
    #[doc = "0x4000 - MPU"]
    pub mpu0: MPU,
    _reserved2: [u8; 504usize],
    #[doc = "0x4400 - MPU"]
    pub mpu1: MPU,
    _reserved3: [u8; 504usize],
    #[doc = "0x4800 - MPU"]
    pub mpu2: MPU,
    _reserved4: [u8; 504usize],
    #[doc = "0x4c00 - MPU"]
    pub mpu3: MPU,
    _reserved5: [u8; 504usize],
    #[doc = "0x5000 - MPU"]
    pub mpu4: MPU,
    _reserved6: [u8; 504usize],
    #[doc = "0x5400 - MPU"]
    pub mpu5: MPU,
    _reserved7: [u8; 504usize],
    #[doc = "0x5800 - MPU"]
    pub mpu6: MPU,
    _reserved8: [u8; 504usize],
    #[doc = "0x5c00 - MPU"]
    pub mpu7: MPU,
    _reserved9: [u8; 504usize],
    #[doc = "0x6000 - MPU"]
    pub mpu8: MPU,
    _reserved10: [u8; 504usize],
    #[doc = "0x6400 - MPU"]
    pub mpu9: MPU,
    _reserved11: [u8; 504usize],
    #[doc = "0x6800 - MPU"]
    pub mpu10: MPU,
    _reserved12: [u8; 504usize],
    #[doc = "0x6c00 - MPU"]
    pub mpu11: MPU,
    _reserved13: [u8; 504usize],
    #[doc = "0x7000 - MPU"]
    pub mpu12: MPU,
    _reserved14: [u8; 504usize],
    #[doc = "0x7400 - MPU"]
    pub mpu13: MPU,
    _reserved15: [u8; 504usize],
    #[doc = "0x7800 - MPU"]
    pub mpu14: MPU,
    _reserved16: [u8; 504usize],
    #[doc = "0x7c00 - MPU"]
    pub mpu15: MPU,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU {
    #[doc = "0x00 - Master 0 protection context control"]
    pub ms0_ctl: self::smpu::MS0_CTL,
    #[doc = "0x04 - Master 1 protection context control"]
    pub ms1_ctl: self::smpu::MS1_CTL,
    #[doc = "0x08 - Master 2 protection context control"]
    pub ms2_ctl: self::smpu::MS2_CTL,
    #[doc = "0x0c - Master 3 protection context control"]
    pub ms3_ctl: self::smpu::MS3_CTL,
    #[doc = "0x10 - Master 4 protection context control"]
    pub ms4_ctl: self::smpu::MS4_CTL,
    #[doc = "0x14 - Master 5 protection context control"]
    pub ms5_ctl: self::smpu::MS5_CTL,
    #[doc = "0x18 - Master 6 protection context control"]
    pub ms6_ctl: self::smpu::MS6_CTL,
    #[doc = "0x1c - Master 7 protection context control"]
    pub ms7_ctl: self::smpu::MS7_CTL,
    #[doc = "0x20 - Master 8 protection context control"]
    pub ms8_ctl: self::smpu::MS8_CTL,
    #[doc = "0x24 - Master 9 protection context control"]
    pub ms9_ctl: self::smpu::MS9_CTL,
    #[doc = "0x28 - Master 10 protection context control"]
    pub ms10_ctl: self::smpu::MS10_CTL,
    #[doc = "0x2c - Master 11 protection context control"]
    pub ms11_ctl: self::smpu::MS11_CTL,
    #[doc = "0x30 - Master 12 protection context control"]
    pub ms12_ctl: self::smpu::MS12_CTL,
    #[doc = "0x34 - Master 13 protection context control"]
    pub ms13_ctl: self::smpu::MS13_CTL,
    #[doc = "0x38 - Master 14 protection context control"]
    pub ms14_ctl: self::smpu::MS14_CTL,
    #[doc = "0x3c - Master 15 protection context control"]
    pub ms15_ctl: self::smpu::MS15_CTL,
    _reserved16: [u8; 8128usize],
    #[doc = "0x2000 - SMPU structure"]
    pub smpu_struct0: SMPU_STRUCT,
    _reserved17: [u8; 24usize],
    #[doc = "0x2040 - SMPU structure"]
    pub smpu_struct1: SMPU_STRUCT,
    _reserved18: [u8; 24usize],
    #[doc = "0x2080 - SMPU structure"]
    pub smpu_struct2: SMPU_STRUCT,
    _reserved19: [u8; 24usize],
    #[doc = "0x20c0 - SMPU structure"]
    pub smpu_struct3: SMPU_STRUCT,
    _reserved20: [u8; 24usize],
    #[doc = "0x2100 - SMPU structure"]
    pub smpu_struct4: SMPU_STRUCT,
    _reserved21: [u8; 24usize],
    #[doc = "0x2140 - SMPU structure"]
    pub smpu_struct5: SMPU_STRUCT,
    _reserved22: [u8; 24usize],
    #[doc = "0x2180 - SMPU structure"]
    pub smpu_struct6: SMPU_STRUCT,
    _reserved23: [u8; 24usize],
    #[doc = "0x21c0 - SMPU structure"]
    pub smpu_struct7: SMPU_STRUCT,
    _reserved24: [u8; 24usize],
    #[doc = "0x2200 - SMPU structure"]
    pub smpu_struct8: SMPU_STRUCT,
    _reserved25: [u8; 24usize],
    #[doc = "0x2240 - SMPU structure"]
    pub smpu_struct9: SMPU_STRUCT,
    _reserved26: [u8; 24usize],
    #[doc = "0x2280 - SMPU structure"]
    pub smpu_struct10: SMPU_STRUCT,
    _reserved27: [u8; 24usize],
    #[doc = "0x22c0 - SMPU structure"]
    pub smpu_struct11: SMPU_STRUCT,
    _reserved28: [u8; 24usize],
    #[doc = "0x2300 - SMPU structure"]
    pub smpu_struct12: SMPU_STRUCT,
    _reserved29: [u8; 24usize],
    #[doc = "0x2340 - SMPU structure"]
    pub smpu_struct13: SMPU_STRUCT,
    _reserved30: [u8; 24usize],
    #[doc = "0x2380 - SMPU structure"]
    pub smpu_struct14: SMPU_STRUCT,
    _reserved31: [u8; 24usize],
    #[doc = "0x23c0 - SMPU structure"]
    pub smpu_struct15: SMPU_STRUCT,
}
#[doc = r"Register block"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    #[doc = "0x00 - Master control"]
    pub ms_ctl: self::mpu::MS_CTL,
    _reserved1: [u8; 508usize],
    #[doc = "0x200 - MPU structure"]
    pub mpu_struct0: MPU_STRUCT,
    _reserved2: [u8; 24usize],
    #[doc = "0x220 - MPU structure"]
    pub mpu_struct1: MPU_STRUCT,
    _reserved3: [u8; 24usize],
    #[doc = "0x240 - MPU structure"]
    pub mpu_struct2: MPU_STRUCT,
    _reserved4: [u8; 24usize],
    #[doc = "0x260 - MPU structure"]
    pub mpu_struct3: MPU_STRUCT,
    _reserved5: [u8; 24usize],
    #[doc = "0x280 - MPU structure"]
    pub mpu_struct4: MPU_STRUCT,
    _reserved6: [u8; 24usize],
    #[doc = "0x2a0 - MPU structure"]
    pub mpu_struct5: MPU_STRUCT,
    _reserved7: [u8; 24usize],
    #[doc = "0x2c0 - MPU structure"]
    pub mpu_struct6: MPU_STRUCT,
    _reserved8: [u8; 24usize],
    #[doc = "0x2e0 - MPU structure"]
    pub mpu_struct7: MPU_STRUCT,
}
#[doc = r"Register block"]
#[doc = "MPU"]
pub mod mpu;
