#[doc = "Reader of register RANGE_INTR_MASK"]
pub type R = crate::R<u32, super::RANGE_INTR_MASK>;
#[doc = "Writer for register RANGE_INTR_MASK"]
pub type W = crate::W<u32, super::RANGE_INTR_MASK>;
#[doc = "Register RANGE_INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::RANGE_INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RANGE_MASK`"]
pub type RANGE_MASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RANGE_MASK`"]
pub struct RANGE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_MASK_W<'a> {
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
    pub fn range_mask(&self) -> RANGE_MASK_R {
        RANGE_MASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_mask(&mut self) -> RANGE_MASK_W {
        RANGE_MASK_W { w: self }
    }
}
