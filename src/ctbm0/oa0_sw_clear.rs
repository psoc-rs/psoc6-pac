#[doc = "Reader of register OA0_SW_CLEAR"]
pub type R = crate::R<u32, super::OA0_SW_CLEAR>;
#[doc = "Writer for register OA0_SW_CLEAR"]
pub type W = crate::W<u32, super::OA0_SW_CLEAR>;
#[doc = "Register OA0_SW_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OA0_SW_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA0P_A00`"]
pub type OA0P_A00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0P_A00`"]
pub struct OA0P_A00_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A00_W<'a> {
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
#[doc = "Reader of field `OA0P_A20`"]
pub type OA0P_A20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0P_A20`"]
pub struct OA0P_A20_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `OA0P_A30`"]
pub type OA0P_A30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0P_A30`"]
pub struct OA0P_A30_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0P_A30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OA0M_A11`"]
pub type OA0M_A11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0M_A11`"]
pub struct OA0M_A11_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0M_A11_W<'a> {
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
#[doc = "Reader of field `OA0M_A81`"]
pub type OA0M_A81_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0M_A81`"]
pub struct OA0M_A81_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0M_A81_W<'a> {
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
#[doc = "Reader of field `OA0O_D51`"]
pub type OA0O_D51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0O_D51`"]
pub struct OA0O_D51_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0O_D51_W<'a> {
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
#[doc = "Reader of field `OA0O_D81`"]
pub type OA0O_D81_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA0O_D81`"]
pub struct OA0O_D81_W<'a> {
    w: &'a mut W,
}
impl<'a> OA0O_D81_W<'a> {
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
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&self) -> OA0P_A00_R {
        OA0P_A00_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&self) -> OA0P_A20_R {
        OA0P_A20_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&self) -> OA0P_A30_R {
        OA0P_A30_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&self) -> OA0M_A11_R {
        OA0M_A11_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&self) -> OA0M_A81_R {
        OA0M_A81_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&self) -> OA0O_D51_R {
        OA0O_D51_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&self) -> OA0O_D81_R {
        OA0O_D81_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&mut self) -> OA0P_A00_W {
        OA0P_A00_W { w: self }
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&mut self) -> OA0P_A20_W {
        OA0P_A20_W { w: self }
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&mut self) -> OA0P_A30_W {
        OA0P_A30_W { w: self }
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&mut self) -> OA0M_A11_W {
        OA0M_A11_W { w: self }
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&mut self) -> OA0M_A81_W {
        OA0M_A81_W { w: self }
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&mut self) -> OA0O_D51_W {
        OA0O_D51_W { w: self }
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&mut self) -> OA0O_D81_W {
        OA0O_D81_W { w: self }
    }
}
