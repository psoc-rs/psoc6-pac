#[doc = "Reader of register CLK_TRIM_ILO_CTL"]
pub type R = crate::R<u32, super::CLK_TRIM_ILO_CTL>;
#[doc = "Writer for register CLK_TRIM_ILO_CTL"]
pub type W = crate::W<u32, super::CLK_TRIM_ILO_CTL>;
#[doc = "Register CLK_TRIM_ILO_CTL `reset()`'s with value 0x2c"]
impl crate::ResetValue for super::CLK_TRIM_ILO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2c
    }
}
#[doc = "Reader of field `ILO_FTRIM`"]
pub type ILO_FTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ILO_FTRIM`"]
pub struct ILO_FTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ILO_FTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&self) -> ILO_FTRIM_R {
        ILO_FTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&mut self) -> ILO_FTRIM_W {
        ILO_FTRIM_W { w: self }
    }
}
