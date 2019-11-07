#[doc = "Reader of register ARB_RW3_WA16"]
pub type R = crate::R<u32, super::ARB_RW3_WA16>;
#[doc = "Writer for register ARB_RW3_WA16"]
pub type W = crate::W<u32, super::ARB_RW3_WA16>;
#[doc = "Register ARB_RW3_WA16 `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW3_WA16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WA16`"]
pub type WA16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WA16`"]
pub struct WA16_W<'a> {
    w: &'a mut W,
}
impl<'a> WA16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Write Address for EP"]
    #[inline(always)]
    pub fn wa16(&self) -> WA16_R {
        WA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for EP"]
    #[inline(always)]
    pub fn wa16(&mut self) -> WA16_W {
        WA16_W { w: self }
    }
}
