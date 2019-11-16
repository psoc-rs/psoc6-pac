#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Command"]
    pub cmd: CMD,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Sequencer Default value"]
    pub seq_default: SEQ_DEFAULT,
    _reserved3: [u8; 28usize],
    #[doc = "0x40 - Sequencer read control 0"]
    pub seq_read_ctl_0: SEQ_READ_CTL_0,
    #[doc = "0x44 - Sequencer read control 1"]
    pub seq_read_ctl_1: SEQ_READ_CTL_1,
    #[doc = "0x48 - Sequencer read control 2"]
    pub seq_read_ctl_2: SEQ_READ_CTL_2,
    #[doc = "0x4c - Sequencer read control 3"]
    pub seq_read_ctl_3: SEQ_READ_CTL_3,
    #[doc = "0x50 - Sequencer read control 4"]
    pub seq_read_ctl_4: SEQ_READ_CTL_4,
    #[doc = "0x54 - Sequencer read control 5"]
    pub seq_read_ctl_5: SEQ_READ_CTL_5,
    _reserved9: [u8; 8usize],
    #[doc = "0x60 - Sequencer program control 0"]
    pub seq_program_ctl_0: SEQ_PROGRAM_CTL_0,
    #[doc = "0x64 - Sequencer program control 1"]
    pub seq_program_ctl_1: SEQ_PROGRAM_CTL_1,
    #[doc = "0x68 - Sequencer program control 2"]
    pub seq_program_ctl_2: SEQ_PROGRAM_CTL_2,
    #[doc = "0x6c - Sequencer program control 3"]
    pub seq_program_ctl_3: SEQ_PROGRAM_CTL_3,
    #[doc = "0x70 - Sequencer program control 4"]
    pub seq_program_ctl_4: SEQ_PROGRAM_CTL_4,
    #[doc = "0x74 - Sequencer program control 5"]
    pub seq_program_ctl_5: SEQ_PROGRAM_CTL_5,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command"]
pub mod cmd;
#[doc = "Sequencer Default value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_default](seq_default) module"]
pub type SEQ_DEFAULT = crate::Reg<u32, _SEQ_DEFAULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_DEFAULT;
#[doc = "`read()` method returns [seq_default::R](seq_default::R) reader structure"]
impl crate::Readable for SEQ_DEFAULT {}
#[doc = "`write(|w| ..)` method takes [seq_default::W](seq_default::W) writer structure"]
impl crate::Writable for SEQ_DEFAULT {}
#[doc = "Sequencer Default value"]
pub mod seq_default;
#[doc = "Sequencer read control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_0](seq_read_ctl_0) module"]
pub type SEQ_READ_CTL_0 = crate::Reg<u32, _SEQ_READ_CTL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_0;
#[doc = "`read()` method returns [seq_read_ctl_0::R](seq_read_ctl_0::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_0 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_0::W](seq_read_ctl_0::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_0 {}
#[doc = "Sequencer read control 0"]
pub mod seq_read_ctl_0;
#[doc = "Sequencer read control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_1](seq_read_ctl_1) module"]
pub type SEQ_READ_CTL_1 = crate::Reg<u32, _SEQ_READ_CTL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_1;
#[doc = "`read()` method returns [seq_read_ctl_1::R](seq_read_ctl_1::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_1 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_1::W](seq_read_ctl_1::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_1 {}
#[doc = "Sequencer read control 1"]
pub mod seq_read_ctl_1;
#[doc = "Sequencer read control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_2](seq_read_ctl_2) module"]
pub type SEQ_READ_CTL_2 = crate::Reg<u32, _SEQ_READ_CTL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_2;
#[doc = "`read()` method returns [seq_read_ctl_2::R](seq_read_ctl_2::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_2 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_2::W](seq_read_ctl_2::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_2 {}
#[doc = "Sequencer read control 2"]
pub mod seq_read_ctl_2;
#[doc = "Sequencer read control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_3](seq_read_ctl_3) module"]
pub type SEQ_READ_CTL_3 = crate::Reg<u32, _SEQ_READ_CTL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_3;
#[doc = "`read()` method returns [seq_read_ctl_3::R](seq_read_ctl_3::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_3 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_3::W](seq_read_ctl_3::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_3 {}
#[doc = "Sequencer read control 3"]
pub mod seq_read_ctl_3;
#[doc = "Sequencer read control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_4](seq_read_ctl_4) module"]
pub type SEQ_READ_CTL_4 = crate::Reg<u32, _SEQ_READ_CTL_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_4;
#[doc = "`read()` method returns [seq_read_ctl_4::R](seq_read_ctl_4::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_4 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_4::W](seq_read_ctl_4::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_4 {}
#[doc = "Sequencer read control 4"]
pub mod seq_read_ctl_4;
#[doc = "Sequencer read control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_5](seq_read_ctl_5) module"]
pub type SEQ_READ_CTL_5 = crate::Reg<u32, _SEQ_READ_CTL_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_READ_CTL_5;
#[doc = "`read()` method returns [seq_read_ctl_5::R](seq_read_ctl_5::R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_5 {}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_5::W](seq_read_ctl_5::W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_5 {}
#[doc = "Sequencer read control 5"]
pub mod seq_read_ctl_5;
#[doc = "Sequencer program control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_0](seq_program_ctl_0) module"]
pub type SEQ_PROGRAM_CTL_0 = crate::Reg<u32, _SEQ_PROGRAM_CTL_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_0;
#[doc = "`read()` method returns [seq_program_ctl_0::R](seq_program_ctl_0::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_0 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_0::W](seq_program_ctl_0::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_0 {}
#[doc = "Sequencer program control 0"]
pub mod seq_program_ctl_0;
#[doc = "Sequencer program control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_1](seq_program_ctl_1) module"]
pub type SEQ_PROGRAM_CTL_1 = crate::Reg<u32, _SEQ_PROGRAM_CTL_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_1;
#[doc = "`read()` method returns [seq_program_ctl_1::R](seq_program_ctl_1::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_1 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_1::W](seq_program_ctl_1::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_1 {}
#[doc = "Sequencer program control 1"]
pub mod seq_program_ctl_1;
#[doc = "Sequencer program control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_2](seq_program_ctl_2) module"]
pub type SEQ_PROGRAM_CTL_2 = crate::Reg<u32, _SEQ_PROGRAM_CTL_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_2;
#[doc = "`read()` method returns [seq_program_ctl_2::R](seq_program_ctl_2::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_2 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_2::W](seq_program_ctl_2::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_2 {}
#[doc = "Sequencer program control 2"]
pub mod seq_program_ctl_2;
#[doc = "Sequencer program control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_3](seq_program_ctl_3) module"]
pub type SEQ_PROGRAM_CTL_3 = crate::Reg<u32, _SEQ_PROGRAM_CTL_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_3;
#[doc = "`read()` method returns [seq_program_ctl_3::R](seq_program_ctl_3::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_3 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_3::W](seq_program_ctl_3::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_3 {}
#[doc = "Sequencer program control 3"]
pub mod seq_program_ctl_3;
#[doc = "Sequencer program control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_4](seq_program_ctl_4) module"]
pub type SEQ_PROGRAM_CTL_4 = crate::Reg<u32, _SEQ_PROGRAM_CTL_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_4;
#[doc = "`read()` method returns [seq_program_ctl_4::R](seq_program_ctl_4::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_4 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_4::W](seq_program_ctl_4::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_4 {}
#[doc = "Sequencer program control 4"]
pub mod seq_program_ctl_4;
#[doc = "Sequencer program control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_program_ctl_5](seq_program_ctl_5) module"]
pub type SEQ_PROGRAM_CTL_5 = crate::Reg<u32, _SEQ_PROGRAM_CTL_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ_PROGRAM_CTL_5;
#[doc = "`read()` method returns [seq_program_ctl_5::R](seq_program_ctl_5::R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_5 {}
#[doc = "`write(|w| ..)` method takes [seq_program_ctl_5::W](seq_program_ctl_5::W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_5 {}
#[doc = "Sequencer program control 5"]
pub mod seq_program_ctl_5;
