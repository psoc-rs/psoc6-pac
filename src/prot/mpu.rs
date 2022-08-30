#[doc = "MS_CTL register accessor: an alias for `Reg<MS_CTL_SPEC>`"]
pub type MS_CTL = crate::Reg<ms_ctl::MS_CTL_SPEC>;
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "MS_CTL_READ_MIR register accessor: an alias for `Reg<MS_CTL_READ_MIR_SPEC>`"]
pub type MS_CTL_READ_MIR = crate::Reg<ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>;
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = r"Register block"]
#[repr(C)]
pub struct MPU_STRUCT {
    #[doc = "0x00 - MPU region address"]
    pub addr: crate::Reg<self::mpu_struct::addr::ADDR_SPEC>,
    #[doc = "0x04 - MPU region attrributes"]
    pub att: crate::Reg<self::mpu_struct::att::ATT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "MPU structure"]
pub mod mpu_struct;
