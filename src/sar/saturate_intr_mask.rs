#[doc = "Reader of register SATURATE_INTR_MASK"]
pub type R = crate::R<u32, super::SATURATE_INTR_MASK>;
#[doc = "Writer for register SATURATE_INTR_MASK"]
pub type W = crate::W<u32, super::SATURATE_INTR_MASK>;
#[doc = "Register SATURATE_INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::SATURATE_INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SATURATE_MASK`"]
pub type SATURATE_MASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SATURATE_MASK`"]
pub struct SATURATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SATURATE_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_mask(&self) -> SATURATE_MASK_R {
        SATURATE_MASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_mask(&mut self) -> SATURATE_MASK_W {
        SATURATE_MASK_W { w: self }
    }
}
