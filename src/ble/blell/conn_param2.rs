#[doc = "Reader of register CONN_PARAM2"]
pub type R = crate::R<u32, super::CONN_PARAM2>;
#[doc = "Writer for register CONN_PARAM2"]
pub type W = crate::W<u32, super::CONN_PARAM2>;
#[doc = "Register CONN_PARAM2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_PARAM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC_INIT_H`"]
pub type CRC_INIT_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRC_INIT_H`"]
pub struct CRC_INIT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_h(&self) -> CRC_INIT_H_R {
        CRC_INIT_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_h(&mut self) -> CRC_INIT_H_W {
        CRC_INIT_H_W { w: self }
    }
}
