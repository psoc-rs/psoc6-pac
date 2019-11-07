#[doc = "Reader of register RD_DUMMY_CTL"]
pub type R = crate::R<u32, super::RD_DUMMY_CTL>;
#[doc = "Writer for register RD_DUMMY_CTL"]
pub type W = crate::W<u32, super::RD_DUMMY_CTL>;
#[doc = "Register RD_DUMMY_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RD_DUMMY_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIZE5`"]
pub type SIZE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE5`"]
pub struct SIZE5_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESENT`"]
pub struct PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn size5(&mut self) -> SIZE5_W {
        SIZE5_W { w: self }
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&mut self) -> PRESENT_W {
        PRESENT_W { w: self }
    }
}
