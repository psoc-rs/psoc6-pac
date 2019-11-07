#[doc = "Reader of register CMD_STOP"]
pub type R = crate::R<u32, super::CMD_STOP>;
#[doc = "Writer for register CMD_STOP"]
pub type W = crate::W<u32, super::CMD_STOP>;
#[doc = "Register CMD_STOP `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_STOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_STOP`"]
pub type COUNTER_STOP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER_STOP`"]
pub struct COUNTER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&self) -> COUNTER_STOP_R {
        COUNTER_STOP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&mut self) -> COUNTER_STOP_W {
        COUNTER_STOP_W { w: self }
    }
}
