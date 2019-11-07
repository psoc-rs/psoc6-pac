#[doc = "Reader of register HOST_EP2_RW2_DR"]
pub type R = crate::R<u32, super::HOST_EP2_RW2_DR>;
#[doc = "Writer for register HOST_EP2_RW2_DR"]
pub type W = crate::W<u32, super::HOST_EP2_RW2_DR>;
#[doc = "Register HOST_EP2_RW2_DR `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_EP2_RW2_DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BFDT16`"]
pub type BFDT16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BFDT16`"]
pub struct BFDT16_W<'a> {
    w: &'a mut W,
}
impl<'a> BFDT16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Register for EP2. The 2-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt16(&self) -> BFDT16_R {
        BFDT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP2. The 2-Byte data is valid."]
    #[inline(always)]
    pub fn bfdt16(&mut self) -> BFDT16_W {
        BFDT16_W { w: self }
    }
}
