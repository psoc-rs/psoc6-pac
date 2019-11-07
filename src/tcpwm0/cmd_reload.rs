#[doc = "Reader of register CMD_RELOAD"]
pub type R = crate::R<u32, super::CMD_RELOAD>;
#[doc = "Writer for register CMD_RELOAD"]
pub type W = crate::W<u32, super::CMD_RELOAD>;
#[doc = "Register CMD_RELOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_RELOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_RELOAD`"]
pub type COUNTER_RELOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER_RELOAD`"]
pub struct COUNTER_RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> COUNTER_RELOAD_R {
        COUNTER_RELOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&mut self) -> COUNTER_RELOAD_W {
        COUNTER_RELOAD_W { w: self }
    }
}
