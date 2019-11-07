#[doc = "Reader of register CONN_REQ_WORD4"]
pub type R = crate::R<u32, super::CONN_REQ_WORD4>;
#[doc = "Writer for register CONN_REQ_WORD4"]
pub type W = crate::W<u32, super::CONN_REQ_WORD4>;
#[doc = "Register CONN_REQ_WORD4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_WINDOW_OFFSET`"]
pub type TX_WINDOW_OFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_WINDOW_OFFSET`"]
pub struct TX_WINDOW_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WINDOW_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This is used to determine the anchor point for the master transmission. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
    #[inline(always)]
    pub fn tx_window_offset(&self) -> TX_WINDOW_OFFSET_R {
        TX_WINDOW_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is used to determine the anchor point for the master transmission. Range: This shall be a multiple of 1.25 ms in the range of 0 ms to connInterval value."]
    #[inline(always)]
    pub fn tx_window_offset(&mut self) -> TX_WINDOW_OFFSET_W {
        TX_WINDOW_OFFSET_W { w: self }
    }
}
