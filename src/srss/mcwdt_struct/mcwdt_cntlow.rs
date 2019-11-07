#[doc = "Reader of register MCWDT_CNTLOW"]
pub type R = crate::R<u32, super::MCWDT_CNTLOW>;
#[doc = "Writer for register MCWDT_CNTLOW"]
pub type W = crate::W<u32, super::MCWDT_CNTLOW>;
#[doc = "Register MCWDT_CNTLOW `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_CNTLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_CTR0`"]
pub type WDT_CTR0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDT_CTR0`"]
pub struct WDT_CTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WDT_CTR1`"]
pub type WDT_CTR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDT_CTR1`"]
pub struct WDT_CTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&self) -> WDT_CTR0_R {
        WDT_CTR0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&self) -> WDT_CTR1_R {
        WDT_CTR1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&mut self) -> WDT_CTR0_W {
        WDT_CTR0_W { w: self }
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&mut self) -> WDT_CTR1_W {
        WDT_CTR1_W { w: self }
    }
}
