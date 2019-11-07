#[doc = "Reader of register LE_PING_TIMER_OFFSET"]
pub type R = crate::R<u32, super::LE_PING_TIMER_OFFSET>;
#[doc = "Writer for register LE_PING_TIMER_OFFSET"]
pub type W = crate::W<u32, super::LE_PING_TIMER_OFFSET>;
#[doc = "Register LE_PING_TIMER_OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::LE_PING_TIMER_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_PING_TIMER_OFFSET`"]
pub type CONN_PING_TIMER_OFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_PING_TIMER_OFFSET`"]
pub struct CONN_PING_TIMER_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_PING_TIMER_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The value of ping timer nearly expired offset in the order of 10ms, valid range 0x0 ~ 0xFFFF. This is the time period after which the ping timer nearly expired interrupt is generated."]
    #[inline(always)]
    pub fn conn_ping_timer_offset(&self) -> CONN_PING_TIMER_OFFSET_R {
        CONN_PING_TIMER_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of ping timer nearly expired offset in the order of 10ms, valid range 0x0 ~ 0xFFFF. This is the time period after which the ping timer nearly expired interrupt is generated."]
    #[inline(always)]
    pub fn conn_ping_timer_offset(&mut self) -> CONN_PING_TIMER_OFFSET_W {
        CONN_PING_TIMER_OFFSET_W { w: self }
    }
}
