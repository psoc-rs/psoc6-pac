#[doc = "Register `CONN_CE_COUNTER` reader"]
pub struct R(crate::R<CONN_CE_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CE_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CE_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CE_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONNECTION_EVENT_COUNTER` reader - This is the free running counter, connEventCounter as defined by Bluetooth spec. Firmware will read the instantaneous Event counter from this register, during connection update and channel map update procedure. Firmware will use this value to calculate the instant from which the new parameters (for connection update and channel map update) will be effective."]
pub type CONNECTION_EVENT_COUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This is the free running counter, connEventCounter as defined by Bluetooth spec. Firmware will read the instantaneous Event counter from this register, during connection update and channel map update procedure. Firmware will use this value to calculate the instant from which the new parameters (for connection update and channel map update) will be effective."]
    #[inline(always)]
    pub fn connection_event_counter(&self) -> CONNECTION_EVENT_COUNTER_R {
        CONNECTION_EVENT_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "connection event counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ce_counter](index.html) module"]
pub struct CONN_CE_COUNTER_SPEC;
impl crate::RegisterSpec for CONN_CE_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ce_counter::R](R) reader structure"]
impl crate::Readable for CONN_CE_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONN_CE_COUNTER to value 0"]
impl crate::Resettable for CONN_CE_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
