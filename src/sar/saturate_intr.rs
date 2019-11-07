#[doc = "Reader of register SATURATE_INTR"]
pub type R = crate::R<u32, super::SATURATE_INTR>;
#[doc = "Writer for register SATURATE_INTR"]
pub type W = crate::W<u32, super::SATURATE_INTR>;
#[doc = "Register SATURATE_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SATURATE_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SATURATE_INTR`"]
pub type SATURATE_INTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SATURATE_INTR`"]
pub struct SATURATE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SATURATE_INTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&self) -> SATURATE_INTR_R {
        SATURATE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&mut self) -> SATURATE_INTR_W {
        SATURATE_INTR_W { w: self }
    }
}
