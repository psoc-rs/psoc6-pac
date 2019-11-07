#[doc = "Reader of register CTRL_SET"]
pub type R = crate::R<u32, super::CTRL_SET>;
#[doc = "Writer for register CTRL_SET"]
pub type W = crate::W<u32, super::CTRL_SET>;
#[doc = "Register CTRL_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER_ENABLED`"]
pub type COUNTER_ENABLED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNTER_ENABLED`"]
pub struct COUNTER_ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_ENABLED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> COUNTER_ENABLED_R {
        COUNTER_ENABLED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&mut self) -> COUNTER_ENABLED_W {
        COUNTER_ENABLED_W { w: self }
    }
}
