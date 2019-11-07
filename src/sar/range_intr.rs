#[doc = "Reader of register RANGE_INTR"]
pub type R = crate::R<u32, super::RANGE_INTR>;
#[doc = "Writer for register RANGE_INTR"]
pub type W = crate::W<u32, super::RANGE_INTR>;
#[doc = "Register RANGE_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RANGE_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RANGE_INTR`"]
pub type RANGE_INTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RANGE_INTR`"]
pub struct RANGE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_INTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&self) -> RANGE_INTR_R {
        RANGE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&mut self) -> RANGE_INTR_W {
        RANGE_INTR_W { w: self }
    }
}
