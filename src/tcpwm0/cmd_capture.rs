#[doc = "Reader of register CMD_CAPTURE"]
pub type R = crate::R<u32, super::CMD_CAPTURE>;
#[doc = "Writer for register CMD_CAPTURE"]
pub type W = crate::W<u32, super::CMD_CAPTURE>;
#[doc = "Register CMD_CAPTURE `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_CAPTURE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_CAPTURE`"]
pub type COUNTER_CAPTURE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER_CAPTURE`"]
pub struct COUNTER_CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_CAPTURE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> COUNTER_CAPTURE_R {
        COUNTER_CAPTURE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&mut self) -> COUNTER_CAPTURE_W {
        COUNTER_CAPTURE_W { w: self }
    }
}
