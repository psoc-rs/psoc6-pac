#[doc = "Reader of register CONN_REQ_WORD5"]
pub type R = crate::R<u32, super::CONN_REQ_WORD5>;
#[doc = "Writer for register CONN_REQ_WORD5"]
pub type W = crate::W<u32, super::CONN_REQ_WORD5>;
#[doc = "Register CONN_REQ_WORD5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONNECTION_INTERVAL_VAL`"]
pub type CONNECTION_INTERVAL_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONNECTION_INTERVAL_VAL`"]
pub struct CONNECTION_INTERVAL_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTION_INTERVAL_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The value configured in this register determines the spacing between the connection events. This shall be a multiple of 1.25 ms in the range of 7.5 ms to 4.0 s."]
    #[inline(always)]
    pub fn connection_interval_val(&self) -> CONNECTION_INTERVAL_VAL_R {
        CONNECTION_INTERVAL_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value configured in this register determines the spacing between the connection events. This shall be a multiple of 1.25 ms in the range of 7.5 ms to 4.0 s."]
    #[inline(always)]
    pub fn connection_interval_val(&mut self) -> CONNECTION_INTERVAL_VAL_W {
        CONNECTION_INTERVAL_VAL_W { w: self }
    }
}
