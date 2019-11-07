#[doc = "Reader of register CTDAC_SW_CLEAR"]
pub type R = crate::R<u32, super::CTDAC_SW_CLEAR>;
#[doc = "Writer for register CTDAC_SW_CLEAR"]
pub type W = crate::W<u32, super::CTDAC_SW_CLEAR>;
#[doc = "Register CTDAC_SW_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTDAC_SW_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTDD_CVD`"]
pub type CTDD_CVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDD_CVD`"]
pub struct CTDD_CVD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDD_CVD_W<'a> {
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
#[doc = "Reader of field `CTDO_CO6`"]
pub type CTDO_CO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDO_CO6`"]
pub struct CTDO_CO6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_CO6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_cvd(&self) -> CTDD_CVD_R {
        CTDD_CVD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_co6(&self) -> CTDO_CO6_R {
        CTDO_CO6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_cvd(&mut self) -> CTDD_CVD_W {
        CTDD_CVD_W { w: self }
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_co6(&mut self) -> CTDO_CO6_W {
        CTDO_CO6_W { w: self }
    }
}
