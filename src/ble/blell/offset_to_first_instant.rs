#[doc = "Reader of register OFFSET_TO_FIRST_INSTANT"]
pub type R = crate::R<u32, super::OFFSET_TO_FIRST_INSTANT>;
#[doc = "Writer for register OFFSET_TO_FIRST_INSTANT"]
pub type W = crate::W<u32, super::OFFSET_TO_FIRST_INSTANT>;
#[doc = "Register OFFSET_TO_FIRST_INSTANT `reset()`'s with value 0x06"]
impl crate::ResetValue for super::OFFSET_TO_FIRST_INSTANT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `OFFSET_TO_FIRST_EVENT`"]
pub type OFFSET_TO_FIRST_EVENT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSET_TO_FIRST_EVENT`"]
pub struct OFFSET_TO_FIRST_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_TO_FIRST_EVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
    #[inline(always)]
    pub fn offset_to_first_event(&self) -> OFFSET_TO_FIRST_EVENT_R {
        OFFSET_TO_FIRST_EVENT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The offset w.r.t the internal reference clock at which instant the first event occurs. This register will give flexibility to the firmware to position the con-nection at a desired point with respect to the internal free running clock. It is optional to be updated by firmware. This is not updated in the current firmware."]
    #[inline(always)]
    pub fn offset_to_first_event(&mut self) -> OFFSET_TO_FIRST_EVENT_W {
        OFFSET_TO_FIRST_EVENT_W { w: self }
    }
}
