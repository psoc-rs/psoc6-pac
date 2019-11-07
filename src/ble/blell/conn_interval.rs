#[doc = "Reader of register CONN_INTERVAL"]
pub type R = crate::R<u32, super::CONN_INTERVAL>;
#[doc = "Writer for register CONN_INTERVAL"]
pub type W = crate::W<u32, super::CONN_INTERVAL>;
#[doc = "Register CONN_INTERVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONNECTION_INTERVAL`"]
pub type CONNECTION_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONNECTION_INTERVAL`"]
pub struct CONNECTION_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTION_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The value configured in this register determines the spacing be-tween the connection events. This shall be a multiple of 1.25 ms in the range of 7.5 ms to 4.0 s."]
    #[inline(always)]
    pub fn connection_interval(&self) -> CONNECTION_INTERVAL_R {
        CONNECTION_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value configured in this register determines the spacing be-tween the connection events. This shall be a multiple of 1.25 ms in the range of 7.5 ms to 4.0 s."]
    #[inline(always)]
    pub fn connection_interval(&mut self) -> CONNECTION_INTERVAL_W {
        CONNECTION_INTERVAL_W { w: self }
    }
}
