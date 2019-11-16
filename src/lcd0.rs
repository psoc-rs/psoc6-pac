#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID & Revision"]
    pub id: ID,
    #[doc = "0x04 - LCD Divider Register"]
    pub divider: DIVIDER,
    #[doc = "0x08 - LCD Configuration Register"]
    pub control: CONTROL,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - LCD Pin Data Registers"]
    pub data0: [DATA0; 8],
    _reserved4: [u8; 224usize],
    #[doc = "0x200 - LCD Pin Data Registers"]
    pub data1: [DATA1; 8],
    _reserved5: [u8; 224usize],
    #[doc = "0x300 - LCD Pin Data Registers"]
    pub data2: [DATA2; 8],
    _reserved6: [u8; 224usize],
    #[doc = "0x400 - LCD Pin Data Registers"]
    pub data3: [DATA3; 8],
}
#[doc = "ID & Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "ID & Revision"]
pub mod id;
#[doc = "LCD Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divider](divider) module"]
pub type DIVIDER = crate::Reg<u32, _DIVIDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVIDER;
#[doc = "`read()` method returns [divider::R](divider::R) reader structure"]
impl crate::Readable for DIVIDER {}
#[doc = "`write(|w| ..)` method takes [divider::W](divider::W) writer structure"]
impl crate::Writable for DIVIDER {}
#[doc = "LCD Divider Register"]
pub mod divider;
#[doc = "LCD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "LCD Configuration Register"]
pub mod control;
#[doc = "LCD Pin Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](data0) module"]
pub type DATA0 = crate::Reg<u32, _DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0;
#[doc = "`read()` method returns [data0::R](data0::R) reader structure"]
impl crate::Readable for DATA0 {}
#[doc = "`write(|w| ..)` method takes [data0::W](data0::W) writer structure"]
impl crate::Writable for DATA0 {}
#[doc = "LCD Pin Data Registers"]
pub mod data0;
#[doc = "LCD Pin Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](data1) module"]
pub type DATA1 = crate::Reg<u32, _DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1;
#[doc = "`read()` method returns [data1::R](data1::R) reader structure"]
impl crate::Readable for DATA1 {}
#[doc = "`write(|w| ..)` method takes [data1::W](data1::W) writer structure"]
impl crate::Writable for DATA1 {}
#[doc = "LCD Pin Data Registers"]
pub mod data1;
#[doc = "LCD Pin Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2](data2) module"]
pub type DATA2 = crate::Reg<u32, _DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA2;
#[doc = "`read()` method returns [data2::R](data2::R) reader structure"]
impl crate::Readable for DATA2 {}
#[doc = "`write(|w| ..)` method takes [data2::W](data2::W) writer structure"]
impl crate::Writable for DATA2 {}
#[doc = "LCD Pin Data Registers"]
pub mod data2;
#[doc = "LCD Pin Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data3](data3) module"]
pub type DATA3 = crate::Reg<u32, _DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA3;
#[doc = "`read()` method returns [data3::R](data3::R) reader structure"]
impl crate::Readable for DATA3 {}
#[doc = "`write(|w| ..)` method takes [data3::W](data3::W) writer structure"]
impl crate::Writable for DATA3 {}
#[doc = "LCD Pin Data Registers"]
pub mod data3;
