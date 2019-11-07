#[doc = "Reader of register DIV_BY_625_CFG"]
pub type R = crate::R<u32, super::DIV_BY_625_CFG>;
#[doc = "Writer for register DIV_BY_625_CFG"]
pub type W = crate::W<u32, super::DIV_BY_625_CFG>;
#[doc = "Register DIV_BY_625_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DIV_BY_625_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIVIDEND`"]
pub type DIVIDEND_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVIDEND`"]
pub struct DIVIDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:23 - This field holds the dividend"]
    #[inline(always)]
    pub fn dividend(&self) -> DIVIDEND_R {
        DIVIDEND_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 8:23 - This field holds the dividend"]
    #[inline(always)]
    pub fn dividend(&mut self) -> DIVIDEND_W {
        DIVIDEND_W { w: self }
    }
}
