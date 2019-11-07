#[doc = "Reader of register CWA"]
pub type R = crate::R<u32, super::CWA>;
#[doc = "Writer for register CWA"]
pub type W = crate::W<u32, super::CWA>;
#[doc = "Register CWA `reset()`'s with value 0"]
impl crate::ResetValue for super::CWA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWA`"]
pub type CWA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWA`"]
pub struct CWA_W<'a> {
    w: &'a mut W,
}
impl<'a> CWA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&self) -> CWA_R {
        CWA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&mut self) -> CWA_W {
        CWA_W { w: self }
    }
}
