#[doc = "Reader of register NI_TIMER"]
pub type R = crate::R<u32, super::NI_TIMER>;
#[doc = "Writer for register NI_TIMER"]
pub type W = crate::W<u32, super::NI_TIMER>;
#[doc = "Register NI_TIMER `reset()`'s with value 0"]
impl crate::ResetValue for super::NI_TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NI_TIMER`"]
pub type NI_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NI_TIMER`"]
pub struct NI_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> NI_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
    #[inline(always)]
    pub fn ni_timer(&self) -> NI_TIMER_R {
        NI_TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
    #[inline(always)]
    pub fn ni_timer(&mut self) -> NI_TIMER_W {
        NI_TIMER_W { w: self }
    }
}
