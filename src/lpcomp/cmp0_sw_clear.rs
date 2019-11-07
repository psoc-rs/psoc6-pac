#[doc = "Reader of register CMP0_SW_CLEAR"]
pub type R = crate::R<u32, super::CMP0_SW_CLEAR>;
#[doc = "Writer for register CMP0_SW_CLEAR"]
pub type W = crate::W<u32, super::CMP0_SW_CLEAR>;
#[doc = "Register CMP0_SW_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP0_SW_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP0_IP0`"]
pub type CMP0_IP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_IP0`"]
pub struct CMP0_IP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_IP0_W<'a> {
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
#[doc = "Reader of field `CMP0_AP0`"]
pub type CMP0_AP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_AP0`"]
pub struct CMP0_AP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_AP0_W<'a> {
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
#[doc = "Reader of field `CMP0_BP0`"]
pub type CMP0_BP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_BP0`"]
pub struct CMP0_BP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_BP0_W<'a> {
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
#[doc = "Reader of field `CMP0_IN0`"]
pub type CMP0_IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_IN0`"]
pub struct CMP0_IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_IN0_W<'a> {
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
#[doc = "Reader of field `CMP0_AN0`"]
pub type CMP0_AN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_AN0`"]
pub struct CMP0_AN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_AN0_W<'a> {
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
#[doc = "Reader of field `CMP0_BN0`"]
pub type CMP0_BN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_BN0`"]
pub struct CMP0_BN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_BN0_W<'a> {
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
#[doc = "Reader of field `CMP0_VN0`"]
pub type CMP0_VN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0_VN0`"]
pub struct CMP0_VN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VN0_W<'a> {
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
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ip0(&self) -> CMP0_IP0_R {
        CMP0_IP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ap0(&self) -> CMP0_AP0_R {
        CMP0_AP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bp0(&self) -> CMP0_BP0_R {
        CMP0_BP0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_in0(&self) -> CMP0_IN0_R {
        CMP0_IN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_an0(&self) -> CMP0_AN0_R {
        CMP0_AN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bn0(&self) -> CMP0_BN0_R {
        CMP0_BN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_vn0(&self) -> CMP0_VN0_R {
        CMP0_VN0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ip0(&mut self) -> CMP0_IP0_W {
        CMP0_IP0_W { w: self }
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ap0(&mut self) -> CMP0_AP0_W {
        CMP0_AP0_W { w: self }
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bp0(&mut self) -> CMP0_BP0_W {
        CMP0_BP0_W { w: self }
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_in0(&mut self) -> CMP0_IN0_W {
        CMP0_IN0_W { w: self }
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_an0(&mut self) -> CMP0_AN0_W {
        CMP0_AN0_W { w: self }
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bn0(&mut self) -> CMP0_BN0_W {
        CMP0_BN0_W { w: self }
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_vn0(&mut self) -> CMP0_VN0_W {
        CMP0_VN0_W { w: self }
    }
}
