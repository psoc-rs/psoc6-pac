#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDAC_EMPTY`"]
pub type VDAC_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDAC_EMPTY`"]
pub struct VDAC_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> VDAC_EMPTY_W<'a> {
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
    #[doc = "Bit 0 - VDAC Interrupt: hardware sets this interrupt when VDAC next value field is empty, i.e. was copied to the current VALUE. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn vdac_empty(&self) -> VDAC_EMPTY_R {
        VDAC_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDAC Interrupt: hardware sets this interrupt when VDAC next value field is empty, i.e. was copied to the current VALUE. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn vdac_empty(&mut self) -> VDAC_EMPTY_W {
        VDAC_EMPTY_W { w: self }
    }
}
