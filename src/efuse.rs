#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Command"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Sequencer Default value"]
    pub seq_default: crate::Reg<seq_default::SEQ_DEFAULT_SPEC>,
    _reserved3: [u8; 0x1c],
    #[doc = "0x40 - Sequencer read control 0"]
    pub seq_read_ctl_0: crate::Reg<seq_read_ctl_0::SEQ_READ_CTL_0_SPEC>,
    #[doc = "0x44 - Sequencer read control 1"]
    pub seq_read_ctl_1: crate::Reg<seq_read_ctl_1::SEQ_READ_CTL_1_SPEC>,
    #[doc = "0x48 - Sequencer read control 2"]
    pub seq_read_ctl_2: crate::Reg<seq_read_ctl_2::SEQ_READ_CTL_2_SPEC>,
    #[doc = "0x4c - Sequencer read control 3"]
    pub seq_read_ctl_3: crate::Reg<seq_read_ctl_3::SEQ_READ_CTL_3_SPEC>,
    #[doc = "0x50 - Sequencer read control 4"]
    pub seq_read_ctl_4: crate::Reg<seq_read_ctl_4::SEQ_READ_CTL_4_SPEC>,
    #[doc = "0x54 - Sequencer read control 5"]
    pub seq_read_ctl_5: crate::Reg<seq_read_ctl_5::SEQ_READ_CTL_5_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x60 - Sequencer program control 0"]
    pub seq_program_ctl_0: crate::Reg<seq_program_ctl_0::SEQ_PROGRAM_CTL_0_SPEC>,
    #[doc = "0x64 - Sequencer program control 1"]
    pub seq_program_ctl_1: crate::Reg<seq_program_ctl_1::SEQ_PROGRAM_CTL_1_SPEC>,
    #[doc = "0x68 - Sequencer program control 2"]
    pub seq_program_ctl_2: crate::Reg<seq_program_ctl_2::SEQ_PROGRAM_CTL_2_SPEC>,
    #[doc = "0x6c - Sequencer program control 3"]
    pub seq_program_ctl_3: crate::Reg<seq_program_ctl_3::SEQ_PROGRAM_CTL_3_SPEC>,
    #[doc = "0x70 - Sequencer program control 4"]
    pub seq_program_ctl_4: crate::Reg<seq_program_ctl_4::SEQ_PROGRAM_CTL_4_SPEC>,
    #[doc = "0x74 - Sequencer program control 5"]
    pub seq_program_ctl_5: crate::Reg<seq_program_ctl_5::SEQ_PROGRAM_CTL_5_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "SEQ_DEFAULT register accessor: an alias for `Reg<SEQ_DEFAULT_SPEC>`"]
pub type SEQ_DEFAULT = crate::Reg<seq_default::SEQ_DEFAULT_SPEC>;
#[doc = "Sequencer Default value"]
pub mod seq_default;
#[doc = "SEQ_READ_CTL_0 register accessor: an alias for `Reg<SEQ_READ_CTL_0_SPEC>`"]
pub type SEQ_READ_CTL_0 = crate::Reg<seq_read_ctl_0::SEQ_READ_CTL_0_SPEC>;
#[doc = "Sequencer read control 0"]
pub mod seq_read_ctl_0;
#[doc = "SEQ_READ_CTL_1 register accessor: an alias for `Reg<SEQ_READ_CTL_1_SPEC>`"]
pub type SEQ_READ_CTL_1 = crate::Reg<seq_read_ctl_1::SEQ_READ_CTL_1_SPEC>;
#[doc = "Sequencer read control 1"]
pub mod seq_read_ctl_1;
#[doc = "SEQ_READ_CTL_2 register accessor: an alias for `Reg<SEQ_READ_CTL_2_SPEC>`"]
pub type SEQ_READ_CTL_2 = crate::Reg<seq_read_ctl_2::SEQ_READ_CTL_2_SPEC>;
#[doc = "Sequencer read control 2"]
pub mod seq_read_ctl_2;
#[doc = "SEQ_READ_CTL_3 register accessor: an alias for `Reg<SEQ_READ_CTL_3_SPEC>`"]
pub type SEQ_READ_CTL_3 = crate::Reg<seq_read_ctl_3::SEQ_READ_CTL_3_SPEC>;
#[doc = "Sequencer read control 3"]
pub mod seq_read_ctl_3;
#[doc = "SEQ_READ_CTL_4 register accessor: an alias for `Reg<SEQ_READ_CTL_4_SPEC>`"]
pub type SEQ_READ_CTL_4 = crate::Reg<seq_read_ctl_4::SEQ_READ_CTL_4_SPEC>;
#[doc = "Sequencer read control 4"]
pub mod seq_read_ctl_4;
#[doc = "SEQ_READ_CTL_5 register accessor: an alias for `Reg<SEQ_READ_CTL_5_SPEC>`"]
pub type SEQ_READ_CTL_5 = crate::Reg<seq_read_ctl_5::SEQ_READ_CTL_5_SPEC>;
#[doc = "Sequencer read control 5"]
pub mod seq_read_ctl_5;
#[doc = "SEQ_PROGRAM_CTL_0 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_0_SPEC>`"]
pub type SEQ_PROGRAM_CTL_0 = crate::Reg<seq_program_ctl_0::SEQ_PROGRAM_CTL_0_SPEC>;
#[doc = "Sequencer program control 0"]
pub mod seq_program_ctl_0;
#[doc = "SEQ_PROGRAM_CTL_1 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_1_SPEC>`"]
pub type SEQ_PROGRAM_CTL_1 = crate::Reg<seq_program_ctl_1::SEQ_PROGRAM_CTL_1_SPEC>;
#[doc = "Sequencer program control 1"]
pub mod seq_program_ctl_1;
#[doc = "SEQ_PROGRAM_CTL_2 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_2_SPEC>`"]
pub type SEQ_PROGRAM_CTL_2 = crate::Reg<seq_program_ctl_2::SEQ_PROGRAM_CTL_2_SPEC>;
#[doc = "Sequencer program control 2"]
pub mod seq_program_ctl_2;
#[doc = "SEQ_PROGRAM_CTL_3 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_3_SPEC>`"]
pub type SEQ_PROGRAM_CTL_3 = crate::Reg<seq_program_ctl_3::SEQ_PROGRAM_CTL_3_SPEC>;
#[doc = "Sequencer program control 3"]
pub mod seq_program_ctl_3;
#[doc = "SEQ_PROGRAM_CTL_4 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_4_SPEC>`"]
pub type SEQ_PROGRAM_CTL_4 = crate::Reg<seq_program_ctl_4::SEQ_PROGRAM_CTL_4_SPEC>;
#[doc = "Sequencer program control 4"]
pub mod seq_program_ctl_4;
#[doc = "SEQ_PROGRAM_CTL_5 register accessor: an alias for `Reg<SEQ_PROGRAM_CTL_5_SPEC>`"]
pub type SEQ_PROGRAM_CTL_5 = crate::Reg<seq_program_ctl_5::SEQ_PROGRAM_CTL_5_SPEC>;
#[doc = "Sequencer program control 5"]
pub mod seq_program_ctl_5;
