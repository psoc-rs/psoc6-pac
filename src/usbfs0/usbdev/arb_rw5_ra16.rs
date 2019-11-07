#[doc = "Reader of register ARB_RW5_RA16"]
pub type R = crate::R<u32, super::ARB_RW5_RA16>;
#[doc = "Writer for register ARB_RW5_RA16"]
pub type W = crate::W<u32, super::ARB_RW5_RA16>;
#[doc = "Register ARB_RW5_RA16 `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW5_RA16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RA16`"]
pub type RA16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RA16`"]
pub struct RA16_W<'a> {
    w: &'a mut W,
}
impl<'a> RA16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    pub fn ra16(&self) -> RA16_R {
        RA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    pub fn ra16(&mut self) -> RA16_W {
        RA16_W { w: self }
    }
}
