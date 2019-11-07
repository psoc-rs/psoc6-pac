#[doc = "Reader of register CWA16"]
pub type R = crate::R<u32, super::CWA16>;
#[doc = "Writer for register CWA16"]
pub type W = crate::W<u32, super::CWA16>;
#[doc = "Register CWA16 `reset()`'s with value 0"]
impl crate::ResetValue for super::CWA16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWA16`"]
pub type CWA16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CWA16`"]
pub struct CWA16_W<'a> {
    w: &'a mut W,
}
impl<'a> CWA16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&self) -> CWA16_R {
        CWA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&mut self) -> CWA16_W {
        CWA16_W { w: self }
    }
}
