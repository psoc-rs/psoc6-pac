#[doc = "Reader of register WDT_CNT"]
pub type R = crate::R<u32, super::WDT_CNT>;
#[doc = "Writer for register WDT_CNT"]
pub type W = crate::W<u32, super::WDT_CNT>;
#[doc = "Register WDT_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::WDT_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNTER`"]
pub type COUNTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNTER`"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
}
