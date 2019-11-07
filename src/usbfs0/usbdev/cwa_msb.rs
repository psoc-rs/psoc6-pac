#[doc = "Reader of register CWA_MSB"]
pub type R = crate::R<u32, super::CWA_MSB>;
#[doc = "Writer for register CWA_MSB"]
pub type W = crate::W<u32, super::CWA_MSB>;
#[doc = "Register CWA_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::CWA_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWA_MSB`"]
pub type CWA_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWA_MSB`"]
pub struct CWA_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CWA_MSB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&self) -> CWA_MSB_R {
        CWA_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&mut self) -> CWA_MSB_W {
        CWA_MSB_W { w: self }
    }
}
