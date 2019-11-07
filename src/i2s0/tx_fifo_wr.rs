#[doc = "Writer for register TX_FIFO_WR"]
pub type W = crate::W<u32, super::TX_FIFO_WR>;
#[doc = "Register TX_FIFO_WR `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_FIFO_WR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data written into the TX FIFO. Behavior is similar to that of a PUSH operation. Note: Don't access to this register while TX_FIFO_CTL.CLEAR is '1'."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
