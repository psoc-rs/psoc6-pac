#[doc = "Reader of register HOST_EP1_RW1_DR"]
pub type R = crate::R<u32, super::HOST_EP1_RW1_DR>;
#[doc = "Writer for register HOST_EP1_RW1_DR"]
pub type W = crate::W<u32, super::HOST_EP1_RW1_DR>;
#[doc = "Register HOST_EP1_RW1_DR `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_EP1_RW1_DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BFDT8`"]
pub type BFDT8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BFDT8`"]
pub struct BFDT8_W<'a> {
    w: &'a mut W,
}
impl<'a> BFDT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Register for EP1. The 1-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt8(&self) -> BFDT8_R {
        BFDT8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP1. The 1-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt8(&mut self) -> BFDT8_W {
        BFDT8_W { w: self }
    }
}
