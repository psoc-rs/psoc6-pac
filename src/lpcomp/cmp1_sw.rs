#[doc = "Reader of register CMP1_SW"]
pub type R = crate::R<u32, super::CMP1_SW>;
#[doc = "Writer for register CMP1_SW"]
pub type W = crate::W<u32, super::CMP1_SW>;
#[doc = "Register CMP1_SW `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP1_SW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP1_IP1`"]
pub type CMP1_IP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_IP1`"]
pub struct CMP1_IP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_IP1_W<'a> {
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
#[doc = "Reader of field `CMP1_AP1`"]
pub type CMP1_AP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_AP1`"]
pub struct CMP1_AP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_AP1_W<'a> {
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
#[doc = "Reader of field `CMP1_BP1`"]
pub type CMP1_BP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_BP1`"]
pub struct CMP1_BP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_BP1_W<'a> {
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
#[doc = "Reader of field `CMP1_IN1`"]
pub type CMP1_IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_IN1`"]
pub struct CMP1_IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_IN1_W<'a> {
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
#[doc = "Reader of field `CMP1_AN1`"]
pub type CMP1_AN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_AN1`"]
pub struct CMP1_AN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_AN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CMP1_BN1`"]
pub type CMP1_BN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_BN1`"]
pub struct CMP1_BN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_BN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CMP1_VN1`"]
pub type CMP1_VN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1_VN1`"]
pub struct CMP1_VN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_VN1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> CMP1_IP1_R {
        CMP1_IP1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> CMP1_AP1_R {
        CMP1_AP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> CMP1_BP1_R {
        CMP1_BP1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> CMP1_IN1_R {
        CMP1_IN1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> CMP1_AN1_R {
        CMP1_AN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> CMP1_BN1_R {
        CMP1_BN1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> CMP1_VN1_R {
        CMP1_VN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&mut self) -> CMP1_IP1_W {
        CMP1_IP1_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&mut self) -> CMP1_AP1_W {
        CMP1_AP1_W { w: self }
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&mut self) -> CMP1_BP1_W {
        CMP1_BP1_W { w: self }
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&mut self) -> CMP1_IN1_W {
        CMP1_IN1_W { w: self }
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&mut self) -> CMP1_AN1_W {
        CMP1_AN1_W { w: self }
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&mut self) -> CMP1_BN1_W {
        CMP1_BN1_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&mut self) -> CMP1_VN1_W {
        CMP1_VN1_W { w: self }
    }
}
