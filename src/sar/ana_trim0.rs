#[doc = "Reader of register ANA_TRIM0"]
pub type R = crate::R<u32, super::ANA_TRIM0>;
#[doc = "Writer for register ANA_TRIM0"]
pub type W = crate::W<u32, super::ANA_TRIM0>;
#[doc = "Register ANA_TRIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANA_TRIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP_TRIM`"]
pub type CAP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP_TRIM`"]
pub struct CAP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TRIMUNIT`"]
pub type TRIMUNIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIMUNIT`"]
pub struct TRIMUNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMUNIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&self) -> TRIMUNIT_R {
        TRIMUNIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W {
        CAP_TRIM_W { w: self }
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&mut self) -> TRIMUNIT_W {
        TRIMUNIT_W { w: self }
    }
}
