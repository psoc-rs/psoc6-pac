#[doc = "Reader of register PDU_RESP_TIMER"]
pub type R = crate::R<u32, super::PDU_RESP_TIMER>;
#[doc = "Writer for register PDU_RESP_TIMER"]
pub type W = crate::W<u32, super::PDU_RESP_TIMER>;
#[doc = "Register PDU_RESP_TIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::PDU_RESP_TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDU_RESP_TIME_VAL`"]
pub type PDU_RESP_TIME_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PDU_RESP_TIME_VAL`"]
pub struct PDU_RESP_TIME_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDU_RESP_TIME_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Non MMMS mode: This register is loaded with the count value to monitor the time to get a response for a PDU from peer device. Firmware starts the timer by issuing the command, RESP_TIMER_ON, after it has queued a PDU for transmission, that requires a response. If a response is received, firmware stops and clears the timer by issuing the command RESP_TIMER_OFF. If this timer expires, it results in hardware closing the connection and triggering a conn_closed interrupt. The discon_status field in the Connection status register is set with the appropriate reason. Units : Milliseconds. Resolution : 1.25 ms MMMS mode: This register is loaded with a count value, which when matched by the internal timer, triggers the GEN_TIMER_INTR. This is recommended to be used as a one shot timer and not as a periodic timer. Firmware starts the timer by loading the expiry time and issuing the command, RESP_TIMER_ON. Once the timer expiry is triggered with the interrupt GEN_TIMER_INTR, the firmware stops the timer by issuing the command RESP_TIMER_OFF. Resolution : 625 us"]
    #[inline(always)]
    pub fn pdu_resp_time_val(&self) -> PDU_RESP_TIME_VAL_R {
        PDU_RESP_TIME_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non MMMS mode: This register is loaded with the count value to monitor the time to get a response for a PDU from peer device. Firmware starts the timer by issuing the command, RESP_TIMER_ON, after it has queued a PDU for transmission, that requires a response. If a response is received, firmware stops and clears the timer by issuing the command RESP_TIMER_OFF. If this timer expires, it results in hardware closing the connection and triggering a conn_closed interrupt. The discon_status field in the Connection status register is set with the appropriate reason. Units : Milliseconds. Resolution : 1.25 ms MMMS mode: This register is loaded with a count value, which when matched by the internal timer, triggers the GEN_TIMER_INTR. This is recommended to be used as a one shot timer and not as a periodic timer. Firmware starts the timer by loading the expiry time and issuing the command, RESP_TIMER_ON. Once the timer expiry is triggered with the interrupt GEN_TIMER_INTR, the firmware stops the timer by issuing the command RESP_TIMER_OFF. Resolution : 625 us"]
    #[inline(always)]
    pub fn pdu_resp_time_val(&mut self) -> PDU_RESP_TIME_VAL_W {
        PDU_RESP_TIME_VAL_W { w: self }
    }
}
