#[doc = "Reader of register OA1_SW_CLEAR"]
pub type R = crate::R<u32, super::OA1_SW_CLEAR>;
#[doc = "Writer for register OA1_SW_CLEAR"]
pub type W = crate::W<u32, super::OA1_SW_CLEAR>;
#[doc = "Register OA1_SW_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OA1_SW_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1P_A03`"]
pub type OA1P_A03_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1P_A03`"]
pub struct OA1P_A03_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A03_W<'a> {
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
#[doc = "Reader of field `OA1P_A13`"]
pub type OA1P_A13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1P_A13`"]
pub struct OA1P_A13_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A13_W<'a> {
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
#[doc = "Reader of field `OA1P_A43`"]
pub type OA1P_A43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1P_A43`"]
pub struct OA1P_A43_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A43_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `OA1P_A73`"]
pub type OA1P_A73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1P_A73`"]
pub struct OA1P_A73_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1P_A73_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `OA1M_A22`"]
pub type OA1M_A22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1M_A22`"]
pub struct OA1M_A22_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1M_A22_W<'a> {
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
#[doc = "Reader of field `OA1M_A82`"]
pub type OA1M_A82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1M_A82`"]
pub struct OA1M_A82_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1M_A82_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `OA1O_D52`"]
pub type OA1O_D52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1O_D52`"]
pub struct OA1O_D52_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D52_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OA1O_D62`"]
pub type OA1O_D62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1O_D62`"]
pub struct OA1O_D62_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D62_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `OA1O_D82`"]
pub type OA1O_D82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1O_D82`"]
pub struct OA1O_D82_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1O_D82_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a03(&self) -> OA1P_A03_R {
        OA1P_A03_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a13(&self) -> OA1P_A13_R {
        OA1P_A13_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a43(&self) -> OA1P_A43_R {
        OA1P_A43_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a73(&self) -> OA1P_A73_R {
        OA1P_A73_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a22(&self) -> OA1M_A22_R {
        OA1M_A22_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a82(&self) -> OA1M_A82_R {
        OA1M_A82_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52(&self) -> OA1O_D52_R {
        OA1O_D52_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62(&self) -> OA1O_D62_R {
        OA1O_D62_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d82(&self) -> OA1O_D82_R {
        OA1O_D82_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a03(&mut self) -> OA1P_A03_W {
        OA1P_A03_W { w: self }
    }
    #[doc = "Bit 1 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a13(&mut self) -> OA1P_A13_W {
        OA1P_A13_W { w: self }
    }
    #[doc = "Bit 4 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a43(&mut self) -> OA1P_A43_W {
        OA1P_A43_W { w: self }
    }
    #[doc = "Bit 7 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a73(&mut self) -> OA1P_A73_W {
        OA1P_A73_W { w: self }
    }
    #[doc = "Bit 8 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a22(&mut self) -> OA1M_A22_W {
        OA1M_A22_W { w: self }
    }
    #[doc = "Bit 14 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a82(&mut self) -> OA1M_A82_W {
        OA1M_A82_W { w: self }
    }
    #[doc = "Bit 18 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52(&mut self) -> OA1O_D52_W {
        OA1O_D52_W { w: self }
    }
    #[doc = "Bit 19 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62(&mut self) -> OA1O_D62_W {
        OA1O_D62_W { w: self }
    }
    #[doc = "Bit 21 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d82(&mut self) -> OA1O_D82_W {
        OA1O_D82_W { w: self }
    }
}
