#[doc = "Reader of register MCWDT_MATCH"]
pub type R = crate::R<u32, super::MCWDT_MATCH>;
#[doc = "Writer for register MCWDT_MATCH"]
pub type W = crate::W<u32, super::MCWDT_MATCH>;
#[doc = "Register MCWDT_MATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::MCWDT_MATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_MATCH0`"]
pub type WDT_MATCH0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDT_MATCH0`"]
pub struct WDT_MATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_MATCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WDT_MATCH1`"]
pub type WDT_MATCH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDT_MATCH1`"]
pub struct WDT_MATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_MATCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&self) -> WDT_MATCH0_R {
        WDT_MATCH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&self) -> WDT_MATCH1_R {
        WDT_MATCH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&mut self) -> WDT_MATCH0_W {
        WDT_MATCH0_W { w: self }
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&mut self) -> WDT_MATCH1_W {
        WDT_MATCH1_W { w: self }
    }
}
