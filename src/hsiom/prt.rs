#[doc = "Port selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel0](port_sel0) module"]
pub type PORT_SEL0 = crate::Reg<u32, _PORT_SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_SEL0;
#[doc = "`read()` method returns [port_sel0::R](port_sel0::R) reader structure"]
impl crate::Readable for PORT_SEL0 {}
#[doc = "`write(|w| ..)` method takes [port_sel0::W](port_sel0::W) writer structure"]
impl crate::Writable for PORT_SEL0 {}
#[doc = "Port selection 0"]
pub mod port_sel0;
#[doc = "Port selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel1](port_sel1) module"]
pub type PORT_SEL1 = crate::Reg<u32, _PORT_SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_SEL1;
#[doc = "`read()` method returns [port_sel1::R](port_sel1::R) reader structure"]
impl crate::Readable for PORT_SEL1 {}
#[doc = "`write(|w| ..)` method takes [port_sel1::W](port_sel1::W) writer structure"]
impl crate::Writable for PORT_SEL1 {}
#[doc = "Port selection 1"]
pub mod port_sel1;
