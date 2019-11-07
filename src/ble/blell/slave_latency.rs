#[doc = "Reader of register SLAVE_LATENCY"]
pub type R = crate::R<u32, super::SLAVE_LATENCY>;
#[doc = "Writer for register SLAVE_LATENCY"]
pub type W = crate::W<u32, super::SLAVE_LATENCY>;
#[doc = "Register SLAVE_LATENCY `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE_LATENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLAVE_LATENCY`"]
pub type SLAVE_LATENCY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLAVE_LATENCY`"]
pub struct SLAVE_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency(&self) -> SLAVE_LATENCY_R {
        SLAVE_LATENCY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency(&mut self) -> SLAVE_LATENCY_W {
        SLAVE_LATENCY_W { w: self }
    }
}
