#[doc = "Reader of register FM_HV_DATA[%s]"]
pub type R = crate::R<u32, super::FM_HV_DATA>;
#[doc = "Writer for register FM_HV_DATA[%s]"]
pub type W = crate::W<u32, super::FM_HV_DATA>;
#[doc = "Register FM_HV_DATA[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_HV_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA32`"]
pub type DATA32_R = crate::R<u32, u32>;
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
impl R {
    #[doc = "Bits 0:31 - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four page latch Bytes (when writing to the page latches, it also requires FM_CTL.IF_SEL to be '1'). Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&mut self) -> DATA32_W {
        DATA32_W { w: self }
    }
}
