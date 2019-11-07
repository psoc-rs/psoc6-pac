#[doc = "Reader of register SATURATE_INTR_SET"]
pub type R = crate::R<u32, super::SATURATE_INTR_SET>;
#[doc = "Writer for register SATURATE_INTR_SET"]
pub type W = crate::W<u32, super::SATURATE_INTR_SET>;
#[doc = "Register SATURATE_INTR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::SATURATE_INTR_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SATURATE_SET`"]
pub type SATURATE_SET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SATURATE_SET`"]
pub struct SATURATE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SATURATE_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_set(&self) -> SATURATE_SET_R {
        SATURATE_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_set(&mut self) -> SATURATE_SET_W {
        SATURATE_SET_W { w: self }
    }
}
