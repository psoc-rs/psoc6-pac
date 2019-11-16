#[doc = "IPC acquire\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acquire](acquire) module"]
pub type ACQUIRE = crate::Reg<u32, _ACQUIRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACQUIRE;
#[doc = "`read()` method returns [acquire::R](acquire::R) reader structure"]
impl crate::Readable for ACQUIRE {}
#[doc = "IPC acquire"]
pub mod acquire;
#[doc = "IPC release\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](release) module"]
pub type RELEASE = crate::Reg<u32, _RELEASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELEASE;
#[doc = "`write(|w| ..)` method takes [release::W](release::W) writer structure"]
impl crate::Writable for RELEASE {}
#[doc = "IPC release"]
pub mod release;
#[doc = "IPC notification\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [notify](notify) module"]
pub type NOTIFY = crate::Reg<u32, _NOTIFY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOTIFY;
#[doc = "`write(|w| ..)` method takes [notify::W](notify::W) writer structure"]
impl crate::Writable for NOTIFY {}
#[doc = "IPC notification"]
pub mod notify;
#[doc = "IPC data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "IPC data"]
pub mod data;
#[doc = "IPC lock status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_status](lock_status) module"]
pub type LOCK_STATUS = crate::Reg<u32, _LOCK_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK_STATUS;
#[doc = "`read()` method returns [lock_status::R](lock_status::R) reader structure"]
impl crate::Readable for LOCK_STATUS {}
#[doc = "IPC lock status"]
pub mod lock_status;
