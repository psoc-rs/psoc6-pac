#[doc = "Writer for register ACLK_CTL"]
pub type W = crate::W<u32, super::ACLK_CTL>;
#[doc = "Register ACLK_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACLK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACLK_GEN`"]
pub struct ACLK_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_GEN_W<'a> {
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
impl W {
    #[doc = "Bit 0 - A write to this register generates a ACLK pulse for the flash macro (also requires FM_CTL.IF_SEL to be '1')."]
    #[inline(always)]
    pub fn aclk_gen(&mut self) -> ACLK_GEN_W {
        ACLK_GEN_W { w: self }
    }
}
