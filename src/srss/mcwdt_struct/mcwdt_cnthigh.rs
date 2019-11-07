#[doc = "Reader of register MCWDT_CNTHIGH"]
pub type R = crate::R<u32, super::MCWDT_CNTHIGH>;
#[doc = "Writer for register MCWDT_CNTHIGH"]
pub type W = crate::W<u32, super::MCWDT_CNTHIGH>;
#[doc = "Register MCWDT_CNTHIGH `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_CNTHIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_CTR2`"]
pub type WDT_CTR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDT_CTR2`"]
pub struct WDT_CTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&self) -> WDT_CTR2_R {
        WDT_CTR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&mut self) -> WDT_CTR2_W {
        WDT_CTR2_W { w: self }
    }
}
