#[doc = "Reader of register INTR_MASK"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register INTR_MASK"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_OVFLW`"]
pub type CNT_OVFLW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CNT_OVFLW`"]
pub struct CNT_OVFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_OVFLW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CNT_OVFLW_R {
        CNT_OVFLW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn cnt_ovflw(&mut self) -> CNT_OVFLW_W {
        CNT_OVFLW_W { w: self }
    }
}
