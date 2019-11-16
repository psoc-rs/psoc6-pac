#[doc = "RCB LL control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RCB LL control register."]
pub mod ctrl;
#[doc = "Master interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Master interrupt request register."]
pub mod intr;
#[doc = "Master interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Master interrupt set request register"]
pub mod intr_set;
#[doc = "Master interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Master interrupt mask register."]
pub mod intr_mask;
#[doc = "Master interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Master interrupt masked request register"]
pub mod intr_masked;
#[doc = "Address of Register#1 in Radio (MDON)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg1_addr](radio_reg1_addr) module"]
pub type RADIO_REG1_ADDR = crate::Reg<u32, _RADIO_REG1_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADIO_REG1_ADDR;
#[doc = "`read()` method returns [radio_reg1_addr::R](radio_reg1_addr::R) reader structure"]
impl crate::Readable for RADIO_REG1_ADDR {}
#[doc = "`write(|w| ..)` method takes [radio_reg1_addr::W](radio_reg1_addr::W) writer structure"]
impl crate::Writable for RADIO_REG1_ADDR {}
#[doc = "Address of Register#1 in Radio (MDON)"]
pub mod radio_reg1_addr;
#[doc = "Address of Register#2 in Radio (RSSI)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg2_addr](radio_reg2_addr) module"]
pub type RADIO_REG2_ADDR = crate::Reg<u32, _RADIO_REG2_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADIO_REG2_ADDR;
#[doc = "`read()` method returns [radio_reg2_addr::R](radio_reg2_addr::R) reader structure"]
impl crate::Readable for RADIO_REG2_ADDR {}
#[doc = "`write(|w| ..)` method takes [radio_reg2_addr::W](radio_reg2_addr::W) writer structure"]
impl crate::Writable for RADIO_REG2_ADDR {}
#[doc = "Address of Register#2 in Radio (RSSI)"]
pub mod radio_reg2_addr;
#[doc = "Address of Register#3 in Radio (ACCL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg3_addr](radio_reg3_addr) module"]
pub type RADIO_REG3_ADDR = crate::Reg<u32, _RADIO_REG3_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADIO_REG3_ADDR;
#[doc = "`read()` method returns [radio_reg3_addr::R](radio_reg3_addr::R) reader structure"]
impl crate::Readable for RADIO_REG3_ADDR {}
#[doc = "`write(|w| ..)` method takes [radio_reg3_addr::W](radio_reg3_addr::W) writer structure"]
impl crate::Writable for RADIO_REG3_ADDR {}
#[doc = "Address of Register#3 in Radio (ACCL)"]
pub mod radio_reg3_addr;
#[doc = "Address of Register#4 in Radio (ACCH)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg4_addr](radio_reg4_addr) module"]
pub type RADIO_REG4_ADDR = crate::Reg<u32, _RADIO_REG4_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADIO_REG4_ADDR;
#[doc = "`read()` method returns [radio_reg4_addr::R](radio_reg4_addr::R) reader structure"]
impl crate::Readable for RADIO_REG4_ADDR {}
#[doc = "`write(|w| ..)` method takes [radio_reg4_addr::W](radio_reg4_addr::W) writer structure"]
impl crate::Writable for RADIO_REG4_ADDR {}
#[doc = "Address of Register#4 in Radio (ACCH)"]
pub mod radio_reg4_addr;
#[doc = "Address of Register#5 in Radio (RSSI ENERGY)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radio_reg5_addr](radio_reg5_addr) module"]
pub type RADIO_REG5_ADDR = crate::Reg<u32, _RADIO_REG5_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADIO_REG5_ADDR;
#[doc = "`read()` method returns [radio_reg5_addr::R](radio_reg5_addr::R) reader structure"]
impl crate::Readable for RADIO_REG5_ADDR {}
#[doc = "`write(|w| ..)` method takes [radio_reg5_addr::W](radio_reg5_addr::W) writer structure"]
impl crate::Writable for RADIO_REG5_ADDR {}
#[doc = "Address of Register#5 in Radio (RSSI ENERGY)"]
pub mod radio_reg5_addr;
#[doc = "N/A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_write_reg](cpu_write_reg) module"]
pub type CPU_WRITE_REG = crate::Reg<u32, _CPU_WRITE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_WRITE_REG;
#[doc = "`read()` method returns [cpu_write_reg::R](cpu_write_reg::R) reader structure"]
impl crate::Readable for CPU_WRITE_REG {}
#[doc = "`write(|w| ..)` method takes [cpu_write_reg::W](cpu_write_reg::W) writer structure"]
impl crate::Writable for CPU_WRITE_REG {}
#[doc = "N/A"]
pub mod cpu_write_reg;
#[doc = "N/A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_read_reg](cpu_read_reg) module"]
pub type CPU_READ_REG = crate::Reg<u32, _CPU_READ_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU_READ_REG;
#[doc = "`read()` method returns [cpu_read_reg::R](cpu_read_reg::R) reader structure"]
impl crate::Readable for CPU_READ_REG {}
#[doc = "`write(|w| ..)` method takes [cpu_read_reg::W](cpu_read_reg::W) writer structure"]
impl crate::Writable for CPU_READ_REG {}
#[doc = "N/A"]
pub mod cpu_read_reg;
