#[doc = "Register `PDU_RESP_TIMER` reader"]
pub struct R(crate::R<PDU_RESP_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDU_RESP_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDU_RESP_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDU_RESP_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDU_RESP_TIMER` writer"]
pub struct W(crate::W<PDU_RESP_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDU_RESP_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDU_RESP_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDU_RESP_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDU_RESP_TIME_VAL` reader - Non MMMS mode: This register is loaded with the count value to monitor the time to get a response for a PDU from peer device. Firmware starts the timer by issuing the command, RESP_TIMER_ON, after it has queued a PDU for transmission, that requires a response. If a response is received, firmware stops and clears the timer by issuing the command RESP_TIMER_OFF. If this timer expires, it results in hardware closing the connection and triggering a conn_closed interrupt. The discon_status field in the Connection status register is set with the appropriate reason. Units : Milliseconds. Resolution : 1.25 ms MMMS mode: This register is loaded with a count value, which when matched by the internal timer, triggers the GEN_TIMER_INTR. This is recommended to be used as a one shot timer and not as a periodic timer. Firmware starts the timer by loading the expiry time and issuing the command, RESP_TIMER_ON. Once the timer expiry is triggered with the interrupt GEN_TIMER_INTR, the firmware stops the timer by issuing the command RESP_TIMER_OFF. Resolution : 625 us"]
pub type PDU_RESP_TIME_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PDU_RESP_TIME_VAL` writer - Non MMMS mode: This register is loaded with the count value to monitor the time to get a response for a PDU from peer device. Firmware starts the timer by issuing the command, RESP_TIMER_ON, after it has queued a PDU for transmission, that requires a response. If a response is received, firmware stops and clears the timer by issuing the command RESP_TIMER_OFF. If this timer expires, it results in hardware closing the connection and triggering a conn_closed interrupt. The discon_status field in the Connection status register is set with the appropriate reason. Units : Milliseconds. Resolution : 1.25 ms MMMS mode: This register is loaded with a count value, which when matched by the internal timer, triggers the GEN_TIMER_INTR. This is recommended to be used as a one shot timer and not as a periodic timer. Firmware starts the timer by loading the expiry time and issuing the command, RESP_TIMER_ON. Once the timer expiry is triggered with the interrupt GEN_TIMER_INTR, the firmware stops the timer by issuing the command RESP_TIMER_OFF. Resolution : 625 us"]
pub type PDU_RESP_TIME_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDU_RESP_TIMER_SPEC, u16, u16, 16, O>;
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
    pub fn pdu_resp_time_val(&mut self) -> PDU_RESP_TIME_VAL_W<0> {
        PDU_RESP_TIME_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDU response timer/Generic Timer (MMMS mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdu_resp_timer](index.html) module"]
pub struct PDU_RESP_TIMER_SPEC;
impl crate::RegisterSpec for PDU_RESP_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdu_resp_timer::R](R) reader structure"]
impl crate::Readable for PDU_RESP_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdu_resp_timer::W](W) writer structure"]
impl crate::Writable for PDU_RESP_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDU_RESP_TIMER to value 0"]
impl crate::Resettable for PDU_RESP_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
