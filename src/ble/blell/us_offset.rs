#[doc = "Reader of register US_OFFSET"]
pub type R = crate::R<u32, super::US_OFFSET>;
#[doc = "Writer for register US_OFFSET"]
pub type W = crate::W<u32, super::US_OFFSET>;
#[doc = "Register US_OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::US_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `US_OFFSET_SLOT_BOUNDARY`"]
pub type US_OFFSET_SLOT_BOUNDARY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `US_OFFSET_SLOT_BOUNDARY`"]
pub struct US_OFFSET_SLOT_BOUNDARY_W<'a> {
    w: &'a mut W,
}
impl<'a> US_OFFSET_SLOT_BOUNDARY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
    #[inline(always)]
    pub fn us_offset_slot_boundary(&self) -> US_OFFSET_SLOT_BOUNDARY_R {
        US_OFFSET_SLOT_BOUNDARY_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Micro Second Offset from the Slot Bounday at which the connection programmed in NEXT_CONN has to be serviced. This register along with NI_TIMER has to be programmed 1.25ms before the connection event. The granularity is 1us"]
    #[inline(always)]
    pub fn us_offset_slot_boundary(&mut self) -> US_OFFSET_SLOT_BOUNDARY_W {
        US_OFFSET_SLOT_BOUNDARY_W { w: self }
    }
}
