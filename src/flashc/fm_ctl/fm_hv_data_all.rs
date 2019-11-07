#[doc = "Writer for register FM_HV_DATA_ALL"]
pub type W = crate::W<u32, super::FM_HV_DATA_ALL>;
#[doc = "Register FM_HV_DATA_ALL `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_HV_DATA_ALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA32`"]
pub struct DATA32_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle"]
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W {
        DATA32_W { w: self }
    }
}
