#[doc = "Instruction Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command_register](command_register) module"]
pub type COMMAND_REGISTER = crate::Reg<u32, _COMMAND_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMMAND_REGISTER;
#[doc = "`write(|w| ..)` method takes [command_register::W](command_register::W) writer structure"]
impl crate::Writable for COMMAND_REGISTER {}
#[doc = "Instruction Register"]
pub mod command_register;
#[doc = "Event(Interrupt) status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_intr](event_intr) module"]
pub type EVENT_INTR = crate::Reg<u32, _EVENT_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENT_INTR;
#[doc = "`read()` method returns [event_intr::R](event_intr::R) reader structure"]
impl crate::Readable for EVENT_INTR {}
#[doc = "`write(|w| ..)` method takes [event_intr::W](event_intr::W) writer structure"]
impl crate::Writable for EVENT_INTR {}
#[doc = "Event(Interrupt) status and Clear register"]
pub mod event_intr;
#[doc = "Event indications enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event_enable](event_enable) module"]
pub type EVENT_ENABLE = crate::Reg<u32, _EVENT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENT_ENABLE;
#[doc = "`read()` method returns [event_enable::R](event_enable::R) reader structure"]
impl crate::Readable for EVENT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [event_enable::W](event_enable::W) writer structure"]
impl crate::Writable for EVENT_ENABLE {}
#[doc = "Event indications enable."]
pub mod event_enable;
#[doc = "Advertising parameters register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_params](adv_params) module"]
pub type ADV_PARAMS = crate::Reg<u32, _ADV_PARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_PARAMS;
#[doc = "`read()` method returns [adv_params::R](adv_params::R) reader structure"]
impl crate::Readable for ADV_PARAMS {}
#[doc = "`write(|w| ..)` method takes [adv_params::W](adv_params::W) writer structure"]
impl crate::Writable for ADV_PARAMS {}
#[doc = "Advertising parameters register."]
pub mod adv_params;
#[doc = "Advertising interval register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_interval_timeout](adv_interval_timeout) module"]
pub type ADV_INTERVAL_TIMEOUT = crate::Reg<u32, _ADV_INTERVAL_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_INTERVAL_TIMEOUT;
#[doc = "`read()` method returns [adv_interval_timeout::R](adv_interval_timeout::R) reader structure"]
impl crate::Readable for ADV_INTERVAL_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [adv_interval_timeout::W](adv_interval_timeout::W) writer structure"]
impl crate::Writable for ADV_INTERVAL_TIMEOUT {}
#[doc = "Advertising interval register."]
pub mod adv_interval_timeout;
#[doc = "Advertising interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_intr](adv_intr) module"]
pub type ADV_INTR = crate::Reg<u32, _ADV_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_INTR;
#[doc = "`read()` method returns [adv_intr::R](adv_intr::R) reader structure"]
impl crate::Readable for ADV_INTR {}
#[doc = "`write(|w| ..)` method takes [adv_intr::W](adv_intr::W) writer structure"]
impl crate::Writable for ADV_INTR {}
#[doc = "Advertising interrupt status and Clear register"]
pub mod adv_intr;
#[doc = "Advertising next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_next_instant](adv_next_instant) module"]
pub type ADV_NEXT_INSTANT = crate::Reg<u32, _ADV_NEXT_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_NEXT_INSTANT;
#[doc = "`read()` method returns [adv_next_instant::R](adv_next_instant::R) reader structure"]
impl crate::Readable for ADV_NEXT_INSTANT {}
#[doc = "Advertising next instant."]
pub mod adv_next_instant;
#[doc = "Scan Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_interval](scan_interval) module"]
pub type SCAN_INTERVAL = crate::Reg<u32, _SCAN_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_INTERVAL;
#[doc = "`read()` method returns [scan_interval::R](scan_interval::R) reader structure"]
impl crate::Readable for SCAN_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [scan_interval::W](scan_interval::W) writer structure"]
impl crate::Writable for SCAN_INTERVAL {}
#[doc = "Scan Interval Register"]
pub mod scan_interval;
#[doc = "Scan window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_window](scan_window) module"]
pub type SCAN_WINDOW = crate::Reg<u32, _SCAN_WINDOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_WINDOW;
#[doc = "`read()` method returns [scan_window::R](scan_window::R) reader structure"]
impl crate::Readable for SCAN_WINDOW {}
#[doc = "`write(|w| ..)` method takes [scan_window::W](scan_window::W) writer structure"]
impl crate::Writable for SCAN_WINDOW {}
#[doc = "Scan window Register"]
pub mod scan_window;
#[doc = "Scanning parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_param](scan_param) module"]
pub type SCAN_PARAM = crate::Reg<u32, _SCAN_PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_PARAM;
#[doc = "`read()` method returns [scan_param::R](scan_param::R) reader structure"]
impl crate::Readable for SCAN_PARAM {}
#[doc = "`write(|w| ..)` method takes [scan_param::W](scan_param::W) writer structure"]
impl crate::Writable for SCAN_PARAM {}
#[doc = "Scanning parameters register"]
pub mod scan_param;
#[doc = "Scan interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_intr](scan_intr) module"]
pub type SCAN_INTR = crate::Reg<u32, _SCAN_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_INTR;
#[doc = "`read()` method returns [scan_intr::R](scan_intr::R) reader structure"]
impl crate::Readable for SCAN_INTR {}
#[doc = "`write(|w| ..)` method takes [scan_intr::W](scan_intr::W) writer structure"]
impl crate::Writable for SCAN_INTR {}
#[doc = "Scan interrupt status and Clear register"]
pub mod scan_intr;
#[doc = "Advertising next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_next_instant](scan_next_instant) module"]
pub type SCAN_NEXT_INSTANT = crate::Reg<u32, _SCAN_NEXT_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_NEXT_INSTANT;
#[doc = "`read()` method returns [scan_next_instant::R](scan_next_instant::R) reader structure"]
impl crate::Readable for SCAN_NEXT_INSTANT {}
#[doc = "Advertising next instant."]
pub mod scan_next_instant;
#[doc = "Initiator Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_interval](init_interval) module"]
pub type INIT_INTERVAL = crate::Reg<u32, _INIT_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_INTERVAL;
#[doc = "`read()` method returns [init_interval::R](init_interval::R) reader structure"]
impl crate::Readable for INIT_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [init_interval::W](init_interval::W) writer structure"]
impl crate::Writable for INIT_INTERVAL {}
#[doc = "Initiator Interval Register"]
pub mod init_interval;
#[doc = "Initiator window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window](init_window) module"]
pub type INIT_WINDOW = crate::Reg<u32, _INIT_WINDOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_WINDOW;
#[doc = "`read()` method returns [init_window::R](init_window::R) reader structure"]
impl crate::Readable for INIT_WINDOW {}
#[doc = "`write(|w| ..)` method takes [init_window::W](init_window::W) writer structure"]
impl crate::Writable for INIT_WINDOW {}
#[doc = "Initiator window Register"]
pub mod init_window;
#[doc = "Initiator parameters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_param](init_param) module"]
pub type INIT_PARAM = crate::Reg<u32, _INIT_PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_PARAM;
#[doc = "`read()` method returns [init_param::R](init_param::R) reader structure"]
impl crate::Readable for INIT_PARAM {}
#[doc = "`write(|w| ..)` method takes [init_param::W](init_param::W) writer structure"]
impl crate::Writable for INIT_PARAM {}
#[doc = "Initiator parameters register"]
pub mod init_param;
#[doc = "Scan interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_intr](init_intr) module"]
pub type INIT_INTR = crate::Reg<u32, _INIT_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_INTR;
#[doc = "`read()` method returns [init_intr::R](init_intr::R) reader structure"]
impl crate::Readable for INIT_INTR {}
#[doc = "`write(|w| ..)` method takes [init_intr::W](init_intr::W) writer structure"]
impl crate::Writable for INIT_INTR {}
#[doc = "Scan interrupt status and Clear register"]
pub mod init_intr;
#[doc = "Initiator next instant.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_next_instant](init_next_instant) module"]
pub type INIT_NEXT_INSTANT = crate::Reg<u32, _INIT_NEXT_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_NEXT_INSTANT;
#[doc = "`read()` method returns [init_next_instant::R](init_next_instant::R) reader structure"]
impl crate::Readable for INIT_NEXT_INSTANT {}
#[doc = "Initiator next instant."]
pub mod init_next_instant;
#[doc = "Lower 16 bit random address of the device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_rand_addr_l](device_rand_addr_l) module"]
pub type DEVICE_RAND_ADDR_L = crate::Reg<u32, _DEVICE_RAND_ADDR_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_RAND_ADDR_L;
#[doc = "`read()` method returns [device_rand_addr_l::R](device_rand_addr_l::R) reader structure"]
impl crate::Readable for DEVICE_RAND_ADDR_L {}
#[doc = "`write(|w| ..)` method takes [device_rand_addr_l::W](device_rand_addr_l::W) writer structure"]
impl crate::Writable for DEVICE_RAND_ADDR_L {}
#[doc = "Lower 16 bit random address of the device."]
pub mod device_rand_addr_l;
#[doc = "Middle 16 bit random address of the device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_rand_addr_m](device_rand_addr_m) module"]
pub type DEVICE_RAND_ADDR_M = crate::Reg<u32, _DEVICE_RAND_ADDR_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_RAND_ADDR_M;
#[doc = "`read()` method returns [device_rand_addr_m::R](device_rand_addr_m::R) reader structure"]
impl crate::Readable for DEVICE_RAND_ADDR_M {}
#[doc = "`write(|w| ..)` method takes [device_rand_addr_m::W](device_rand_addr_m::W) writer structure"]
impl crate::Writable for DEVICE_RAND_ADDR_M {}
#[doc = "Middle 16 bit random address of the device."]
pub mod device_rand_addr_m;
#[doc = "Higher 16 bit random address of the device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_rand_addr_h](device_rand_addr_h) module"]
pub type DEVICE_RAND_ADDR_H = crate::Reg<u32, _DEVICE_RAND_ADDR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_RAND_ADDR_H;
#[doc = "`read()` method returns [device_rand_addr_h::R](device_rand_addr_h::R) reader structure"]
impl crate::Readable for DEVICE_RAND_ADDR_H {}
#[doc = "`write(|w| ..)` method takes [device_rand_addr_h::W](device_rand_addr_h::W) writer structure"]
impl crate::Writable for DEVICE_RAND_ADDR_H {}
#[doc = "Higher 16 bit random address of the device."]
pub mod device_rand_addr_h;
#[doc = "Lower 16 bit address of the peer device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_l](peer_addr_l) module"]
pub type PEER_ADDR_L = crate::Reg<u32, _PEER_ADDR_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_L;
#[doc = "`read()` method returns [peer_addr_l::R](peer_addr_l::R) reader structure"]
impl crate::Readable for PEER_ADDR_L {}
#[doc = "`write(|w| ..)` method takes [peer_addr_l::W](peer_addr_l::W) writer structure"]
impl crate::Writable for PEER_ADDR_L {}
#[doc = "Lower 16 bit address of the peer device."]
pub mod peer_addr_l;
#[doc = "Middle 16 bit address of the peer device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_m](peer_addr_m) module"]
pub type PEER_ADDR_M = crate::Reg<u32, _PEER_ADDR_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_M;
#[doc = "`read()` method returns [peer_addr_m::R](peer_addr_m::R) reader structure"]
impl crate::Readable for PEER_ADDR_M {}
#[doc = "`write(|w| ..)` method takes [peer_addr_m::W](peer_addr_m::W) writer structure"]
impl crate::Writable for PEER_ADDR_M {}
#[doc = "Middle 16 bit address of the peer device."]
pub mod peer_addr_m;
#[doc = "Higher 16 bit address of the peer device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_h](peer_addr_h) module"]
pub type PEER_ADDR_H = crate::Reg<u32, _PEER_ADDR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_H;
#[doc = "`read()` method returns [peer_addr_h::R](peer_addr_h::R) reader structure"]
impl crate::Readable for PEER_ADDR_H {}
#[doc = "`write(|w| ..)` method takes [peer_addr_h::W](peer_addr_h::W) writer structure"]
impl crate::Writable for PEER_ADDR_H {}
#[doc = "Higher 16 bit address of the peer device."]
pub mod peer_addr_h;
#[doc = "whitelist address type\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_addr_type](wl_addr_type) module"]
pub type WL_ADDR_TYPE = crate::Reg<u32, _WL_ADDR_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_ADDR_TYPE;
#[doc = "`read()` method returns [wl_addr_type::R](wl_addr_type::R) reader structure"]
impl crate::Readable for WL_ADDR_TYPE {}
#[doc = "`write(|w| ..)` method takes [wl_addr_type::W](wl_addr_type::W) writer structure"]
impl crate::Writable for WL_ADDR_TYPE {}
#[doc = "whitelist address type"]
pub mod wl_addr_type;
#[doc = "whitelist valid entry bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_enable](wl_enable) module"]
pub type WL_ENABLE = crate::Reg<u32, _WL_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_ENABLE;
#[doc = "`read()` method returns [wl_enable::R](wl_enable::R) reader structure"]
impl crate::Readable for WL_ENABLE {}
#[doc = "`write(|w| ..)` method takes [wl_enable::W](wl_enable::W) writer structure"]
impl crate::Writable for WL_ENABLE {}
#[doc = "whitelist valid entry bit"]
pub mod wl_enable;
#[doc = "Transmit window offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_window_offset](transmit_window_offset) module"]
pub type TRANSMIT_WINDOW_OFFSET = crate::Reg<u32, _TRANSMIT_WINDOW_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSMIT_WINDOW_OFFSET;
#[doc = "`read()` method returns [transmit_window_offset::R](transmit_window_offset::R) reader structure"]
impl crate::Readable for TRANSMIT_WINDOW_OFFSET {}
#[doc = "`write(|w| ..)` method takes [transmit_window_offset::W](transmit_window_offset::W) writer structure"]
impl crate::Writable for TRANSMIT_WINDOW_OFFSET {}
#[doc = "Transmit window offset"]
pub mod transmit_window_offset;
#[doc = "Transmit window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_window_size](transmit_window_size) module"]
pub type TRANSMIT_WINDOW_SIZE = crate::Reg<u32, _TRANSMIT_WINDOW_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSMIT_WINDOW_SIZE;
#[doc = "`read()` method returns [transmit_window_size::R](transmit_window_size::R) reader structure"]
impl crate::Readable for TRANSMIT_WINDOW_SIZE {}
#[doc = "`write(|w| ..)` method takes [transmit_window_size::W](transmit_window_size::W) writer structure"]
impl crate::Writable for TRANSMIT_WINDOW_SIZE {}
#[doc = "Transmit window size"]
pub mod transmit_window_size;
#[doc = "Data channel map 0 (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_l0](data_channels_l0) module"]
pub type DATA_CHANNELS_L0 = crate::Reg<u32, _DATA_CHANNELS_L0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_L0;
#[doc = "`read()` method returns [data_channels_l0::R](data_channels_l0::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_L0 {}
#[doc = "`write(|w| ..)` method takes [data_channels_l0::W](data_channels_l0::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_L0 {}
#[doc = "Data channel map 0 (lower word)"]
pub mod data_channels_l0;
#[doc = "Data channel map 0 (middle word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_m0](data_channels_m0) module"]
pub type DATA_CHANNELS_M0 = crate::Reg<u32, _DATA_CHANNELS_M0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_M0;
#[doc = "`read()` method returns [data_channels_m0::R](data_channels_m0::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_M0 {}
#[doc = "`write(|w| ..)` method takes [data_channels_m0::W](data_channels_m0::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_M0 {}
#[doc = "Data channel map 0 (middle word)"]
pub mod data_channels_m0;
#[doc = "Data channel map 0 (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_h0](data_channels_h0) module"]
pub type DATA_CHANNELS_H0 = crate::Reg<u32, _DATA_CHANNELS_H0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_H0;
#[doc = "`read()` method returns [data_channels_h0::R](data_channels_h0::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_H0 {}
#[doc = "`write(|w| ..)` method takes [data_channels_h0::W](data_channels_h0::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_H0 {}
#[doc = "Data channel map 0 (upper word)"]
pub mod data_channels_h0;
#[doc = "Data channel map 1 (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_l1](data_channels_l1) module"]
pub type DATA_CHANNELS_L1 = crate::Reg<u32, _DATA_CHANNELS_L1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_L1;
#[doc = "`read()` method returns [data_channels_l1::R](data_channels_l1::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_L1 {}
#[doc = "`write(|w| ..)` method takes [data_channels_l1::W](data_channels_l1::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_L1 {}
#[doc = "Data channel map 1 (lower word)"]
pub mod data_channels_l1;
#[doc = "Data channel map 1 (middle word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_m1](data_channels_m1) module"]
pub type DATA_CHANNELS_M1 = crate::Reg<u32, _DATA_CHANNELS_M1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_M1;
#[doc = "`read()` method returns [data_channels_m1::R](data_channels_m1::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_M1 {}
#[doc = "`write(|w| ..)` method takes [data_channels_m1::W](data_channels_m1::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_M1 {}
#[doc = "Data channel map 1 (middle word)"]
pub mod data_channels_m1;
#[doc = "Data channel map 1 (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_h1](data_channels_h1) module"]
pub type DATA_CHANNELS_H1 = crate::Reg<u32, _DATA_CHANNELS_H1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CHANNELS_H1;
#[doc = "`read()` method returns [data_channels_h1::R](data_channels_h1::R) reader structure"]
impl crate::Readable for DATA_CHANNELS_H1 {}
#[doc = "`write(|w| ..)` method takes [data_channels_h1::W](data_channels_h1::W) writer structure"]
impl crate::Writable for DATA_CHANNELS_H1 {}
#[doc = "Data channel map 1 (upper word)"]
pub mod data_channels_h1;
#[doc = "Connection interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_intr](conn_intr) module"]
pub type CONN_INTR = crate::Reg<u32, _CONN_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_INTR;
#[doc = "`read()` method returns [conn_intr::R](conn_intr::R) reader structure"]
impl crate::Readable for CONN_INTR {}
#[doc = "`write(|w| ..)` method takes [conn_intr::W](conn_intr::W) writer structure"]
impl crate::Writable for CONN_INTR {}
#[doc = "Connection interrupt status and Clear register"]
pub mod conn_intr;
#[doc = "Connection channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_status](conn_status) module"]
pub type CONN_STATUS = crate::Reg<u32, _CONN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_STATUS;
#[doc = "`read()` method returns [conn_status::R](conn_status::R) reader structure"]
impl crate::Readable for CONN_STATUS {}
#[doc = "Connection channel status"]
pub mod conn_status;
#[doc = "Connection Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_index](conn_index) module"]
pub type CONN_INDEX = crate::Reg<u32, _CONN_INDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_INDEX;
#[doc = "`read()` method returns [conn_index::R](conn_index::R) reader structure"]
impl crate::Readable for CONN_INDEX {}
#[doc = "`write(|w| ..)` method takes [conn_index::W](conn_index::W) writer structure"]
impl crate::Writable for CONN_INDEX {}
#[doc = "Connection Index register"]
pub mod conn_index;
#[doc = "Wakeup configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_config](wakeup_config) module"]
pub type WAKEUP_CONFIG = crate::Reg<u32, _WAKEUP_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_CONFIG;
#[doc = "`read()` method returns [wakeup_config::R](wakeup_config::R) reader structure"]
impl crate::Readable for WAKEUP_CONFIG {}
#[doc = "`write(|w| ..)` method takes [wakeup_config::W](wakeup_config::W) writer structure"]
impl crate::Writable for WAKEUP_CONFIG {}
#[doc = "Wakeup configuration"]
pub mod wakeup_config;
#[doc = "Wakeup control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_control](wakeup_control) module"]
pub type WAKEUP_CONTROL = crate::Reg<u32, _WAKEUP_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_CONTROL;
#[doc = "`read()` method returns [wakeup_control::R](wakeup_control::R) reader structure"]
impl crate::Readable for WAKEUP_CONTROL {}
#[doc = "`write(|w| ..)` method takes [wakeup_control::W](wakeup_control::W) writer structure"]
impl crate::Writable for WAKEUP_CONTROL {}
#[doc = "Wakeup control"]
pub mod wakeup_control;
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_config](clock_config) module"]
pub type CLOCK_CONFIG = crate::Reg<u32, _CLOCK_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CONFIG;
#[doc = "`read()` method returns [clock_config::R](clock_config::R) reader structure"]
impl crate::Readable for CLOCK_CONFIG {}
#[doc = "`write(|w| ..)` method takes [clock_config::W](clock_config::W) writer structure"]
impl crate::Writable for CLOCK_CONFIG {}
#[doc = "Clock control"]
pub mod clock_config;
#[doc = "Reference Clock\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_counter_l](tim_counter_l) module"]
pub type TIM_COUNTER_L = crate::Reg<u32, _TIM_COUNTER_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM_COUNTER_L;
#[doc = "`read()` method returns [tim_counter_l::R](tim_counter_l::R) reader structure"]
impl crate::Readable for TIM_COUNTER_L {}
#[doc = "Reference Clock"]
pub mod tim_counter_l;
#[doc = "Wakeup configuration extended\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_config_extd](wakeup_config_extd) module"]
pub type WAKEUP_CONFIG_EXTD = crate::Reg<u32, _WAKEUP_CONFIG_EXTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEUP_CONFIG_EXTD;
#[doc = "`read()` method returns [wakeup_config_extd::R](wakeup_config_extd::R) reader structure"]
impl crate::Readable for WAKEUP_CONFIG_EXTD {}
#[doc = "`write(|w| ..)` method takes [wakeup_config_extd::W](wakeup_config_extd::W) writer structure"]
impl crate::Writable for WAKEUP_CONFIG_EXTD {}
#[doc = "Wakeup configuration extended"]
pub mod wakeup_config_extd;
#[doc = "BLE Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poc_reg__tim_control](poc_reg__tim_control) module"]
pub type POC_REG__TIM_CONTROL = crate::Reg<u32, _POC_REG__TIM_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POC_REG__TIM_CONTROL;
#[doc = "`read()` method returns [poc_reg__tim_control::R](poc_reg__tim_control::R) reader structure"]
impl crate::Readable for POC_REG__TIM_CONTROL {}
#[doc = "`write(|w| ..)` method takes [poc_reg__tim_control::W](poc_reg__tim_control::W) writer structure"]
impl crate::Writable for POC_REG__TIM_CONTROL {}
#[doc = "BLE Time Control"]
pub mod poc_reg__tim_control;
#[doc = "Advertising data transmit FIFO. Access ADVCH_TX_FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_tx_data_fifo](adv_tx_data_fifo) module"]
pub type ADV_TX_DATA_FIFO = crate::Reg<u32, _ADV_TX_DATA_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_TX_DATA_FIFO;
#[doc = "`read()` method returns [adv_tx_data_fifo::R](adv_tx_data_fifo::R) reader structure"]
impl crate::Readable for ADV_TX_DATA_FIFO {}
#[doc = "`write(|w| ..)` method takes [adv_tx_data_fifo::W](adv_tx_data_fifo::W) writer structure"]
impl crate::Writable for ADV_TX_DATA_FIFO {}
#[doc = "Advertising data transmit FIFO. Access ADVCH_TX_FIFO."]
pub mod adv_tx_data_fifo;
#[doc = "Advertising scan response data transmit FIFO. Access ADVCH_TX_FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_scn_rsp_tx_fifo](adv_scn_rsp_tx_fifo) module"]
pub type ADV_SCN_RSP_TX_FIFO = crate::Reg<u32, _ADV_SCN_RSP_TX_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_SCN_RSP_TX_FIFO;
#[doc = "`read()` method returns [adv_scn_rsp_tx_fifo::R](adv_scn_rsp_tx_fifo::R) reader structure"]
impl crate::Readable for ADV_SCN_RSP_TX_FIFO {}
#[doc = "`write(|w| ..)` method takes [adv_scn_rsp_tx_fifo::W](adv_scn_rsp_tx_fifo::W) writer structure"]
impl crate::Writable for ADV_SCN_RSP_TX_FIFO {}
#[doc = "Advertising scan response data transmit FIFO. Access ADVCH_TX_FIFO."]
pub mod adv_scn_rsp_tx_fifo;
#[doc = "advertising scan response data receive data FIFO. Access ADVRX_FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_scn_adv_rx_fifo](init_scn_adv_rx_fifo) module"]
pub type INIT_SCN_ADV_RX_FIFO = crate::Reg<u32, _INIT_SCN_ADV_RX_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_SCN_ADV_RX_FIFO;
#[doc = "`read()` method returns [init_scn_adv_rx_fifo::R](init_scn_adv_rx_fifo::R) reader structure"]
impl crate::Readable for INIT_SCN_ADV_RX_FIFO {}
#[doc = "advertising scan response data receive data FIFO. Access ADVRX_FIFO."]
pub mod init_scn_adv_rx_fifo;
#[doc = "Connection Interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_interval](conn_interval) module"]
pub type CONN_INTERVAL = crate::Reg<u32, _CONN_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_INTERVAL;
#[doc = "`read()` method returns [conn_interval::R](conn_interval::R) reader structure"]
impl crate::Readable for CONN_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [conn_interval::W](conn_interval::W) writer structure"]
impl crate::Writable for CONN_INTERVAL {}
#[doc = "Connection Interval"]
pub mod conn_interval;
#[doc = "Supervision timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sup_timeout](sup_timeout) module"]
pub type SUP_TIMEOUT = crate::Reg<u32, _SUP_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUP_TIMEOUT;
#[doc = "`read()` method returns [sup_timeout::R](sup_timeout::R) reader structure"]
impl crate::Readable for SUP_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [sup_timeout::W](sup_timeout::W) writer structure"]
impl crate::Writable for SUP_TIMEOUT {}
#[doc = "Supervision timeout"]
pub mod sup_timeout;
#[doc = "Slave Latency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_latency](slave_latency) module"]
pub type SLAVE_LATENCY = crate::Reg<u32, _SLAVE_LATENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_LATENCY;
#[doc = "`read()` method returns [slave_latency::R](slave_latency::R) reader structure"]
impl crate::Readable for SLAVE_LATENCY {}
#[doc = "`write(|w| ..)` method takes [slave_latency::W](slave_latency::W) writer structure"]
impl crate::Writable for SLAVE_LATENCY {}
#[doc = "Slave Latency"]
pub mod slave_latency;
#[doc = "Connection event length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_length](ce_length) module"]
pub type CE_LENGTH = crate::Reg<u32, _CE_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CE_LENGTH;
#[doc = "`read()` method returns [ce_length::R](ce_length::R) reader structure"]
impl crate::Readable for CE_LENGTH {}
#[doc = "`write(|w| ..)` method takes [ce_length::W](ce_length::W) writer structure"]
impl crate::Writable for CE_LENGTH {}
#[doc = "Connection event length"]
pub mod ce_length;
#[doc = "Access address (lower)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdu_access_addr_l_register](pdu_access_addr_l_register) module"]
pub type PDU_ACCESS_ADDR_L_REGISTER = crate::Reg<u32, _PDU_ACCESS_ADDR_L_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDU_ACCESS_ADDR_L_REGISTER;
#[doc = "`read()` method returns [pdu_access_addr_l_register::R](pdu_access_addr_l_register::R) reader structure"]
impl crate::Readable for PDU_ACCESS_ADDR_L_REGISTER {}
#[doc = "`write(|w| ..)` method takes [pdu_access_addr_l_register::W](pdu_access_addr_l_register::W) writer structure"]
impl crate::Writable for PDU_ACCESS_ADDR_L_REGISTER {}
#[doc = "Access address (lower)"]
pub mod pdu_access_addr_l_register;
#[doc = "Access address (upper)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdu_access_addr_h_register](pdu_access_addr_h_register) module"]
pub type PDU_ACCESS_ADDR_H_REGISTER = crate::Reg<u32, _PDU_ACCESS_ADDR_H_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDU_ACCESS_ADDR_H_REGISTER;
#[doc = "`read()` method returns [pdu_access_addr_h_register::R](pdu_access_addr_h_register::R) reader structure"]
impl crate::Readable for PDU_ACCESS_ADDR_H_REGISTER {}
#[doc = "`write(|w| ..)` method takes [pdu_access_addr_h_register::W](pdu_access_addr_h_register::W) writer structure"]
impl crate::Writable for PDU_ACCESS_ADDR_H_REGISTER {}
#[doc = "Access address (upper)"]
pub mod pdu_access_addr_h_register;
#[doc = "Connection event instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ce_instant](conn_ce_instant) module"]
pub type CONN_CE_INSTANT = crate::Reg<u32, _CONN_CE_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CE_INSTANT;
#[doc = "`read()` method returns [conn_ce_instant::R](conn_ce_instant::R) reader structure"]
impl crate::Readable for CONN_CE_INSTANT {}
#[doc = "`write(|w| ..)` method takes [conn_ce_instant::W](conn_ce_instant::W) writer structure"]
impl crate::Writable for CONN_CE_INSTANT {}
#[doc = "Connection event instant"]
pub mod conn_ce_instant;
#[doc = "connection configuration & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cnfg_sts_register](ce_cnfg_sts_register) module"]
pub type CE_CNFG_STS_REGISTER = crate::Reg<u32, _CE_CNFG_STS_REGISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CE_CNFG_STS_REGISTER;
#[doc = "`read()` method returns [ce_cnfg_sts_register::R](ce_cnfg_sts_register::R) reader structure"]
impl crate::Readable for CE_CNFG_STS_REGISTER {}
#[doc = "`write(|w| ..)` method takes [ce_cnfg_sts_register::W](ce_cnfg_sts_register::W) writer structure"]
impl crate::Writable for CE_CNFG_STS_REGISTER {}
#[doc = "connection configuration & status register"]
pub mod ce_cnfg_sts_register;
#[doc = "Next connection event instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_ce_instant](next_ce_instant) module"]
pub type NEXT_CE_INSTANT = crate::Reg<u32, _NEXT_CE_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_CE_INSTANT;
#[doc = "`read()` method returns [next_ce_instant::R](next_ce_instant::R) reader structure"]
impl crate::Readable for NEXT_CE_INSTANT {}
#[doc = "Next connection event instant"]
pub mod next_ce_instant;
#[doc = "connection event counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ce_counter](conn_ce_counter) module"]
pub type CONN_CE_COUNTER = crate::Reg<u32, _CONN_CE_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CE_COUNTER;
#[doc = "`read()` method returns [conn_ce_counter::R](conn_ce_counter::R) reader structure"]
impl crate::Readable for CONN_CE_COUNTER {}
#[doc = "connection event counter"]
pub mod conn_ce_counter;
#[doc = "data list sent update and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_list_sent_update__status](data_list_sent_update__status) module"]
pub type DATA_LIST_SENT_UPDATE__STATUS = crate::Reg<u32, _DATA_LIST_SENT_UPDATE__STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_LIST_SENT_UPDATE__STATUS;
#[doc = "`read()` method returns [data_list_sent_update__status::R](data_list_sent_update__status::R) reader structure"]
impl crate::Readable for DATA_LIST_SENT_UPDATE__STATUS {}
#[doc = "`write(|w| ..)` method takes [data_list_sent_update__status::W](data_list_sent_update__status::W) writer structure"]
impl crate::Writable for DATA_LIST_SENT_UPDATE__STATUS {}
#[doc = "data list sent update and status"]
pub mod data_list_sent_update__status;
#[doc = "data list ack update and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_list_ack_update__status](data_list_ack_update__status) module"]
pub type DATA_LIST_ACK_UPDATE__STATUS = crate::Reg<u32, _DATA_LIST_ACK_UPDATE__STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_LIST_ACK_UPDATE__STATUS;
#[doc = "`read()` method returns [data_list_ack_update__status::R](data_list_ack_update__status::R) reader structure"]
impl crate::Readable for DATA_LIST_ACK_UPDATE__STATUS {}
#[doc = "`write(|w| ..)` method takes [data_list_ack_update__status::W](data_list_ack_update__status::W) writer structure"]
impl crate::Writable for DATA_LIST_ACK_UPDATE__STATUS {}
#[doc = "data list ack update and status"]
pub mod data_list_ack_update__status;
#[doc = "connection configuration & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_cnfg_sts_register_ext](ce_cnfg_sts_register_ext) module"]
pub type CE_CNFG_STS_REGISTER_EXT = crate::Reg<u32, _CE_CNFG_STS_REGISTER_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CE_CNFG_STS_REGISTER_EXT;
#[doc = "`read()` method returns [ce_cnfg_sts_register_ext::R](ce_cnfg_sts_register_ext::R) reader structure"]
impl crate::Readable for CE_CNFG_STS_REGISTER_EXT {}
#[doc = "`write(|w| ..)` method takes [ce_cnfg_sts_register_ext::W](ce_cnfg_sts_register_ext::W) writer structure"]
impl crate::Writable for CE_CNFG_STS_REGISTER_EXT {}
#[doc = "connection configuration & status register"]
pub mod ce_cnfg_sts_register_ext;
#[doc = "Connection extended interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ext_intr](conn_ext_intr) module"]
pub type CONN_EXT_INTR = crate::Reg<u32, _CONN_EXT_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_EXT_INTR;
#[doc = "`read()` method returns [conn_ext_intr::R](conn_ext_intr::R) reader structure"]
impl crate::Readable for CONN_EXT_INTR {}
#[doc = "`write(|w| ..)` method takes [conn_ext_intr::W](conn_ext_intr::W) writer structure"]
impl crate::Writable for CONN_EXT_INTR {}
#[doc = "Connection extended interrupt status and Clear register"]
pub mod conn_ext_intr;
#[doc = "Connection Extended Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ext_intr_mask](conn_ext_intr_mask) module"]
pub type CONN_EXT_INTR_MASK = crate::Reg<u32, _CONN_EXT_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_EXT_INTR_MASK;
#[doc = "`read()` method returns [conn_ext_intr_mask::R](conn_ext_intr_mask::R) reader structure"]
impl crate::Readable for CONN_EXT_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [conn_ext_intr_mask::W](conn_ext_intr_mask::W) writer structure"]
impl crate::Writable for CONN_EXT_INTR_MASK {}
#[doc = "Connection Extended Interrupt mask"]
pub mod conn_ext_intr_mask;
#[doc = "Data buffer descriptor 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_mem_descriptor](data_mem_descriptor) module"]
pub type DATA_MEM_DESCRIPTOR = crate::Reg<u32, _DATA_MEM_DESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_MEM_DESCRIPTOR;
#[doc = "`read()` method returns [data_mem_descriptor::R](data_mem_descriptor::R) reader structure"]
impl crate::Readable for DATA_MEM_DESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [data_mem_descriptor::W](data_mem_descriptor::W) writer structure"]
impl crate::Writable for DATA_MEM_DESCRIPTOR {}
#[doc = "Data buffer descriptor 0 to 4"]
pub mod data_mem_descriptor;
#[doc = "Window widen for interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [window_widen_intvl](window_widen_intvl) module"]
pub type WINDOW_WIDEN_INTVL = crate::Reg<u32, _WINDOW_WIDEN_INTVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINDOW_WIDEN_INTVL;
#[doc = "`read()` method returns [window_widen_intvl::R](window_widen_intvl::R) reader structure"]
impl crate::Readable for WINDOW_WIDEN_INTVL {}
#[doc = "`write(|w| ..)` method takes [window_widen_intvl::W](window_widen_intvl::W) writer structure"]
impl crate::Writable for WINDOW_WIDEN_INTVL {}
#[doc = "Window widen for interval"]
pub mod window_widen_intvl;
#[doc = "Window widen for offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [window_widen_winoff](window_widen_winoff) module"]
pub type WINDOW_WIDEN_WINOFF = crate::Reg<u32, _WINDOW_WIDEN_WINOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINDOW_WIDEN_WINOFF;
#[doc = "`read()` method returns [window_widen_winoff::R](window_widen_winoff::R) reader structure"]
impl crate::Readable for WINDOW_WIDEN_WINOFF {}
#[doc = "`write(|w| ..)` method takes [window_widen_winoff::W](window_widen_winoff::W) writer structure"]
impl crate::Writable for WINDOW_WIDEN_WINOFF {}
#[doc = "Window widen for offset"]
pub mod window_widen_winoff;
#[doc = "Direct Test Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_rf_test_mode](le_rf_test_mode) module"]
pub type LE_RF_TEST_MODE = crate::Reg<u32, _LE_RF_TEST_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_RF_TEST_MODE;
#[doc = "`read()` method returns [le_rf_test_mode::R](le_rf_test_mode::R) reader structure"]
impl crate::Readable for LE_RF_TEST_MODE {}
#[doc = "`write(|w| ..)` method takes [le_rf_test_mode::W](le_rf_test_mode::W) writer structure"]
impl crate::Writable for LE_RF_TEST_MODE {}
#[doc = "Direct Test Mode control"]
pub mod le_rf_test_mode;
#[doc = "Direct Test Mode receive packet count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtm_rx_pkt_count](dtm_rx_pkt_count) module"]
pub type DTM_RX_PKT_COUNT = crate::Reg<u32, _DTM_RX_PKT_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTM_RX_PKT_COUNT;
#[doc = "`read()` method returns [dtm_rx_pkt_count::R](dtm_rx_pkt_count::R) reader structure"]
impl crate::Readable for DTM_RX_PKT_COUNT {}
#[doc = "Direct Test Mode receive packet count"]
pub mod dtm_rx_pkt_count;
#[doc = "Direct Test Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_rf_test_mode_ext](le_rf_test_mode_ext) module"]
pub type LE_RF_TEST_MODE_EXT = crate::Reg<u32, _LE_RF_TEST_MODE_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_RF_TEST_MODE_EXT;
#[doc = "`read()` method returns [le_rf_test_mode_ext::R](le_rf_test_mode_ext::R) reader structure"]
impl crate::Readable for LE_RF_TEST_MODE_EXT {}
#[doc = "`write(|w| ..)` method takes [le_rf_test_mode_ext::W](le_rf_test_mode_ext::W) writer structure"]
impl crate::Writable for LE_RF_TEST_MODE_EXT {}
#[doc = "Direct Test Mode control"]
pub mod le_rf_test_mode_ext;
#[doc = "Channel Address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrx_hop](txrx_hop) module"]
pub type TXRX_HOP = crate::Reg<u32, _TXRX_HOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXRX_HOP;
#[doc = "`read()` method returns [txrx_hop::R](txrx_hop::R) reader structure"]
impl crate::Readable for TXRX_HOP {}
#[doc = "Channel Address register"]
pub mod txrx_hop;
#[doc = "Transmit/Receive data delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_on_delay](tx_rx_on_delay) module"]
pub type TX_RX_ON_DELAY = crate::Reg<u32, _TX_RX_ON_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_RX_ON_DELAY;
#[doc = "`read()` method returns [tx_rx_on_delay::R](tx_rx_on_delay::R) reader structure"]
impl crate::Readable for TX_RX_ON_DELAY {}
#[doc = "`write(|w| ..)` method takes [tx_rx_on_delay::W](tx_rx_on_delay::W) writer structure"]
impl crate::Writable for TX_RX_ON_DELAY {}
#[doc = "Transmit/Receive data delay"]
pub mod tx_rx_on_delay;
#[doc = "ADV packet access code low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_accaddr_l](adv_accaddr_l) module"]
pub type ADV_ACCADDR_L = crate::Reg<u32, _ADV_ACCADDR_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_ACCADDR_L;
#[doc = "`read()` method returns [adv_accaddr_l::R](adv_accaddr_l::R) reader structure"]
impl crate::Readable for ADV_ACCADDR_L {}
#[doc = "`write(|w| ..)` method takes [adv_accaddr_l::W](adv_accaddr_l::W) writer structure"]
impl crate::Writable for ADV_ACCADDR_L {}
#[doc = "ADV packet access code low word"]
pub mod adv_accaddr_l;
#[doc = "ADV packet access code high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_accaddr_h](adv_accaddr_h) module"]
pub type ADV_ACCADDR_H = crate::Reg<u32, _ADV_ACCADDR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_ACCADDR_H;
#[doc = "`read()` method returns [adv_accaddr_h::R](adv_accaddr_h::R) reader structure"]
impl crate::Readable for ADV_ACCADDR_H {}
#[doc = "`write(|w| ..)` method takes [adv_accaddr_h::W](adv_accaddr_h::W) writer structure"]
impl crate::Writable for ADV_ACCADDR_H {}
#[doc = "ADV packet access code high word"]
pub mod adv_accaddr_h;
#[doc = "Advertising channel transmit power setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_ch_tx_power_lvl_ls](adv_ch_tx_power_lvl_ls) module"]
pub type ADV_CH_TX_POWER_LVL_LS = crate::Reg<u32, _ADV_CH_TX_POWER_LVL_LS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_CH_TX_POWER_LVL_LS;
#[doc = "`read()` method returns [adv_ch_tx_power_lvl_ls::R](adv_ch_tx_power_lvl_ls::R) reader structure"]
impl crate::Readable for ADV_CH_TX_POWER_LVL_LS {}
#[doc = "`write(|w| ..)` method takes [adv_ch_tx_power_lvl_ls::W](adv_ch_tx_power_lvl_ls::W) writer structure"]
impl crate::Writable for ADV_CH_TX_POWER_LVL_LS {}
#[doc = "Advertising channel transmit power setting"]
pub mod adv_ch_tx_power_lvl_ls;
#[doc = "Advertising channel transmit power setting extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_ch_tx_power_lvl_ms](adv_ch_tx_power_lvl_ms) module"]
pub type ADV_CH_TX_POWER_LVL_MS = crate::Reg<u32, _ADV_CH_TX_POWER_LVL_MS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_CH_TX_POWER_LVL_MS;
#[doc = "`read()` method returns [adv_ch_tx_power_lvl_ms::R](adv_ch_tx_power_lvl_ms::R) reader structure"]
impl crate::Readable for ADV_CH_TX_POWER_LVL_MS {}
#[doc = "`write(|w| ..)` method takes [adv_ch_tx_power_lvl_ms::W](adv_ch_tx_power_lvl_ms::W) writer structure"]
impl crate::Writable for ADV_CH_TX_POWER_LVL_MS {}
#[doc = "Advertising channel transmit power setting extension"]
pub mod adv_ch_tx_power_lvl_ms;
#[doc = "Connection channel transmit power setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ch_tx_power_lvl_ls](conn_ch_tx_power_lvl_ls) module"]
pub type CONN_CH_TX_POWER_LVL_LS = crate::Reg<u32, _CONN_CH_TX_POWER_LVL_LS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CH_TX_POWER_LVL_LS;
#[doc = "`read()` method returns [conn_ch_tx_power_lvl_ls::R](conn_ch_tx_power_lvl_ls::R) reader structure"]
impl crate::Readable for CONN_CH_TX_POWER_LVL_LS {}
#[doc = "`write(|w| ..)` method takes [conn_ch_tx_power_lvl_ls::W](conn_ch_tx_power_lvl_ls::W) writer structure"]
impl crate::Writable for CONN_CH_TX_POWER_LVL_LS {}
#[doc = "Connection channel transmit power setting"]
pub mod conn_ch_tx_power_lvl_ls;
#[doc = "Connection channel transmit power setting extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ch_tx_power_lvl_ms](conn_ch_tx_power_lvl_ms) module"]
pub type CONN_CH_TX_POWER_LVL_MS = crate::Reg<u32, _CONN_CH_TX_POWER_LVL_MS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CH_TX_POWER_LVL_MS;
#[doc = "`read()` method returns [conn_ch_tx_power_lvl_ms::R](conn_ch_tx_power_lvl_ms::R) reader structure"]
impl crate::Readable for CONN_CH_TX_POWER_LVL_MS {}
#[doc = "`write(|w| ..)` method takes [conn_ch_tx_power_lvl_ms::W](conn_ch_tx_power_lvl_ms::W) writer structure"]
impl crate::Writable for CONN_CH_TX_POWER_LVL_MS {}
#[doc = "Connection channel transmit power setting extension"]
pub mod conn_ch_tx_power_lvl_ms;
#[doc = "Device public address lower register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pub_addr_l](dev_pub_addr_l) module"]
pub type DEV_PUB_ADDR_L = crate::Reg<u32, _DEV_PUB_ADDR_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PUB_ADDR_L;
#[doc = "`read()` method returns [dev_pub_addr_l::R](dev_pub_addr_l::R) reader structure"]
impl crate::Readable for DEV_PUB_ADDR_L {}
#[doc = "`write(|w| ..)` method takes [dev_pub_addr_l::W](dev_pub_addr_l::W) writer structure"]
impl crate::Writable for DEV_PUB_ADDR_L {}
#[doc = "Device public address lower register"]
pub mod dev_pub_addr_l;
#[doc = "Device public address middle register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pub_addr_m](dev_pub_addr_m) module"]
pub type DEV_PUB_ADDR_M = crate::Reg<u32, _DEV_PUB_ADDR_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PUB_ADDR_M;
#[doc = "`read()` method returns [dev_pub_addr_m::R](dev_pub_addr_m::R) reader structure"]
impl crate::Readable for DEV_PUB_ADDR_M {}
#[doc = "`write(|w| ..)` method takes [dev_pub_addr_m::W](dev_pub_addr_m::W) writer structure"]
impl crate::Writable for DEV_PUB_ADDR_M {}
#[doc = "Device public address middle register"]
pub mod dev_pub_addr_m;
#[doc = "Device public address higher register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pub_addr_h](dev_pub_addr_h) module"]
pub type DEV_PUB_ADDR_H = crate::Reg<u32, _DEV_PUB_ADDR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PUB_ADDR_H;
#[doc = "`read()` method returns [dev_pub_addr_h::R](dev_pub_addr_h::R) reader structure"]
impl crate::Readable for DEV_PUB_ADDR_H {}
#[doc = "`write(|w| ..)` method takes [dev_pub_addr_h::W](dev_pub_addr_h::W) writer structure"]
impl crate::Writable for DEV_PUB_ADDR_H {}
#[doc = "Device public address higher register"]
pub mod dev_pub_addr_h;
#[doc = "Offset to first instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offset_to_first_instant](offset_to_first_instant) module"]
pub type OFFSET_TO_FIRST_INSTANT = crate::Reg<u32, _OFFSET_TO_FIRST_INSTANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFSET_TO_FIRST_INSTANT;
#[doc = "`read()` method returns [offset_to_first_instant::R](offset_to_first_instant::R) reader structure"]
impl crate::Readable for OFFSET_TO_FIRST_INSTANT {}
#[doc = "`write(|w| ..)` method takes [offset_to_first_instant::W](offset_to_first_instant::W) writer structure"]
impl crate::Writable for OFFSET_TO_FIRST_INSTANT {}
#[doc = "Offset to first instant"]
pub mod offset_to_first_instant;
#[doc = "Advertiser configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_config](adv_config) module"]
pub type ADV_CONFIG = crate::Reg<u32, _ADV_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_CONFIG;
#[doc = "`read()` method returns [adv_config::R](adv_config::R) reader structure"]
impl crate::Readable for ADV_CONFIG {}
#[doc = "`write(|w| ..)` method takes [adv_config::W](adv_config::W) writer structure"]
impl crate::Writable for ADV_CONFIG {}
#[doc = "Advertiser configuration register"]
pub mod adv_config;
#[doc = "Scan configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_config](scan_config) module"]
pub type SCAN_CONFIG = crate::Reg<u32, _SCAN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAN_CONFIG;
#[doc = "`read()` method returns [scan_config::R](scan_config::R) reader structure"]
impl crate::Readable for SCAN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [scan_config::W](scan_config::W) writer structure"]
impl crate::Writable for SCAN_CONFIG {}
#[doc = "Scan configuration register"]
pub mod scan_config;
#[doc = "Initiator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_config](init_config) module"]
pub type INIT_CONFIG = crate::Reg<u32, _INIT_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_CONFIG;
#[doc = "`read()` method returns [init_config::R](init_config::R) reader structure"]
impl crate::Readable for INIT_CONFIG {}
#[doc = "`write(|w| ..)` method takes [init_config::W](init_config::W) writer structure"]
impl crate::Writable for INIT_CONFIG {}
#[doc = "Initiator configuration register"]
pub mod init_config;
#[doc = "Connection configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_config](conn_config) module"]
pub type CONN_CONFIG = crate::Reg<u32, _CONN_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CONFIG;
#[doc = "`read()` method returns [conn_config::R](conn_config::R) reader structure"]
impl crate::Readable for CONN_CONFIG {}
#[doc = "`write(|w| ..)` method takes [conn_config::W](conn_config::W) writer structure"]
impl crate::Writable for CONN_CONFIG {}
#[doc = "Connection configuration register"]
pub mod conn_config;
#[doc = "Connection parameter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param1](conn_param1) module"]
pub type CONN_PARAM1 = crate::Reg<u32, _CONN_PARAM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_PARAM1;
#[doc = "`read()` method returns [conn_param1::R](conn_param1::R) reader structure"]
impl crate::Readable for CONN_PARAM1 {}
#[doc = "`write(|w| ..)` method takes [conn_param1::W](conn_param1::W) writer structure"]
impl crate::Writable for CONN_PARAM1 {}
#[doc = "Connection parameter 1"]
pub mod conn_param1;
#[doc = "Connection parameter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param2](conn_param2) module"]
pub type CONN_PARAM2 = crate::Reg<u32, _CONN_PARAM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_PARAM2;
#[doc = "`read()` method returns [conn_param2::R](conn_param2::R) reader structure"]
impl crate::Readable for CONN_PARAM2 {}
#[doc = "`write(|w| ..)` method takes [conn_param2::W](conn_param2::W) writer structure"]
impl crate::Writable for CONN_PARAM2 {}
#[doc = "Connection parameter 2"]
pub mod conn_param2;
#[doc = "Connection Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_intr_mask](conn_intr_mask) module"]
pub type CONN_INTR_MASK = crate::Reg<u32, _CONN_INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_INTR_MASK;
#[doc = "`read()` method returns [conn_intr_mask::R](conn_intr_mask::R) reader structure"]
impl crate::Readable for CONN_INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [conn_intr_mask::W](conn_intr_mask::W) writer structure"]
impl crate::Writable for CONN_INTR_MASK {}
#[doc = "Connection Interrupt mask"]
pub mod conn_intr_mask;
#[doc = "slave timing control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_timing_control](slave_timing_control) module"]
pub type SLAVE_TIMING_CONTROL = crate::Reg<u32, _SLAVE_TIMING_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLAVE_TIMING_CONTROL;
#[doc = "`read()` method returns [slave_timing_control::R](slave_timing_control::R) reader structure"]
impl crate::Readable for SLAVE_TIMING_CONTROL {}
#[doc = "`write(|w| ..)` method takes [slave_timing_control::W](slave_timing_control::W) writer structure"]
impl crate::Writable for SLAVE_TIMING_CONTROL {}
#[doc = "slave timing control"]
pub mod slave_timing_control;
#[doc = "Receive trigger control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_trig_ctrl](receive_trig_ctrl) module"]
pub type RECEIVE_TRIG_CTRL = crate::Reg<u32, _RECEIVE_TRIG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_TRIG_CTRL;
#[doc = "`read()` method returns [receive_trig_ctrl::R](receive_trig_ctrl::R) reader structure"]
impl crate::Readable for RECEIVE_TRIG_CTRL {}
#[doc = "`write(|w| ..)` method takes [receive_trig_ctrl::W](receive_trig_ctrl::W) writer structure"]
impl crate::Writable for RECEIVE_TRIG_CTRL {}
#[doc = "Receive trigger control"]
pub mod receive_trig_ctrl;
#[doc = "LL debug register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_1](ll_dbg_1) module"]
pub type LL_DBG_1 = crate::Reg<u32, _LL_DBG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_1;
#[doc = "`read()` method returns [ll_dbg_1::R](ll_dbg_1::R) reader structure"]
impl crate::Readable for LL_DBG_1 {}
#[doc = "LL debug register 1"]
pub mod ll_dbg_1;
#[doc = "LL debug register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_2](ll_dbg_2) module"]
pub type LL_DBG_2 = crate::Reg<u32, _LL_DBG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_2;
#[doc = "`read()` method returns [ll_dbg_2::R](ll_dbg_2::R) reader structure"]
impl crate::Readable for LL_DBG_2 {}
#[doc = "LL debug register 2"]
pub mod ll_dbg_2;
#[doc = "LL debug register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_3](ll_dbg_3) module"]
pub type LL_DBG_3 = crate::Reg<u32, _LL_DBG_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_3;
#[doc = "`read()` method returns [ll_dbg_3::R](ll_dbg_3::R) reader structure"]
impl crate::Readable for LL_DBG_3 {}
#[doc = "LL debug register 3"]
pub mod ll_dbg_3;
#[doc = "LL debug register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_4](ll_dbg_4) module"]
pub type LL_DBG_4 = crate::Reg<u32, _LL_DBG_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_4;
#[doc = "`read()` method returns [ll_dbg_4::R](ll_dbg_4::R) reader structure"]
impl crate::Readable for LL_DBG_4 {}
#[doc = "LL debug register 4"]
pub mod ll_dbg_4;
#[doc = "LL debug register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_5](ll_dbg_5) module"]
pub type LL_DBG_5 = crate::Reg<u32, _LL_DBG_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_5;
#[doc = "`read()` method returns [ll_dbg_5::R](ll_dbg_5::R) reader structure"]
impl crate::Readable for LL_DBG_5 {}
#[doc = "LL debug register 5"]
pub mod ll_dbg_5;
#[doc = "LL debug register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_6](ll_dbg_6) module"]
pub type LL_DBG_6 = crate::Reg<u32, _LL_DBG_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_6;
#[doc = "`read()` method returns [ll_dbg_6::R](ll_dbg_6::R) reader structure"]
impl crate::Readable for LL_DBG_6 {}
#[doc = "LL debug register 6"]
pub mod ll_dbg_6;
#[doc = "LL debug register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_7](ll_dbg_7) module"]
pub type LL_DBG_7 = crate::Reg<u32, _LL_DBG_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_7;
#[doc = "`read()` method returns [ll_dbg_7::R](ll_dbg_7::R) reader structure"]
impl crate::Readable for LL_DBG_7 {}
#[doc = "LL debug register 7"]
pub mod ll_dbg_7;
#[doc = "LL debug register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_8](ll_dbg_8) module"]
pub type LL_DBG_8 = crate::Reg<u32, _LL_DBG_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_8;
#[doc = "`read()` method returns [ll_dbg_8::R](ll_dbg_8::R) reader structure"]
impl crate::Readable for LL_DBG_8 {}
#[doc = "LL debug register 8"]
pub mod ll_dbg_8;
#[doc = "LL debug register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_9](ll_dbg_9) module"]
pub type LL_DBG_9 = crate::Reg<u32, _LL_DBG_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_9;
#[doc = "`read()` method returns [ll_dbg_9::R](ll_dbg_9::R) reader structure"]
impl crate::Readable for LL_DBG_9 {}
#[doc = "LL debug register 9"]
pub mod ll_dbg_9;
#[doc = "LL debug register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_10](ll_dbg_10) module"]
pub type LL_DBG_10 = crate::Reg<u32, _LL_DBG_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_DBG_10;
#[doc = "`read()` method returns [ll_dbg_10::R](ll_dbg_10::R) reader structure"]
impl crate::Readable for LL_DBG_10 {}
#[doc = "LL debug register 10"]
pub mod ll_dbg_10;
#[doc = "Lower 16 bit address of the peer device for INIT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_init_l](peer_addr_init_l) module"]
pub type PEER_ADDR_INIT_L = crate::Reg<u32, _PEER_ADDR_INIT_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_INIT_L;
#[doc = "`read()` method returns [peer_addr_init_l::R](peer_addr_init_l::R) reader structure"]
impl crate::Readable for PEER_ADDR_INIT_L {}
#[doc = "`write(|w| ..)` method takes [peer_addr_init_l::W](peer_addr_init_l::W) writer structure"]
impl crate::Writable for PEER_ADDR_INIT_L {}
#[doc = "Lower 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_l;
#[doc = "Middle 16 bit address of the peer device for INIT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_init_m](peer_addr_init_m) module"]
pub type PEER_ADDR_INIT_M = crate::Reg<u32, _PEER_ADDR_INIT_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_INIT_M;
#[doc = "`read()` method returns [peer_addr_init_m::R](peer_addr_init_m::R) reader structure"]
impl crate::Readable for PEER_ADDR_INIT_M {}
#[doc = "`write(|w| ..)` method takes [peer_addr_init_m::W](peer_addr_init_m::W) writer structure"]
impl crate::Writable for PEER_ADDR_INIT_M {}
#[doc = "Middle 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_m;
#[doc = "Higher 16 bit address of the peer device for INIT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_addr_init_h](peer_addr_init_h) module"]
pub type PEER_ADDR_INIT_H = crate::Reg<u32, _PEER_ADDR_INIT_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_ADDR_INIT_H;
#[doc = "`read()` method returns [peer_addr_init_h::R](peer_addr_init_h::R) reader structure"]
impl crate::Readable for PEER_ADDR_INIT_H {}
#[doc = "`write(|w| ..)` method takes [peer_addr_init_h::W](peer_addr_init_h::W) writer structure"]
impl crate::Writable for PEER_ADDR_INIT_H {}
#[doc = "Higher 16 bit address of the peer device for INIT."]
pub mod peer_addr_init_h;
#[doc = "Lower 16 bits of the secondary address of the peer device for ADV_DIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_sec_addr_adv_l](peer_sec_addr_adv_l) module"]
pub type PEER_SEC_ADDR_ADV_L = crate::Reg<u32, _PEER_SEC_ADDR_ADV_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_SEC_ADDR_ADV_L;
#[doc = "`read()` method returns [peer_sec_addr_adv_l::R](peer_sec_addr_adv_l::R) reader structure"]
impl crate::Readable for PEER_SEC_ADDR_ADV_L {}
#[doc = "`write(|w| ..)` method takes [peer_sec_addr_adv_l::W](peer_sec_addr_adv_l::W) writer structure"]
impl crate::Writable for PEER_SEC_ADDR_ADV_L {}
#[doc = "Lower 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_l;
#[doc = "Middle 16 bits of the secondary address of the peer device for ADV_DIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_sec_addr_adv_m](peer_sec_addr_adv_m) module"]
pub type PEER_SEC_ADDR_ADV_M = crate::Reg<u32, _PEER_SEC_ADDR_ADV_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_SEC_ADDR_ADV_M;
#[doc = "`read()` method returns [peer_sec_addr_adv_m::R](peer_sec_addr_adv_m::R) reader structure"]
impl crate::Readable for PEER_SEC_ADDR_ADV_M {}
#[doc = "`write(|w| ..)` method takes [peer_sec_addr_adv_m::W](peer_sec_addr_adv_m::W) writer structure"]
impl crate::Writable for PEER_SEC_ADDR_ADV_M {}
#[doc = "Middle 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_m;
#[doc = "Higher 16 bits of the secondary address of the peer device for ADV_DIR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peer_sec_addr_adv_h](peer_sec_addr_adv_h) module"]
pub type PEER_SEC_ADDR_ADV_H = crate::Reg<u32, _PEER_SEC_ADDR_ADV_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEER_SEC_ADDR_ADV_H;
#[doc = "`read()` method returns [peer_sec_addr_adv_h::R](peer_sec_addr_adv_h::R) reader structure"]
impl crate::Readable for PEER_SEC_ADDR_ADV_H {}
#[doc = "`write(|w| ..)` method takes [peer_sec_addr_adv_h::W](peer_sec_addr_adv_h::W) writer structure"]
impl crate::Writable for PEER_SEC_ADDR_ADV_H {}
#[doc = "Higher 16 bits of the secondary address of the peer device for ADV_DIR."]
pub mod peer_sec_addr_adv_h;
#[doc = "Initiator Window NI timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_timer_ctrl](init_window_timer_ctrl) module"]
pub type INIT_WINDOW_TIMER_CTRL = crate::Reg<u32, _INIT_WINDOW_TIMER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_WINDOW_TIMER_CTRL;
#[doc = "`read()` method returns [init_window_timer_ctrl::R](init_window_timer_ctrl::R) reader structure"]
impl crate::Readable for INIT_WINDOW_TIMER_CTRL {}
#[doc = "`write(|w| ..)` method takes [init_window_timer_ctrl::W](init_window_timer_ctrl::W) writer structure"]
impl crate::Writable for INIT_WINDOW_TIMER_CTRL {}
#[doc = "Initiator Window NI timer control"]
pub mod init_window_timer_ctrl;
#[doc = "Connection extended configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_config_ext](conn_config_ext) module"]
pub type CONN_CONFIG_EXT = crate::Reg<u32, _CONN_CONFIG_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_CONFIG_EXT;
#[doc = "`read()` method returns [conn_config_ext::R](conn_config_ext::R) reader structure"]
impl crate::Readable for CONN_CONFIG_EXT {}
#[doc = "`write(|w| ..)` method takes [conn_config_ext::W](conn_config_ext::W) writer structure"]
impl crate::Writable for CONN_CONFIG_EXT {}
#[doc = "Connection extended configuration register"]
pub mod conn_config_ext;
#[doc = "DPLL & CY Correlator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpll_config](dpll_config) module"]
pub type DPLL_CONFIG = crate::Reg<u32, _DPLL_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLL_CONFIG;
#[doc = "`read()` method returns [dpll_config::R](dpll_config::R) reader structure"]
impl crate::Readable for DPLL_CONFIG {}
#[doc = "`write(|w| ..)` method takes [dpll_config::W](dpll_config::W) writer structure"]
impl crate::Writable for DPLL_CONFIG {}
#[doc = "DPLL & CY Correlator configuration register"]
pub mod dpll_config;
#[doc = "Initiator Window NI instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_ni_val](init_ni_val) module"]
pub type INIT_NI_VAL = crate::Reg<u32, _INIT_NI_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_NI_VAL;
#[doc = "`read()` method returns [init_ni_val::R](init_ni_val::R) reader structure"]
impl crate::Readable for INIT_NI_VAL {}
#[doc = "`write(|w| ..)` method takes [init_ni_val::W](init_ni_val::W) writer structure"]
impl crate::Writable for INIT_NI_VAL {}
#[doc = "Initiator Window NI instant"]
pub mod init_ni_val;
#[doc = "Initiator Window offset captured at conn request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_offset](init_window_offset) module"]
pub type INIT_WINDOW_OFFSET = crate::Reg<u32, _INIT_WINDOW_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_WINDOW_OFFSET;
#[doc = "`read()` method returns [init_window_offset::R](init_window_offset::R) reader structure"]
impl crate::Readable for INIT_WINDOW_OFFSET {}
#[doc = "Initiator Window offset captured at conn request"]
pub mod init_window_offset;
#[doc = "Initiator Window NI anchor point captured at conn request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_ni_anchor_pt](init_window_ni_anchor_pt) module"]
pub type INIT_WINDOW_NI_ANCHOR_PT = crate::Reg<u32, _INIT_WINDOW_NI_ANCHOR_PT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT_WINDOW_NI_ANCHOR_PT;
#[doc = "`read()` method returns [init_window_ni_anchor_pt::R](init_window_ni_anchor_pt::R) reader structure"]
impl crate::Readable for INIT_WINDOW_NI_ANCHOR_PT {}
#[doc = "Initiator Window NI anchor point captured at conn request"]
pub mod init_window_ni_anchor_pt;
#[doc = "Connection update new interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_interval](conn_update_new_interval) module"]
pub type CONN_UPDATE_NEW_INTERVAL = crate::Reg<u32, _CONN_UPDATE_NEW_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_UPDATE_NEW_INTERVAL;
#[doc = "`read()` method returns [conn_update_new_interval::R](conn_update_new_interval::R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [conn_update_new_interval::W](conn_update_new_interval::W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_INTERVAL {}
#[doc = "Connection update new interval"]
pub mod conn_update_new_interval;
#[doc = "Connection update new latency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_latency](conn_update_new_latency) module"]
pub type CONN_UPDATE_NEW_LATENCY = crate::Reg<u32, _CONN_UPDATE_NEW_LATENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_UPDATE_NEW_LATENCY;
#[doc = "`read()` method returns [conn_update_new_latency::R](conn_update_new_latency::R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_LATENCY {}
#[doc = "`write(|w| ..)` method takes [conn_update_new_latency::W](conn_update_new_latency::W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_LATENCY {}
#[doc = "Connection update new latency"]
pub mod conn_update_new_latency;
#[doc = "Connection update new supervision timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_sup_to](conn_update_new_sup_to) module"]
pub type CONN_UPDATE_NEW_SUP_TO = crate::Reg<u32, _CONN_UPDATE_NEW_SUP_TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_UPDATE_NEW_SUP_TO;
#[doc = "`read()` method returns [conn_update_new_sup_to::R](conn_update_new_sup_to::R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_SUP_TO {}
#[doc = "`write(|w| ..)` method takes [conn_update_new_sup_to::W](conn_update_new_sup_to::W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_SUP_TO {}
#[doc = "Connection update new supervision timeout"]
pub mod conn_update_new_sup_to;
#[doc = "Connection update new Slave Latency X Conn interval Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_sl_interval](conn_update_new_sl_interval) module"]
pub type CONN_UPDATE_NEW_SL_INTERVAL = crate::Reg<u32, _CONN_UPDATE_NEW_SL_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_UPDATE_NEW_SL_INTERVAL;
#[doc = "`read()` method returns [conn_update_new_sl_interval::R](conn_update_new_sl_interval::R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_SL_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [conn_update_new_sl_interval::W](conn_update_new_sl_interval::W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_SL_INTERVAL {}
#[doc = "Connection update new Slave Latency X Conn interval Value"]
pub mod conn_update_new_sl_interval;
#[doc = "Connection request address word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word0](conn_req_word0) module"]
pub type CONN_REQ_WORD0 = crate::Reg<u32, _CONN_REQ_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD0;
#[doc = "`read()` method returns [conn_req_word0::R](conn_req_word0::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD0 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word0::W](conn_req_word0::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD0 {}
#[doc = "Connection request address word 0"]
pub mod conn_req_word0;
#[doc = "Connection request address word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word1](conn_req_word1) module"]
pub type CONN_REQ_WORD1 = crate::Reg<u32, _CONN_REQ_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD1;
#[doc = "`read()` method returns [conn_req_word1::R](conn_req_word1::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD1 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word1::W](conn_req_word1::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD1 {}
#[doc = "Connection request address word 1"]
pub mod conn_req_word1;
#[doc = "Connection request address word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word2](conn_req_word2) module"]
pub type CONN_REQ_WORD2 = crate::Reg<u32, _CONN_REQ_WORD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD2;
#[doc = "`read()` method returns [conn_req_word2::R](conn_req_word2::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD2 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word2::W](conn_req_word2::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD2 {}
#[doc = "Connection request address word 2"]
pub mod conn_req_word2;
#[doc = "Connection request address word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word3](conn_req_word3) module"]
pub type CONN_REQ_WORD3 = crate::Reg<u32, _CONN_REQ_WORD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD3;
#[doc = "`read()` method returns [conn_req_word3::R](conn_req_word3::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD3 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word3::W](conn_req_word3::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD3 {}
#[doc = "Connection request address word 3"]
pub mod conn_req_word3;
#[doc = "Connection request address word 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word4](conn_req_word4) module"]
pub type CONN_REQ_WORD4 = crate::Reg<u32, _CONN_REQ_WORD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD4;
#[doc = "`read()` method returns [conn_req_word4::R](conn_req_word4::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD4 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word4::W](conn_req_word4::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD4 {}
#[doc = "Connection request address word 4"]
pub mod conn_req_word4;
#[doc = "Connection request address word 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word5](conn_req_word5) module"]
pub type CONN_REQ_WORD5 = crate::Reg<u32, _CONN_REQ_WORD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD5;
#[doc = "`read()` method returns [conn_req_word5::R](conn_req_word5::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD5 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word5::W](conn_req_word5::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD5 {}
#[doc = "Connection request address word 5"]
pub mod conn_req_word5;
#[doc = "Connection request address word 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word6](conn_req_word6) module"]
pub type CONN_REQ_WORD6 = crate::Reg<u32, _CONN_REQ_WORD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD6;
#[doc = "`read()` method returns [conn_req_word6::R](conn_req_word6::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD6 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word6::W](conn_req_word6::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD6 {}
#[doc = "Connection request address word 6"]
pub mod conn_req_word6;
#[doc = "Connection request address word 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word7](conn_req_word7) module"]
pub type CONN_REQ_WORD7 = crate::Reg<u32, _CONN_REQ_WORD7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD7;
#[doc = "`read()` method returns [conn_req_word7::R](conn_req_word7::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD7 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word7::W](conn_req_word7::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD7 {}
#[doc = "Connection request address word 7"]
pub mod conn_req_word7;
#[doc = "Connection request address word 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word8](conn_req_word8) module"]
pub type CONN_REQ_WORD8 = crate::Reg<u32, _CONN_REQ_WORD8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD8;
#[doc = "`read()` method returns [conn_req_word8::R](conn_req_word8::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD8 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word8::W](conn_req_word8::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD8 {}
#[doc = "Connection request address word 8"]
pub mod conn_req_word8;
#[doc = "Connection request address word 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word9](conn_req_word9) module"]
pub type CONN_REQ_WORD9 = crate::Reg<u32, _CONN_REQ_WORD9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD9;
#[doc = "`read()` method returns [conn_req_word9::R](conn_req_word9::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD9 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word9::W](conn_req_word9::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD9 {}
#[doc = "Connection request address word 9"]
pub mod conn_req_word9;
#[doc = "Connection request address word 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word10](conn_req_word10) module"]
pub type CONN_REQ_WORD10 = crate::Reg<u32, _CONN_REQ_WORD10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD10;
#[doc = "`read()` method returns [conn_req_word10::R](conn_req_word10::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD10 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word10::W](conn_req_word10::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD10 {}
#[doc = "Connection request address word 10"]
pub mod conn_req_word10;
#[doc = "Connection request address word 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word11](conn_req_word11) module"]
pub type CONN_REQ_WORD11 = crate::Reg<u32, _CONN_REQ_WORD11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_REQ_WORD11;
#[doc = "`read()` method returns [conn_req_word11::R](conn_req_word11::R) reader structure"]
impl crate::Readable for CONN_REQ_WORD11 {}
#[doc = "`write(|w| ..)` method takes [conn_req_word11::W](conn_req_word11::W) writer structure"]
impl crate::Writable for CONN_REQ_WORD11 {}
#[doc = "Connection request address word 11"]
pub mod conn_req_word11;
#[doc = "PDU response timer/Generic Timer (MMMS mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdu_resp_timer](pdu_resp_timer) module"]
pub type PDU_RESP_TIMER = crate::Reg<u32, _PDU_RESP_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDU_RESP_TIMER;
#[doc = "`read()` method returns [pdu_resp_timer::R](pdu_resp_timer::R) reader structure"]
impl crate::Readable for PDU_RESP_TIMER {}
#[doc = "`write(|w| ..)` method takes [pdu_resp_timer::W](pdu_resp_timer::W) writer structure"]
impl crate::Writable for PDU_RESP_TIMER {}
#[doc = "PDU response timer/Generic Timer (MMMS mode)"]
pub mod pdu_resp_timer;
#[doc = "Next response timeout instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_resp_timer_exp](next_resp_timer_exp) module"]
pub type NEXT_RESP_TIMER_EXP = crate::Reg<u32, _NEXT_RESP_TIMER_EXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_RESP_TIMER_EXP;
#[doc = "`read()` method returns [next_resp_timer_exp::R](next_resp_timer_exp::R) reader structure"]
impl crate::Readable for NEXT_RESP_TIMER_EXP {}
#[doc = "Next response timeout instant"]
pub mod next_resp_timer_exp;
#[doc = "Next supervision timeout instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_sup_to](next_sup_to) module"]
pub type NEXT_SUP_TO = crate::Reg<u32, _NEXT_SUP_TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_SUP_TO;
#[doc = "`read()` method returns [next_sup_to::R](next_sup_to::R) reader structure"]
impl crate::Readable for NEXT_SUP_TO {}
#[doc = "Next supervision timeout instant"]
pub mod next_sup_to;
#[doc = "Feature enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [llh_feature_config](llh_feature_config) module"]
pub type LLH_FEATURE_CONFIG = crate::Reg<u32, _LLH_FEATURE_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LLH_FEATURE_CONFIG;
#[doc = "`read()` method returns [llh_feature_config::R](llh_feature_config::R) reader structure"]
impl crate::Readable for LLH_FEATURE_CONFIG {}
#[doc = "`write(|w| ..)` method takes [llh_feature_config::W](llh_feature_config::W) writer structure"]
impl crate::Writable for LLH_FEATURE_CONFIG {}
#[doc = "Feature enable"]
pub mod llh_feature_config;
#[doc = "Window minimum step size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win_min_step_size](win_min_step_size) module"]
pub type WIN_MIN_STEP_SIZE = crate::Reg<u32, _WIN_MIN_STEP_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIN_MIN_STEP_SIZE;
#[doc = "`read()` method returns [win_min_step_size::R](win_min_step_size::R) reader structure"]
impl crate::Readable for WIN_MIN_STEP_SIZE {}
#[doc = "`write(|w| ..)` method takes [win_min_step_size::W](win_min_step_size::W) writer structure"]
impl crate::Writable for WIN_MIN_STEP_SIZE {}
#[doc = "Window minimum step size"]
pub mod win_min_step_size;
#[doc = "Slave window adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_win_adj](slv_win_adj) module"]
pub type SLV_WIN_ADJ = crate::Reg<u32, _SLV_WIN_ADJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLV_WIN_ADJ;
#[doc = "`read()` method returns [slv_win_adj::R](slv_win_adj::R) reader structure"]
impl crate::Readable for SLV_WIN_ADJ {}
#[doc = "`write(|w| ..)` method takes [slv_win_adj::W](slv_win_adj::W) writer structure"]
impl crate::Writable for SLV_WIN_ADJ {}
#[doc = "Slave window adjustment"]
pub mod slv_win_adj;
#[doc = "Slave Latency X Conn Interval Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_conn_interval](sl_conn_interval) module"]
pub type SL_CONN_INTERVAL = crate::Reg<u32, _SL_CONN_INTERVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SL_CONN_INTERVAL;
#[doc = "`read()` method returns [sl_conn_interval::R](sl_conn_interval::R) reader structure"]
impl crate::Readable for SL_CONN_INTERVAL {}
#[doc = "`write(|w| ..)` method takes [sl_conn_interval::W](sl_conn_interval::W) writer structure"]
impl crate::Writable for SL_CONN_INTERVAL {}
#[doc = "Slave Latency X Conn Interval Value"]
pub mod sl_conn_interval;
#[doc = "LE Ping connection timer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_addr](le_ping_timer_addr) module"]
pub type LE_PING_TIMER_ADDR = crate::Reg<u32, _LE_PING_TIMER_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_PING_TIMER_ADDR;
#[doc = "`read()` method returns [le_ping_timer_addr::R](le_ping_timer_addr::R) reader structure"]
impl crate::Readable for LE_PING_TIMER_ADDR {}
#[doc = "`write(|w| ..)` method takes [le_ping_timer_addr::W](le_ping_timer_addr::W) writer structure"]
impl crate::Writable for LE_PING_TIMER_ADDR {}
#[doc = "LE Ping connection timer address"]
pub mod le_ping_timer_addr;
#[doc = "LE Ping connection timer offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_offset](le_ping_timer_offset) module"]
pub type LE_PING_TIMER_OFFSET = crate::Reg<u32, _LE_PING_TIMER_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_PING_TIMER_OFFSET;
#[doc = "`read()` method returns [le_ping_timer_offset::R](le_ping_timer_offset::R) reader structure"]
impl crate::Readable for LE_PING_TIMER_OFFSET {}
#[doc = "`write(|w| ..)` method takes [le_ping_timer_offset::W](le_ping_timer_offset::W) writer structure"]
impl crate::Writable for LE_PING_TIMER_OFFSET {}
#[doc = "LE Ping connection timer offset"]
pub mod le_ping_timer_offset;
#[doc = "LE Ping timer next expiry instant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_next_exp](le_ping_timer_next_exp) module"]
pub type LE_PING_TIMER_NEXT_EXP = crate::Reg<u32, _LE_PING_TIMER_NEXT_EXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_PING_TIMER_NEXT_EXP;
#[doc = "`read()` method returns [le_ping_timer_next_exp::R](le_ping_timer_next_exp::R) reader structure"]
impl crate::Readable for LE_PING_TIMER_NEXT_EXP {}
#[doc = "LE Ping timer next expiry instant"]
pub mod le_ping_timer_next_exp;
#[doc = "LE Ping Timer wrap count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_wrap_count](le_ping_timer_wrap_count) module"]
pub type LE_PING_TIMER_WRAP_COUNT = crate::Reg<u32, _LE_PING_TIMER_WRAP_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE_PING_TIMER_WRAP_COUNT;
#[doc = "`read()` method returns [le_ping_timer_wrap_count::R](le_ping_timer_wrap_count::R) reader structure"]
impl crate::Readable for LE_PING_TIMER_WRAP_COUNT {}
#[doc = "LE Ping Timer wrap count"]
pub mod le_ping_timer_wrap_count;
#[doc = "Transmit enable extension delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_en_ext_delay](tx_en_ext_delay) module"]
pub type TX_EN_EXT_DELAY = crate::Reg<u32, _TX_EN_EXT_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_EN_EXT_DELAY;
#[doc = "`read()` method returns [tx_en_ext_delay::R](tx_en_ext_delay::R) reader structure"]
impl crate::Readable for TX_EN_EXT_DELAY {}
#[doc = "`write(|w| ..)` method takes [tx_en_ext_delay::W](tx_en_ext_delay::W) writer structure"]
impl crate::Writable for TX_EN_EXT_DELAY {}
#[doc = "Transmit enable extension delay"]
pub mod tx_en_ext_delay;
#[doc = "Transmit/Receive enable delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rx_synth_delay](tx_rx_synth_delay) module"]
pub type TX_RX_SYNTH_DELAY = crate::Reg<u32, _TX_RX_SYNTH_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_RX_SYNTH_DELAY;
#[doc = "`read()` method returns [tx_rx_synth_delay::R](tx_rx_synth_delay::R) reader structure"]
impl crate::Readable for TX_RX_SYNTH_DELAY {}
#[doc = "`write(|w| ..)` method takes [tx_rx_synth_delay::W](tx_rx_synth_delay::W) writer structure"]
impl crate::Writable for TX_RX_SYNTH_DELAY {}
#[doc = "Transmit/Receive enable delay"]
pub mod tx_rx_synth_delay;
#[doc = "External TX PA and RX LNA delay configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_pa_lna_dly_cnfg](ext_pa_lna_dly_cnfg) module"]
pub type EXT_PA_LNA_DLY_CNFG = crate::Reg<u32, _EXT_PA_LNA_DLY_CNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_PA_LNA_DLY_CNFG;
#[doc = "`read()` method returns [ext_pa_lna_dly_cnfg::R](ext_pa_lna_dly_cnfg::R) reader structure"]
impl crate::Readable for EXT_PA_LNA_DLY_CNFG {}
#[doc = "`write(|w| ..)` method takes [ext_pa_lna_dly_cnfg::W](ext_pa_lna_dly_cnfg::W) writer structure"]
impl crate::Writable for EXT_PA_LNA_DLY_CNFG {}
#[doc = "External TX PA and RX LNA delay configuration"]
pub mod ext_pa_lna_dly_cnfg;
#[doc = "Link Layer additional configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_config](ll_config) module"]
pub type LL_CONFIG = crate::Reg<u32, _LL_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_CONFIG;
#[doc = "`read()` method returns [ll_config::R](ll_config::R) reader structure"]
impl crate::Readable for LL_CONFIG {}
#[doc = "`write(|w| ..)` method takes [ll_config::W](ll_config::W) writer structure"]
impl crate::Writable for LL_CONFIG {}
#[doc = "Link Layer additional configuration"]
pub mod ll_config;
#[doc = "LL Backward compatibility\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_control](ll_control) module"]
pub type LL_CONTROL = crate::Reg<u32, _LL_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LL_CONTROL;
#[doc = "`read()` method returns [ll_control::R](ll_control::R) reader structure"]
impl crate::Readable for LL_CONTROL {}
#[doc = "`write(|w| ..)` method takes [ll_control::W](ll_control::W) writer structure"]
impl crate::Writable for LL_CONTROL {}
#[doc = "LL Backward compatibility"]
pub mod ll_control;
#[doc = "Device Resolvable/Non-Resolvable Private address lower register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pa_addr_l](dev_pa_addr_l) module"]
pub type DEV_PA_ADDR_L = crate::Reg<u32, _DEV_PA_ADDR_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PA_ADDR_L;
#[doc = "`read()` method returns [dev_pa_addr_l::R](dev_pa_addr_l::R) reader structure"]
impl crate::Readable for DEV_PA_ADDR_L {}
#[doc = "`write(|w| ..)` method takes [dev_pa_addr_l::W](dev_pa_addr_l::W) writer structure"]
impl crate::Writable for DEV_PA_ADDR_L {}
#[doc = "Device Resolvable/Non-Resolvable Private address lower register"]
pub mod dev_pa_addr_l;
#[doc = "Device Resolvable/Non-Resolvable Private address middle register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pa_addr_m](dev_pa_addr_m) module"]
pub type DEV_PA_ADDR_M = crate::Reg<u32, _DEV_PA_ADDR_M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PA_ADDR_M;
#[doc = "`read()` method returns [dev_pa_addr_m::R](dev_pa_addr_m::R) reader structure"]
impl crate::Readable for DEV_PA_ADDR_M {}
#[doc = "`write(|w| ..)` method takes [dev_pa_addr_m::W](dev_pa_addr_m::W) writer structure"]
impl crate::Writable for DEV_PA_ADDR_M {}
#[doc = "Device Resolvable/Non-Resolvable Private address middle register"]
pub mod dev_pa_addr_m;
#[doc = "Device Resolvable/Non-Resolvable Private address higher register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pa_addr_h](dev_pa_addr_h) module"]
pub type DEV_PA_ADDR_H = crate::Reg<u32, _DEV_PA_ADDR_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEV_PA_ADDR_H;
#[doc = "`read()` method returns [dev_pa_addr_h::R](dev_pa_addr_h::R) reader structure"]
impl crate::Readable for DEV_PA_ADDR_H {}
#[doc = "`write(|w| ..)` method takes [dev_pa_addr_h::W](dev_pa_addr_h::W) writer structure"]
impl crate::Writable for DEV_PA_ADDR_H {}
#[doc = "Device Resolvable/Non-Resolvable Private address higher register"]
pub mod dev_pa_addr_h;
#[doc = "Resolving list entry control bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_enable](rslv_list_enable) module"]
pub type RSLV_LIST_ENABLE = crate::Reg<u32, _RSLV_LIST_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSLV_LIST_ENABLE;
#[doc = "`read()` method returns [rslv_list_enable::R](rslv_list_enable::R) reader structure"]
impl crate::Readable for RSLV_LIST_ENABLE {}
#[doc = "`write(|w| ..)` method takes [rslv_list_enable::W](rslv_list_enable::W) writer structure"]
impl crate::Writable for RSLV_LIST_ENABLE {}
#[doc = "Resolving list entry control bit"]
pub mod rslv_list_enable;
#[doc = "whitelist valid entry bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_connection_status](wl_connection_status) module"]
pub type WL_CONNECTION_STATUS = crate::Reg<u32, _WL_CONNECTION_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WL_CONNECTION_STATUS;
#[doc = "`read()` method returns [wl_connection_status::R](wl_connection_status::R) reader structure"]
impl crate::Readable for WL_CONNECTION_STATUS {}
#[doc = "`write(|w| ..)` method takes [wl_connection_status::W](wl_connection_status::W) writer structure"]
impl crate::Writable for WL_CONNECTION_STATUS {}
#[doc = "whitelist valid entry bit"]
pub mod wl_connection_status;
#[doc = "DLE Connection RX memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_rxmem_base_addr_dle](conn_rxmem_base_addr_dle) module"]
pub type CONN_RXMEM_BASE_ADDR_DLE = crate::Reg<u32, _CONN_RXMEM_BASE_ADDR_DLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_RXMEM_BASE_ADDR_DLE;
#[doc = "`read()` method returns [conn_rxmem_base_addr_dle::R](conn_rxmem_base_addr_dle::R) reader structure"]
impl crate::Readable for CONN_RXMEM_BASE_ADDR_DLE {}
#[doc = "`write(|w| ..)` method takes [conn_rxmem_base_addr_dle::W](conn_rxmem_base_addr_dle::W) writer structure"]
impl crate::Writable for CONN_RXMEM_BASE_ADDR_DLE {}
#[doc = "DLE Connection RX memory base address"]
pub mod conn_rxmem_base_addr_dle;
#[doc = "DLE Connection TX memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_txmem_base_addr_dle](conn_txmem_base_addr_dle) module"]
pub type CONN_TXMEM_BASE_ADDR_DLE = crate::Reg<u32, _CONN_TXMEM_BASE_ADDR_DLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_TXMEM_BASE_ADDR_DLE;
#[doc = "`read()` method returns [conn_txmem_base_addr_dle::R](conn_txmem_base_addr_dle::R) reader structure"]
impl crate::Readable for CONN_TXMEM_BASE_ADDR_DLE {}
#[doc = "`write(|w| ..)` method takes [conn_txmem_base_addr_dle::W](conn_txmem_base_addr_dle::W) writer structure"]
impl crate::Writable for CONN_TXMEM_BASE_ADDR_DLE {}
#[doc = "DLE Connection TX memory base address"]
pub mod conn_txmem_base_addr_dle;
#[doc = "Connection Parameter memory base address for connection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_1_param_mem_base_addr](conn_1_param_mem_base_addr) module"]
pub type CONN_1_PARAM_MEM_BASE_ADDR = crate::Reg<u32, _CONN_1_PARAM_MEM_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_1_PARAM_MEM_BASE_ADDR;
#[doc = "`read()` method returns [conn_1_param_mem_base_addr::R](conn_1_param_mem_base_addr::R) reader structure"]
impl crate::Readable for CONN_1_PARAM_MEM_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [conn_1_param_mem_base_addr::W](conn_1_param_mem_base_addr::W) writer structure"]
impl crate::Writable for CONN_1_PARAM_MEM_BASE_ADDR {}
#[doc = "Connection Parameter memory base address for connection 1"]
pub mod conn_1_param_mem_base_addr;
#[doc = "Connection Parameter memory base address for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_param_mem_base_addr](conn_2_param_mem_base_addr) module"]
pub type CONN_2_PARAM_MEM_BASE_ADDR = crate::Reg<u32, _CONN_2_PARAM_MEM_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_2_PARAM_MEM_BASE_ADDR;
#[doc = "`read()` method returns [conn_2_param_mem_base_addr::R](conn_2_param_mem_base_addr::R) reader structure"]
impl crate::Readable for CONN_2_PARAM_MEM_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [conn_2_param_mem_base_addr::W](conn_2_param_mem_base_addr::W) writer structure"]
impl crate::Writable for CONN_2_PARAM_MEM_BASE_ADDR {}
#[doc = "Connection Parameter memory base address for connection 2"]
pub mod conn_2_param_mem_base_addr;
#[doc = "Connection Parameter memory base address for connection 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_3_param_mem_base_addr](conn_3_param_mem_base_addr) module"]
pub type CONN_3_PARAM_MEM_BASE_ADDR = crate::Reg<u32, _CONN_3_PARAM_MEM_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_3_PARAM_MEM_BASE_ADDR;
#[doc = "`read()` method returns [conn_3_param_mem_base_addr::R](conn_3_param_mem_base_addr::R) reader structure"]
impl crate::Readable for CONN_3_PARAM_MEM_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [conn_3_param_mem_base_addr::W](conn_3_param_mem_base_addr::W) writer structure"]
impl crate::Writable for CONN_3_PARAM_MEM_BASE_ADDR {}
#[doc = "Connection Parameter memory base address for connection 3"]
pub mod conn_3_param_mem_base_addr;
#[doc = "Connection Parameter memory base address for connection 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_4_param_mem_base_addr](conn_4_param_mem_base_addr) module"]
pub type CONN_4_PARAM_MEM_BASE_ADDR = crate::Reg<u32, _CONN_4_PARAM_MEM_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_4_PARAM_MEM_BASE_ADDR;
#[doc = "`read()` method returns [conn_4_param_mem_base_addr::R](conn_4_param_mem_base_addr::R) reader structure"]
impl crate::Readable for CONN_4_PARAM_MEM_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [conn_4_param_mem_base_addr::W](conn_4_param_mem_base_addr::W) writer structure"]
impl crate::Writable for CONN_4_PARAM_MEM_BASE_ADDR {}
#[doc = "Connection Parameter memory base address for connection 4"]
pub mod conn_4_param_mem_base_addr;
#[doc = "Next Instant Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ni_timer](ni_timer) module"]
pub type NI_TIMER = crate::Reg<u32, _NI_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NI_TIMER;
#[doc = "`read()` method returns [ni_timer::R](ni_timer::R) reader structure"]
impl crate::Readable for NI_TIMER {}
#[doc = "`write(|w| ..)` method takes [ni_timer::W](ni_timer::W) writer structure"]
impl crate::Writable for NI_TIMER {}
#[doc = "Next Instant Timer"]
pub mod ni_timer;
#[doc = "Micro-second Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_offset](us_offset) module"]
pub type US_OFFSET = crate::Reg<u32, _US_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_OFFSET;
#[doc = "`read()` method returns [us_offset::R](us_offset::R) reader structure"]
impl crate::Readable for US_OFFSET {}
#[doc = "`write(|w| ..)` method takes [us_offset::W](us_offset::W) writer structure"]
impl crate::Writable for US_OFFSET {}
#[doc = "Micro-second Offset"]
pub mod us_offset;
#[doc = "Next Connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_conn](next_conn) module"]
pub type NEXT_CONN = crate::Reg<u32, _NEXT_CONN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_CONN;
#[doc = "`read()` method returns [next_conn::R](next_conn::R) reader structure"]
impl crate::Readable for NEXT_CONN {}
#[doc = "`write(|w| ..)` method takes [next_conn::W](next_conn::W) writer structure"]
impl crate::Writable for NEXT_CONN {}
#[doc = "Next Connection"]
pub mod next_conn;
#[doc = "Abort next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ni_abort](ni_abort) module"]
pub type NI_ABORT = crate::Reg<u32, _NI_ABORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NI_ABORT;
#[doc = "`read()` method returns [ni_abort::R](ni_abort::R) reader structure"]
impl crate::Readable for NI_ABORT {}
#[doc = "`write(|w| ..)` method takes [ni_abort::W](ni_abort::W) writer structure"]
impl crate::Writable for NI_ABORT {}
#[doc = "Abort next scheduled connection"]
pub mod ni_abort;
#[doc = "Connection NI Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ni_status](conn_ni_status) module"]
pub type CONN_NI_STATUS = crate::Reg<u32, _CONN_NI_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_NI_STATUS;
#[doc = "`read()` method returns [conn_ni_status::R](conn_ni_status::R) reader structure"]
impl crate::Readable for CONN_NI_STATUS {}
#[doc = "Connection NI Status"]
pub mod conn_ni_status;
#[doc = "Next Supervision timeout Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_sup_to_status](next_sup_to_status) module"]
pub type NEXT_SUP_TO_STATUS = crate::Reg<u32, _NEXT_SUP_TO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NEXT_SUP_TO_STATUS;
#[doc = "`read()` method returns [next_sup_to_status::R](next_sup_to_status::R) reader structure"]
impl crate::Readable for NEXT_SUP_TO_STATUS {}
#[doc = "Next Supervision timeout Status"]
pub mod next_sup_to_status;
#[doc = "Connection Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_conn_status](mmms_conn_status) module"]
pub type MMMS_CONN_STATUS = crate::Reg<u32, _MMMS_CONN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_CONN_STATUS;
#[doc = "`read()` method returns [mmms_conn_status::R](mmms_conn_status::R) reader structure"]
impl crate::Readable for MMMS_CONN_STATUS {}
#[doc = "Connection Status"]
pub mod mmms_conn_status;
#[doc = "BT Slot Captured Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_slot_capt_status](bt_slot_capt_status) module"]
pub type BT_SLOT_CAPT_STATUS = crate::Reg<u32, _BT_SLOT_CAPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT_SLOT_CAPT_STATUS;
#[doc = "`read()` method returns [bt_slot_capt_status::R](bt_slot_capt_status::R) reader structure"]
impl crate::Readable for BT_SLOT_CAPT_STATUS {}
#[doc = "BT Slot Captured Status"]
pub mod bt_slot_capt_status;
#[doc = "Micro-second Capture Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_capt_status](us_capt_status) module"]
pub type US_CAPT_STATUS = crate::Reg<u32, _US_CAPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_CAPT_STATUS;
#[doc = "`read()` method returns [us_capt_status::R](us_capt_status::R) reader structure"]
impl crate::Readable for US_CAPT_STATUS {}
#[doc = "Micro-second Capture Status"]
pub mod us_capt_status;
#[doc = "Micro-second Offset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_offset_status](us_offset_status) module"]
pub type US_OFFSET_STATUS = crate::Reg<u32, _US_OFFSET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_OFFSET_STATUS;
#[doc = "`read()` method returns [us_offset_status::R](us_offset_status::R) reader structure"]
impl crate::Readable for US_OFFSET_STATUS {}
#[doc = "Micro-second Offset Status"]
pub mod us_offset_status;
#[doc = "Accumulated Window Widen Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accu_window_widen_status](accu_window_widen_status) module"]
pub type ACCU_WINDOW_WIDEN_STATUS = crate::Reg<u32, _ACCU_WINDOW_WIDEN_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCU_WINDOW_WIDEN_STATUS;
#[doc = "`read()` method returns [accu_window_widen_status::R](accu_window_widen_status::R) reader structure"]
impl crate::Readable for ACCU_WINDOW_WIDEN_STATUS {}
#[doc = "Accumulated Window Widen Status"]
pub mod accu_window_widen_status;
#[doc = "Status when early interrupt is raised\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [early_intr_status](early_intr_status) module"]
pub type EARLY_INTR_STATUS = crate::Reg<u32, _EARLY_INTR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARLY_INTR_STATUS;
#[doc = "`read()` method returns [early_intr_status::R](early_intr_status::R) reader structure"]
impl crate::Readable for EARLY_INTR_STATUS {}
#[doc = "Status when early interrupt is raised"]
pub mod early_intr_status;
#[doc = "Multi-Master Multi-Slave Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_config](mmms_config) module"]
pub type MMMS_CONFIG = crate::Reg<u32, _MMMS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_CONFIG;
#[doc = "`read()` method returns [mmms_config::R](mmms_config::R) reader structure"]
impl crate::Readable for MMMS_CONFIG {}
#[doc = "`write(|w| ..)` method takes [mmms_config::W](mmms_config::W) writer structure"]
impl crate::Writable for MMMS_CONFIG {}
#[doc = "Multi-Master Multi-Slave Config"]
pub mod mmms_config;
#[doc = "Running US of the current BT Slot\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_counter](us_counter) module"]
pub type US_COUNTER = crate::Reg<u32, _US_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_COUNTER;
#[doc = "`read()` method returns [us_counter::R](us_counter::R) reader structure"]
impl crate::Readable for US_COUNTER {}
#[doc = "Running US of the current BT Slot"]
pub mod us_counter;
#[doc = "Previous captured US of the BT Slot\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_capt_prev](us_capt_prev) module"]
pub type US_CAPT_PREV = crate::Reg<u32, _US_CAPT_PREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_CAPT_PREV;
#[doc = "`read()` method returns [us_capt_prev::R](us_capt_prev::R) reader structure"]
impl crate::Readable for US_CAPT_PREV {}
#[doc = "`write(|w| ..)` method takes [us_capt_prev::W](us_capt_prev::W) writer structure"]
impl crate::Writable for US_CAPT_PREV {}
#[doc = "Previous captured US of the BT Slot"]
pub mod us_capt_prev;
#[doc = "NI at early interrupt\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [early_intr_ni](early_intr_ni) module"]
pub type EARLY_INTR_NI = crate::Reg<u32, _EARLY_INTR_NI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARLY_INTR_NI;
#[doc = "`read()` method returns [early_intr_ni::R](early_intr_ni::R) reader structure"]
impl crate::Readable for EARLY_INTR_NI {}
#[doc = "NI at early interrupt"]
pub mod early_intr_ni;
#[doc = "BT slot capture for master connection creation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_master_create_bt_capt](mmms_master_create_bt_capt) module"]
pub type MMMS_MASTER_CREATE_BT_CAPT = crate::Reg<u32, _MMMS_MASTER_CREATE_BT_CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_MASTER_CREATE_BT_CAPT;
#[doc = "`read()` method returns [mmms_master_create_bt_capt::R](mmms_master_create_bt_capt::R) reader structure"]
impl crate::Readable for MMMS_MASTER_CREATE_BT_CAPT {}
#[doc = "BT slot capture for master connection creation"]
pub mod mmms_master_create_bt_capt;
#[doc = "BT slot capture for slave connection creation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_slave_create_bt_capt](mmms_slave_create_bt_capt) module"]
pub type MMMS_SLAVE_CREATE_BT_CAPT = crate::Reg<u32, _MMMS_SLAVE_CREATE_BT_CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_SLAVE_CREATE_BT_CAPT;
#[doc = "`read()` method returns [mmms_slave_create_bt_capt::R](mmms_slave_create_bt_capt::R) reader structure"]
impl crate::Readable for MMMS_SLAVE_CREATE_BT_CAPT {}
#[doc = "BT slot capture for slave connection creation"]
pub mod mmms_slave_create_bt_capt;
#[doc = "Micro second capture for slave connection creation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_slave_create_us_capt](mmms_slave_create_us_capt) module"]
pub type MMMS_SLAVE_CREATE_US_CAPT = crate::Reg<u32, _MMMS_SLAVE_CREATE_US_CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_SLAVE_CREATE_US_CAPT;
#[doc = "`read()` method returns [mmms_slave_create_us_capt::R](mmms_slave_create_us_capt::R) reader structure"]
impl crate::Readable for MMMS_SLAVE_CREATE_US_CAPT {}
#[doc = "Micro second capture for slave connection creation"]
pub mod mmms_slave_create_us_capt;
#[doc = "Data buffer descriptor 0 to 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_data_mem_descriptor](mmms_data_mem_descriptor) module"]
pub type MMMS_DATA_MEM_DESCRIPTOR = crate::Reg<u32, _MMMS_DATA_MEM_DESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_DATA_MEM_DESCRIPTOR;
#[doc = "`read()` method returns [mmms_data_mem_descriptor::R](mmms_data_mem_descriptor::R) reader structure"]
impl crate::Readable for MMMS_DATA_MEM_DESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [mmms_data_mem_descriptor::W](mmms_data_mem_descriptor::W) writer structure"]
impl crate::Writable for MMMS_DATA_MEM_DESCRIPTOR {}
#[doc = "Data buffer descriptor 0 to 15"]
pub mod mmms_data_mem_descriptor;
#[doc = "data list sent update and status for connection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_1_data_list_sent](conn_1_data_list_sent) module"]
pub type CONN_1_DATA_LIST_SENT = crate::Reg<u32, _CONN_1_DATA_LIST_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_1_DATA_LIST_SENT;
#[doc = "`read()` method returns [conn_1_data_list_sent::R](conn_1_data_list_sent::R) reader structure"]
impl crate::Readable for CONN_1_DATA_LIST_SENT {}
#[doc = "`write(|w| ..)` method takes [conn_1_data_list_sent::W](conn_1_data_list_sent::W) writer structure"]
impl crate::Writable for CONN_1_DATA_LIST_SENT {}
#[doc = "data list sent update and status for connection 1"]
pub mod conn_1_data_list_sent;
#[doc = "data list ack update and status for connection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_1_data_list_ack](conn_1_data_list_ack) module"]
pub type CONN_1_DATA_LIST_ACK = crate::Reg<u32, _CONN_1_DATA_LIST_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_1_DATA_LIST_ACK;
#[doc = "`read()` method returns [conn_1_data_list_ack::R](conn_1_data_list_ack::R) reader structure"]
impl crate::Readable for CONN_1_DATA_LIST_ACK {}
#[doc = "`write(|w| ..)` method takes [conn_1_data_list_ack::W](conn_1_data_list_ack::W) writer structure"]
impl crate::Writable for CONN_1_DATA_LIST_ACK {}
#[doc = "data list ack update and status for connection 1"]
pub mod conn_1_data_list_ack;
#[doc = "Connection specific pause resume for connection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_1_ce_data_list_cfg](conn_1_ce_data_list_cfg) module"]
pub type CONN_1_CE_DATA_LIST_CFG = crate::Reg<u32, _CONN_1_CE_DATA_LIST_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_1_CE_DATA_LIST_CFG;
#[doc = "`read()` method returns [conn_1_ce_data_list_cfg::R](conn_1_ce_data_list_cfg::R) reader structure"]
impl crate::Readable for CONN_1_CE_DATA_LIST_CFG {}
#[doc = "`write(|w| ..)` method takes [conn_1_ce_data_list_cfg::W](conn_1_ce_data_list_cfg::W) writer structure"]
impl crate::Writable for CONN_1_CE_DATA_LIST_CFG {}
#[doc = "Connection specific pause resume for connection 1"]
pub mod conn_1_ce_data_list_cfg;
#[doc = "data list sent update and status for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_data_list_sent](conn_2_data_list_sent) module"]
pub type CONN_2_DATA_LIST_SENT = crate::Reg<u32, _CONN_2_DATA_LIST_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_2_DATA_LIST_SENT;
#[doc = "`read()` method returns [conn_2_data_list_sent::R](conn_2_data_list_sent::R) reader structure"]
impl crate::Readable for CONN_2_DATA_LIST_SENT {}
#[doc = "`write(|w| ..)` method takes [conn_2_data_list_sent::W](conn_2_data_list_sent::W) writer structure"]
impl crate::Writable for CONN_2_DATA_LIST_SENT {}
#[doc = "data list sent update and status for connection 2"]
pub mod conn_2_data_list_sent;
#[doc = "data list ack update and status for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_data_list_ack](conn_2_data_list_ack) module"]
pub type CONN_2_DATA_LIST_ACK = crate::Reg<u32, _CONN_2_DATA_LIST_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_2_DATA_LIST_ACK;
#[doc = "`read()` method returns [conn_2_data_list_ack::R](conn_2_data_list_ack::R) reader structure"]
impl crate::Readable for CONN_2_DATA_LIST_ACK {}
#[doc = "`write(|w| ..)` method takes [conn_2_data_list_ack::W](conn_2_data_list_ack::W) writer structure"]
impl crate::Writable for CONN_2_DATA_LIST_ACK {}
#[doc = "data list ack update and status for connection 2"]
pub mod conn_2_data_list_ack;
#[doc = "Connection specific pause resume for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_ce_data_list_cfg](conn_2_ce_data_list_cfg) module"]
pub type CONN_2_CE_DATA_LIST_CFG = crate::Reg<u32, _CONN_2_CE_DATA_LIST_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_2_CE_DATA_LIST_CFG;
#[doc = "`read()` method returns [conn_2_ce_data_list_cfg::R](conn_2_ce_data_list_cfg::R) reader structure"]
impl crate::Readable for CONN_2_CE_DATA_LIST_CFG {}
#[doc = "`write(|w| ..)` method takes [conn_2_ce_data_list_cfg::W](conn_2_ce_data_list_cfg::W) writer structure"]
impl crate::Writable for CONN_2_CE_DATA_LIST_CFG {}
#[doc = "Connection specific pause resume for connection 2"]
pub mod conn_2_ce_data_list_cfg;
#[doc = "data list sent update and status for connection 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_3_data_list_sent](conn_3_data_list_sent) module"]
pub type CONN_3_DATA_LIST_SENT = crate::Reg<u32, _CONN_3_DATA_LIST_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_3_DATA_LIST_SENT;
#[doc = "`read()` method returns [conn_3_data_list_sent::R](conn_3_data_list_sent::R) reader structure"]
impl crate::Readable for CONN_3_DATA_LIST_SENT {}
#[doc = "`write(|w| ..)` method takes [conn_3_data_list_sent::W](conn_3_data_list_sent::W) writer structure"]
impl crate::Writable for CONN_3_DATA_LIST_SENT {}
#[doc = "data list sent update and status for connection 3"]
pub mod conn_3_data_list_sent;
#[doc = "data list ack update and status for connection 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_3_data_list_ack](conn_3_data_list_ack) module"]
pub type CONN_3_DATA_LIST_ACK = crate::Reg<u32, _CONN_3_DATA_LIST_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_3_DATA_LIST_ACK;
#[doc = "`read()` method returns [conn_3_data_list_ack::R](conn_3_data_list_ack::R) reader structure"]
impl crate::Readable for CONN_3_DATA_LIST_ACK {}
#[doc = "`write(|w| ..)` method takes [conn_3_data_list_ack::W](conn_3_data_list_ack::W) writer structure"]
impl crate::Writable for CONN_3_DATA_LIST_ACK {}
#[doc = "data list ack update and status for connection 3"]
pub mod conn_3_data_list_ack;
#[doc = "Connection specific pause resume for connection 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_3_ce_data_list_cfg](conn_3_ce_data_list_cfg) module"]
pub type CONN_3_CE_DATA_LIST_CFG = crate::Reg<u32, _CONN_3_CE_DATA_LIST_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_3_CE_DATA_LIST_CFG;
#[doc = "`read()` method returns [conn_3_ce_data_list_cfg::R](conn_3_ce_data_list_cfg::R) reader structure"]
impl crate::Readable for CONN_3_CE_DATA_LIST_CFG {}
#[doc = "`write(|w| ..)` method takes [conn_3_ce_data_list_cfg::W](conn_3_ce_data_list_cfg::W) writer structure"]
impl crate::Writable for CONN_3_CE_DATA_LIST_CFG {}
#[doc = "Connection specific pause resume for connection 3"]
pub mod conn_3_ce_data_list_cfg;
#[doc = "data list sent update and status for connection 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_4_data_list_sent](conn_4_data_list_sent) module"]
pub type CONN_4_DATA_LIST_SENT = crate::Reg<u32, _CONN_4_DATA_LIST_SENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_4_DATA_LIST_SENT;
#[doc = "`read()` method returns [conn_4_data_list_sent::R](conn_4_data_list_sent::R) reader structure"]
impl crate::Readable for CONN_4_DATA_LIST_SENT {}
#[doc = "`write(|w| ..)` method takes [conn_4_data_list_sent::W](conn_4_data_list_sent::W) writer structure"]
impl crate::Writable for CONN_4_DATA_LIST_SENT {}
#[doc = "data list sent update and status for connection 4"]
pub mod conn_4_data_list_sent;
#[doc = "data list ack update and status for connection 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_4_data_list_ack](conn_4_data_list_ack) module"]
pub type CONN_4_DATA_LIST_ACK = crate::Reg<u32, _CONN_4_DATA_LIST_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_4_DATA_LIST_ACK;
#[doc = "`read()` method returns [conn_4_data_list_ack::R](conn_4_data_list_ack::R) reader structure"]
impl crate::Readable for CONN_4_DATA_LIST_ACK {}
#[doc = "`write(|w| ..)` method takes [conn_4_data_list_ack::W](conn_4_data_list_ack::W) writer structure"]
impl crate::Writable for CONN_4_DATA_LIST_ACK {}
#[doc = "data list ack update and status for connection 4"]
pub mod conn_4_data_list_ack;
#[doc = "Connection specific pause resume for connection 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_4_ce_data_list_cfg](conn_4_ce_data_list_cfg) module"]
pub type CONN_4_CE_DATA_LIST_CFG = crate::Reg<u32, _CONN_4_CE_DATA_LIST_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_4_CE_DATA_LIST_CFG;
#[doc = "`read()` method returns [conn_4_ce_data_list_cfg::R](conn_4_ce_data_list_cfg::R) reader structure"]
impl crate::Readable for CONN_4_CE_DATA_LIST_CFG {}
#[doc = "`write(|w| ..)` method takes [conn_4_ce_data_list_cfg::W](conn_4_ce_data_list_cfg::W) writer structure"]
impl crate::Writable for CONN_4_CE_DATA_LIST_CFG {}
#[doc = "Connection specific pause resume for connection 4"]
pub mod conn_4_ce_data_list_cfg;
#[doc = "Enable bits for ADV_NI, SCAN_NI and INIT_NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_enable](mmms_advch_ni_enable) module"]
pub type MMMS_ADVCH_NI_ENABLE = crate::Reg<u32, _MMMS_ADVCH_NI_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_ADVCH_NI_ENABLE;
#[doc = "`read()` method returns [mmms_advch_ni_enable::R](mmms_advch_ni_enable::R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_ENABLE {}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_enable::W](mmms_advch_ni_enable::W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_ENABLE {}
#[doc = "Enable bits for ADV_NI, SCAN_NI and INIT_NI"]
pub mod mmms_advch_ni_enable;
#[doc = "Next instant valid for ADV, SCAN, INIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_valid](mmms_advch_ni_valid) module"]
pub type MMMS_ADVCH_NI_VALID = crate::Reg<u32, _MMMS_ADVCH_NI_VALID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_ADVCH_NI_VALID;
#[doc = "`read()` method returns [mmms_advch_ni_valid::R](mmms_advch_ni_valid::R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_VALID {}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_valid::W](mmms_advch_ni_valid::W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_VALID {}
#[doc = "Next instant valid for ADV, SCAN, INIT"]
pub mod mmms_advch_ni_valid;
#[doc = "Abort the next instant of ADV, SCAN, INIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_abort](mmms_advch_ni_abort) module"]
pub type MMMS_ADVCH_NI_ABORT = crate::Reg<u32, _MMMS_ADVCH_NI_ABORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_ADVCH_NI_ABORT;
#[doc = "`read()` method returns [mmms_advch_ni_abort::R](mmms_advch_ni_abort::R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_ABORT {}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_abort::W](mmms_advch_ni_abort::W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_ABORT {}
#[doc = "Abort the next instant of ADV, SCAN, INIT"]
pub mod mmms_advch_ni_abort;
#[doc = "Register to configure the supervision timeout for next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param_next_sup_to](conn_param_next_sup_to) module"]
pub type CONN_PARAM_NEXT_SUP_TO = crate::Reg<u32, _CONN_PARAM_NEXT_SUP_TO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_PARAM_NEXT_SUP_TO;
#[doc = "`read()` method returns [conn_param_next_sup_to::R](conn_param_next_sup_to::R) reader structure"]
impl crate::Readable for CONN_PARAM_NEXT_SUP_TO {}
#[doc = "`write(|w| ..)` method takes [conn_param_next_sup_to::W](conn_param_next_sup_to::W) writer structure"]
impl crate::Writable for CONN_PARAM_NEXT_SUP_TO {}
#[doc = "Register to configure the supervision timeout for next scheduled connection"]
pub mod conn_param_next_sup_to;
#[doc = "Register to configure Accumulated window widening for next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param_acc_win_widen](conn_param_acc_win_widen) module"]
pub type CONN_PARAM_ACC_WIN_WIDEN = crate::Reg<u32, _CONN_PARAM_ACC_WIN_WIDEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_PARAM_ACC_WIN_WIDEN;
#[doc = "`read()` method returns [conn_param_acc_win_widen::R](conn_param_acc_win_widen::R) reader structure"]
impl crate::Readable for CONN_PARAM_ACC_WIN_WIDEN {}
#[doc = "`write(|w| ..)` method takes [conn_param_acc_win_widen::W](conn_param_acc_win_widen::W) writer structure"]
impl crate::Writable for CONN_PARAM_ACC_WIN_WIDEN {}
#[doc = "Register to configure Accumulated window widening for next scheduled connection"]
pub mod conn_param_acc_win_widen;
#[doc = "Register to configure offset from connection anchor point at which connection parameter memory should be read\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_load_offset](hw_load_offset) module"]
pub type HW_LOAD_OFFSET = crate::Reg<u32, _HW_LOAD_OFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HW_LOAD_OFFSET;
#[doc = "`read()` method returns [hw_load_offset::R](hw_load_offset::R) reader structure"]
impl crate::Readable for HW_LOAD_OFFSET {}
#[doc = "`write(|w| ..)` method takes [hw_load_offset::W](hw_load_offset::W) writer structure"]
impl crate::Writable for HW_LOAD_OFFSET {}
#[doc = "Register to configure offset from connection anchor point at which connection parameter memory should be read"]
pub mod hw_load_offset;
#[doc = "Random number generated by Hardware for ADV NI calculation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_rand](adv_rand) module"]
pub type ADV_RAND = crate::Reg<u32, _ADV_RAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADV_RAND;
#[doc = "`read()` method returns [adv_rand::R](adv_rand::R) reader structure"]
impl crate::Readable for ADV_RAND {}
#[doc = "Random number generated by Hardware for ADV NI calculation"]
pub mod adv_rand;
#[doc = "Packet Counter of packets in RX FIFO in MMMS mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_rx_pkt_cntr](mmms_rx_pkt_cntr) module"]
pub type MMMS_RX_PKT_CNTR = crate::Reg<u32, _MMMS_RX_PKT_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMMS_RX_PKT_CNTR;
#[doc = "`read()` method returns [mmms_rx_pkt_cntr::R](mmms_rx_pkt_cntr::R) reader structure"]
impl crate::Readable for MMMS_RX_PKT_CNTR {}
#[doc = "Packet Counter of packets in RX FIFO in MMMS mode"]
pub mod mmms_rx_pkt_cntr;
#[doc = "Packet Counter for Individual connection index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_rx_pkt_cntr](conn_rx_pkt_cntr) module"]
pub type CONN_RX_PKT_CNTR = crate::Reg<u32, _CONN_RX_PKT_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONN_RX_PKT_CNTR;
#[doc = "`read()` method returns [conn_rx_pkt_cntr::R](conn_rx_pkt_cntr::R) reader structure"]
impl crate::Readable for CONN_RX_PKT_CNTR {}
#[doc = "Packet Counter for Individual connection index"]
pub mod conn_rx_pkt_cntr;
#[doc = "Whitelist base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [whitelist_base_addr](whitelist_base_addr) module"]
pub type WHITELIST_BASE_ADDR = crate::Reg<u32, _WHITELIST_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WHITELIST_BASE_ADDR;
#[doc = "`read()` method returns [whitelist_base_addr::R](whitelist_base_addr::R) reader structure"]
impl crate::Readable for WHITELIST_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [whitelist_base_addr::W](whitelist_base_addr::W) writer structure"]
impl crate::Writable for WHITELIST_BASE_ADDR {}
#[doc = "Whitelist base address"]
pub mod whitelist_base_addr;
#[doc = "Resolving list base address for storing Peer Identity address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_peer_idntt_base_addr](rslv_list_peer_idntt_base_addr) module"]
pub type RSLV_LIST_PEER_IDNTT_BASE_ADDR = crate::Reg<u32, _RSLV_LIST_PEER_IDNTT_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSLV_LIST_PEER_IDNTT_BASE_ADDR;
#[doc = "`read()` method returns [rslv_list_peer_idntt_base_addr::R](rslv_list_peer_idntt_base_addr::R) reader structure"]
impl crate::Readable for RSLV_LIST_PEER_IDNTT_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [rslv_list_peer_idntt_base_addr::W](rslv_list_peer_idntt_base_addr::W) writer structure"]
impl crate::Writable for RSLV_LIST_PEER_IDNTT_BASE_ADDR {}
#[doc = "Resolving list base address for storing Peer Identity address"]
pub mod rslv_list_peer_idntt_base_addr;
#[doc = "Resolving list base address for storing resolved Peer RPA address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_peer_rpa_base_addr](rslv_list_peer_rpa_base_addr) module"]
pub type RSLV_LIST_PEER_RPA_BASE_ADDR = crate::Reg<u32, _RSLV_LIST_PEER_RPA_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSLV_LIST_PEER_RPA_BASE_ADDR;
#[doc = "`read()` method returns [rslv_list_peer_rpa_base_addr::R](rslv_list_peer_rpa_base_addr::R) reader structure"]
impl crate::Readable for RSLV_LIST_PEER_RPA_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [rslv_list_peer_rpa_base_addr::W](rslv_list_peer_rpa_base_addr::W) writer structure"]
impl crate::Writable for RSLV_LIST_PEER_RPA_BASE_ADDR {}
#[doc = "Resolving list base address for storing resolved Peer RPA address"]
pub mod rslv_list_peer_rpa_base_addr;
#[doc = "Resolving list base address for storing Resolved received INITA RPA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_rcvd_init_rpa_base_addr](rslv_list_rcvd_init_rpa_base_addr) module"]
pub type RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR = crate::Reg<u32, _RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR;
#[doc = "`read()` method returns [rslv_list_rcvd_init_rpa_base_addr::R](rslv_list_rcvd_init_rpa_base_addr::R) reader structure"]
impl crate::Readable for RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [rslv_list_rcvd_init_rpa_base_addr::W](rslv_list_rcvd_init_rpa_base_addr::W) writer structure"]
impl crate::Writable for RSLV_LIST_RCVD_INIT_RPA_BASE_ADDR {}
#[doc = "Resolving list base address for storing Resolved received INITA RPA"]
pub mod rslv_list_rcvd_init_rpa_base_addr;
#[doc = "Resolving list base address for storing generated TX INITA RPA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rslv_list_tx_init_rpa_base_addr](rslv_list_tx_init_rpa_base_addr) module"]
pub type RSLV_LIST_TX_INIT_RPA_BASE_ADDR = crate::Reg<u32, _RSLV_LIST_TX_INIT_RPA_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSLV_LIST_TX_INIT_RPA_BASE_ADDR;
#[doc = "`read()` method returns [rslv_list_tx_init_rpa_base_addr::R](rslv_list_tx_init_rpa_base_addr::R) reader structure"]
impl crate::Readable for RSLV_LIST_TX_INIT_RPA_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [rslv_list_tx_init_rpa_base_addr::W](rslv_list_tx_init_rpa_base_addr::W) writer structure"]
impl crate::Writable for RSLV_LIST_TX_INIT_RPA_BASE_ADDR {}
#[doc = "Resolving list base address for storing generated TX INITA RPA"]
pub mod rslv_list_tx_init_rpa_base_addr;
