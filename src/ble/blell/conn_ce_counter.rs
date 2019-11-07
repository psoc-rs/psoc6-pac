#[doc = "Reader of register CONN_CE_COUNTER"]
pub type R = crate::R<u32, super::CONN_CE_COUNTER>;
#[doc = "Reader of field `CONNECTION_EVENT_COUNTER`"]
pub type CONNECTION_EVENT_COUNTER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This is the free running counter, connEventCounter as defined by Bluetooth spec. Firmware will read the instantaneous Event counter from this register, during connection update and channel map update procedure. Firmware will use this value to calculate the instant from which the new parameters (for connection update and channel map update) will be effective."]
    #[inline(always)]
    pub fn connection_event_counter(&self) -> CONNECTION_EVENT_COUNTER_R {
        CONNECTION_EVENT_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
