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
#[doc = "Device region base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Device region base address"]
pub mod addr;
#[doc = "Device region mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Device region mask"]
pub mod mask;
#[doc = "Address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_ctl](addr_ctl) module"]
pub type ADDR_CTL = crate::Reg<u32, _ADDR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR_CTL;
#[doc = "`read()` method returns [addr_ctl::R](addr_ctl::R) reader structure"]
impl crate::Readable for ADDR_CTL {}
#[doc = "`write(|w| ..)` method takes [addr_ctl::W](addr_ctl::W) writer structure"]
impl crate::Writable for ADDR_CTL {}
#[doc = "Address control"]
pub mod addr_ctl;
#[doc = "Read command control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_cmd_ctl](rd_cmd_ctl) module"]
pub type RD_CMD_CTL = crate::Reg<u32, _RD_CMD_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_CMD_CTL;
#[doc = "`read()` method returns [rd_cmd_ctl::R](rd_cmd_ctl::R) reader structure"]
impl crate::Readable for RD_CMD_CTL {}
#[doc = "`write(|w| ..)` method takes [rd_cmd_ctl::W](rd_cmd_ctl::W) writer structure"]
impl crate::Writable for RD_CMD_CTL {}
#[doc = "Read command control"]
pub mod rd_cmd_ctl;
#[doc = "Read address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_addr_ctl](rd_addr_ctl) module"]
pub type RD_ADDR_CTL = crate::Reg<u32, _RD_ADDR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_ADDR_CTL;
#[doc = "`read()` method returns [rd_addr_ctl::R](rd_addr_ctl::R) reader structure"]
impl crate::Readable for RD_ADDR_CTL {}
#[doc = "`write(|w| ..)` method takes [rd_addr_ctl::W](rd_addr_ctl::W) writer structure"]
impl crate::Writable for RD_ADDR_CTL {}
#[doc = "Read address control"]
pub mod rd_addr_ctl;
#[doc = "Read mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mode_ctl](rd_mode_ctl) module"]
pub type RD_MODE_CTL = crate::Reg<u32, _RD_MODE_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_MODE_CTL;
#[doc = "`read()` method returns [rd_mode_ctl::R](rd_mode_ctl::R) reader structure"]
impl crate::Readable for RD_MODE_CTL {}
#[doc = "`write(|w| ..)` method takes [rd_mode_ctl::W](rd_mode_ctl::W) writer structure"]
impl crate::Writable for RD_MODE_CTL {}
#[doc = "Read mode control"]
pub mod rd_mode_ctl;
#[doc = "Read dummy control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_dummy_ctl](rd_dummy_ctl) module"]
pub type RD_DUMMY_CTL = crate::Reg<u32, _RD_DUMMY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_DUMMY_CTL;
#[doc = "`read()` method returns [rd_dummy_ctl::R](rd_dummy_ctl::R) reader structure"]
impl crate::Readable for RD_DUMMY_CTL {}
#[doc = "`write(|w| ..)` method takes [rd_dummy_ctl::W](rd_dummy_ctl::W) writer structure"]
impl crate::Writable for RD_DUMMY_CTL {}
#[doc = "Read dummy control"]
pub mod rd_dummy_ctl;
#[doc = "Read data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_data_ctl](rd_data_ctl) module"]
pub type RD_DATA_CTL = crate::Reg<u32, _RD_DATA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RD_DATA_CTL;
#[doc = "`read()` method returns [rd_data_ctl::R](rd_data_ctl::R) reader structure"]
impl crate::Readable for RD_DATA_CTL {}
#[doc = "`write(|w| ..)` method takes [rd_data_ctl::W](rd_data_ctl::W) writer structure"]
impl crate::Writable for RD_DATA_CTL {}
#[doc = "Read data control"]
pub mod rd_data_ctl;
#[doc = "Write command control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_cmd_ctl](wr_cmd_ctl) module"]
pub type WR_CMD_CTL = crate::Reg<u32, _WR_CMD_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_CMD_CTL;
#[doc = "`read()` method returns [wr_cmd_ctl::R](wr_cmd_ctl::R) reader structure"]
impl crate::Readable for WR_CMD_CTL {}
#[doc = "`write(|w| ..)` method takes [wr_cmd_ctl::W](wr_cmd_ctl::W) writer structure"]
impl crate::Writable for WR_CMD_CTL {}
#[doc = "Write command control"]
pub mod wr_cmd_ctl;
#[doc = "Write address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_addr_ctl](wr_addr_ctl) module"]
pub type WR_ADDR_CTL = crate::Reg<u32, _WR_ADDR_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_ADDR_CTL;
#[doc = "`read()` method returns [wr_addr_ctl::R](wr_addr_ctl::R) reader structure"]
impl crate::Readable for WR_ADDR_CTL {}
#[doc = "`write(|w| ..)` method takes [wr_addr_ctl::W](wr_addr_ctl::W) writer structure"]
impl crate::Writable for WR_ADDR_CTL {}
#[doc = "Write address control"]
pub mod wr_addr_ctl;
#[doc = "Write mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_mode_ctl](wr_mode_ctl) module"]
pub type WR_MODE_CTL = crate::Reg<u32, _WR_MODE_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_MODE_CTL;
#[doc = "`read()` method returns [wr_mode_ctl::R](wr_mode_ctl::R) reader structure"]
impl crate::Readable for WR_MODE_CTL {}
#[doc = "`write(|w| ..)` method takes [wr_mode_ctl::W](wr_mode_ctl::W) writer structure"]
impl crate::Writable for WR_MODE_CTL {}
#[doc = "Write mode control"]
pub mod wr_mode_ctl;
#[doc = "Write dummy control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_dummy_ctl](wr_dummy_ctl) module"]
pub type WR_DUMMY_CTL = crate::Reg<u32, _WR_DUMMY_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_DUMMY_CTL;
#[doc = "`read()` method returns [wr_dummy_ctl::R](wr_dummy_ctl::R) reader structure"]
impl crate::Readable for WR_DUMMY_CTL {}
#[doc = "`write(|w| ..)` method takes [wr_dummy_ctl::W](wr_dummy_ctl::W) writer structure"]
impl crate::Writable for WR_DUMMY_CTL {}
#[doc = "Write dummy control"]
pub mod wr_dummy_ctl;
#[doc = "Write data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_data_ctl](wr_data_ctl) module"]
pub type WR_DATA_CTL = crate::Reg<u32, _WR_DATA_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WR_DATA_CTL;
#[doc = "`read()` method returns [wr_data_ctl::R](wr_data_ctl::R) reader structure"]
impl crate::Readable for WR_DATA_CTL {}
#[doc = "`write(|w| ..)` method takes [wr_data_ctl::W](wr_data_ctl::W) writer structure"]
impl crate::Writable for WR_DATA_CTL {}
#[doc = "Write data control"]
pub mod wr_data_ctl;
