#[doc = "Reader of register HOST_RTIMER"]
pub type R = crate::R<u32, super::HOST_RTIMER>;
#[doc = "Writer for register HOST_RTIMER"]
pub type W = crate::W<u32, super::HOST_RTIMER>;
#[doc = "Register HOST_RTIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_RTIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTIMER`"]
pub type RTIMER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTIMER`"]
pub struct RTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&self) -> RTIMER_R {
        RTIMER_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&mut self) -> RTIMER_W {
        RTIMER_W { w: self }
    }
}
