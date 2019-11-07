#[doc = "Reader of register ARB_RW2_WA_MSB"]
pub type R = crate::R<u32, super::ARB_RW2_WA_MSB>;
#[doc = "Writer for register ARB_RW2_WA_MSB"]
pub type W = crate::W<u32, super::ARB_RW2_WA_MSB>;
#[doc = "Register ARB_RW2_WA_MSB `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW2_WA_MSB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WA_MSB`"]
pub type WA_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WA_MSB`"]
pub struct WA_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_MSB_W<'a> {
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
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&self) -> WA_MSB_R {
        WA_MSB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&mut self) -> WA_MSB_W {
        WA_MSB_W { w: self }
    }
}
