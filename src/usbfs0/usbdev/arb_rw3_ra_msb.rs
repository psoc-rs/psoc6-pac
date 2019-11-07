#[doc = "Reader of register ARB_RW3_RA_MSB"]
pub type R = crate::R<u32, super::ARB_RW3_RA_MSB>;
#[doc = "Writer for register ARB_RW3_RA_MSB"]
pub type W = crate::W<u32, super::ARB_RW3_RA_MSB>;
#[doc = "Register ARB_RW3_RA_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW3_RA_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RA_MSB`"]
pub type RA_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RA_MSB`"]
pub struct RA_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_MSB_W<'a> {
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
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    pub fn ra_msb(&self) -> RA_MSB_R {
        RA_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    pub fn ra_msb(&mut self) -> RA_MSB_W {
        RA_MSB_W { w: self }
    }
}
