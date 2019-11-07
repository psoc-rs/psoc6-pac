#[doc = "Reader of register CMD_START"]
pub type R = crate::R<u32, super::CMD_START>;
#[doc = "Writer for register CMD_START"]
pub type W = crate::W<u32, super::CMD_START>;
#[doc = "Register CMD_START `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_START`"]
pub type COUNTER_START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER_START`"]
pub struct COUNTER_START_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&self) -> COUNTER_START_R {
        COUNTER_START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&mut self) -> COUNTER_START_W {
        COUNTER_START_W { w: self }
    }
}
