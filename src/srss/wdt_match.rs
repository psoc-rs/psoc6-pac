#[doc = "Reader of register WDT_MATCH"]
pub type R = crate::R<u32, super::WDT_MATCH>;
#[doc = "Writer for register WDT_MATCH"]
pub type W = crate::W<u32, super::WDT_MATCH>;
#[doc = "Register WDT_MATCH `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::WDT_MATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCH`"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `IGNORE_BITS`"]
pub type IGNORE_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IGNORE_BITS`"]
pub struct IGNORE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&self) -> IGNORE_BITS_R {
        IGNORE_BITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&mut self) -> IGNORE_BITS_W {
        IGNORE_BITS_W { w: self }
    }
}
