#[doc = "Port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "Port output data register"]
pub mod out;
#[doc = "Port output data set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_clr](out_clr) module"]
pub type OUT_CLR = crate::Reg<u32, _OUT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CLR;
#[doc = "`read()` method returns [out_clr::R](out_clr::R) reader structure"]
impl crate::Readable for OUT_CLR {}
#[doc = "`write(|w| ..)` method takes [out_clr::W](out_clr::W) writer structure"]
impl crate::Writable for OUT_CLR {}
#[doc = "Port output data set register"]
pub mod out_clr;
#[doc = "Port output data clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_set](out_set) module"]
pub type OUT_SET = crate::Reg<u32, _OUT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_SET;
#[doc = "`read()` method returns [out_set::R](out_set::R) reader structure"]
impl crate::Readable for OUT_SET {}
#[doc = "`write(|w| ..)` method takes [out_set::W](out_set::W) writer structure"]
impl crate::Writable for OUT_SET {}
#[doc = "Port output data clear register"]
pub mod out_set;
#[doc = "Port output data invert register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out_inv](out_inv) module"]
pub type OUT_INV = crate::Reg<u32, _OUT_INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_INV;
#[doc = "`read()` method returns [out_inv::R](out_inv::R) reader structure"]
impl crate::Readable for OUT_INV {}
#[doc = "`write(|w| ..)` method takes [out_inv::W](out_inv::W) writer structure"]
impl crate::Writable for OUT_INV {}
#[doc = "Port output data invert register"]
pub mod out_inv;
#[doc = "Port input state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "Port input state register"]
pub mod in_;
#[doc = "Port interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "Port interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Port interrupt mask register"]
pub mod intr_mask;
#[doc = "Port interrupt masked status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_masked](intr_masked) module"]
pub type INTR_MASKED = crate::Reg<u32, _INTR_MASKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASKED;
#[doc = "`read()` method returns [intr_masked::R](intr_masked::R) reader structure"]
impl crate::Readable for INTR_MASKED {}
#[doc = "Port interrupt masked status register"]
pub mod intr_masked;
#[doc = "Port interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_set](intr_set) module"]
pub type INTR_SET = crate::Reg<u32, _INTR_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_SET;
#[doc = "`read()` method returns [intr_set::R](intr_set::R) reader structure"]
impl crate::Readable for INTR_SET {}
#[doc = "`write(|w| ..)` method takes [intr_set::W](intr_set::W) writer structure"]
impl crate::Writable for INTR_SET {}
#[doc = "Port interrupt set register"]
pub mod intr_set;
#[doc = "Port interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_cfg](intr_cfg) module"]
pub type INTR_CFG = crate::Reg<u32, _INTR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CFG;
#[doc = "`read()` method returns [intr_cfg::R](intr_cfg::R) reader structure"]
impl crate::Readable for INTR_CFG {}
#[doc = "`write(|w| ..)` method takes [intr_cfg::W](intr_cfg::W) writer structure"]
impl crate::Writable for INTR_CFG {}
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "Port configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Port configuration register"]
pub mod cfg;
#[doc = "Port input buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_in](cfg_in) module"]
pub type CFG_IN = crate::Reg<u32, _CFG_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_IN;
#[doc = "`read()` method returns [cfg_in::R](cfg_in::R) reader structure"]
impl crate::Readable for CFG_IN {}
#[doc = "`write(|w| ..)` method takes [cfg_in::W](cfg_in::W) writer structure"]
impl crate::Writable for CFG_IN {}
#[doc = "Port input buffer configuration register"]
pub mod cfg_in;
#[doc = "Port output buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_out](cfg_out) module"]
pub type CFG_OUT = crate::Reg<u32, _CFG_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_OUT;
#[doc = "`read()` method returns [cfg_out::R](cfg_out::R) reader structure"]
impl crate::Readable for CFG_OUT {}
#[doc = "`write(|w| ..)` method takes [cfg_out::W](cfg_out::W) writer structure"]
impl crate::Writable for CFG_OUT {}
#[doc = "Port output buffer configuration register"]
pub mod cfg_out;
#[doc = "Port SIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_sio](cfg_sio) module"]
pub type CFG_SIO = crate::Reg<u32, _CFG_SIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_SIO;
#[doc = "`read()` method returns [cfg_sio::R](cfg_sio::R) reader structure"]
impl crate::Readable for CFG_SIO {}
#[doc = "`write(|w| ..)` method takes [cfg_sio::W](cfg_sio::W) writer structure"]
impl crate::Writable for CFG_SIO {}
#[doc = "Port SIO configuration register"]
pub mod cfg_sio;
#[doc = "Port GPIO5V input buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg_in_gpio5v](cfg_in_gpio5v) module"]
pub type CFG_IN_GPIO5V = crate::Reg<u32, _CFG_IN_GPIO5V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG_IN_GPIO5V;
#[doc = "`read()` method returns [cfg_in_gpio5v::R](cfg_in_gpio5v::R) reader structure"]
impl crate::Readable for CFG_IN_GPIO5V {}
#[doc = "`write(|w| ..)` method takes [cfg_in_gpio5v::W](cfg_in_gpio5v::W) writer structure"]
impl crate::Writable for CFG_IN_GPIO5V {}
#[doc = "Port GPIO5V input buffer configuration register"]
pub mod cfg_in_gpio5v;
