#[doc = "Reader of register COUNTER"]
pub type R = crate::R<u32, super::COUNTER>;
#[doc = "Writer for register COUNTER"]
pub type W = crate::W<u32, super::COUNTER>;
#[doc = "Register COUNTER `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER`"]
pub type COUNTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER`"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
}
