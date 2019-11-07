#[doc = "Reader of register CTD_SW_CLEAR"]
pub type R = crate::R<u32, super::CTD_SW_CLEAR>;
#[doc = "Writer for register CTD_SW_CLEAR"]
pub type W = crate::W<u32, super::CTD_SW_CLEAR>;
#[doc = "Register CTD_SW_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTD_SW_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTDD_CRD`"]
pub type CTDD_CRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDD_CRD`"]
pub struct CTDD_CRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDD_CRD_W<'a> {
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
#[doc = "Reader of field `CTDS_CRS`"]
pub type CTDS_CRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDS_CRS`"]
pub struct CTDS_CRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDS_CRS_W<'a> {
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
#[doc = "Reader of field `CTDS_COR`"]
pub type CTDS_COR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDS_COR`"]
pub struct CTDS_COR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDS_COR_W<'a> {
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
#[doc = "Reader of field `CTDO_C6H`"]
pub type CTDO_C6H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDO_C6H`"]
pub struct CTDO_C6H_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_C6H_W<'a> {
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
#[doc = "Reader of field `CTDO_COS`"]
pub type CTDO_COS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDO_COS`"]
pub struct CTDO_COS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_COS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CTDH_COB`"]
pub type CTDH_COB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDH_COB`"]
pub struct CTDH_COB_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_COB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CTDH_CHD`"]
pub type CTDH_CHD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDH_CHD`"]
pub struct CTDH_CHD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CHD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CTDH_CA0`"]
pub type CTDH_CA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDH_CA0`"]
pub struct CTDH_CA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CA0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CTDH_CIS`"]
pub type CTDH_CIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDH_CIS`"]
pub struct CTDH_CIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_CIS_W<'a> {
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
#[doc = "Reader of field `CTDH_ILR`"]
pub type CTDH_ILR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDH_ILR`"]
pub struct CTDH_ILR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDH_ILR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_crd(&self) -> CTDD_CRD_R {
        CTDD_CRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_crs(&self) -> CTDS_CRS_R {
        CTDS_CRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_cor(&self) -> CTDS_COR_R {
        CTDS_COR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_c6h(&self) -> CTDO_C6H_R {
        CTDO_C6H_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_cos(&self) -> CTDO_COS_R {
        CTDO_COS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cob(&self) -> CTDH_COB_R {
        CTDH_COB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_chd(&self) -> CTDH_CHD_R {
        CTDH_CHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ca0(&self) -> CTDH_CA0_R {
        CTDH_CA0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cis(&self) -> CTDH_CIS_R {
        CTDH_CIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ilr(&self) -> CTDH_ILR_R {
        CTDH_ILR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_crd(&mut self) -> CTDD_CRD_W {
        CTDD_CRD_W { w: self }
    }
    #[doc = "Bit 4 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_crs(&mut self) -> CTDS_CRS_W {
        CTDS_CRS_W { w: self }
    }
    #[doc = "Bit 5 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_cor(&mut self) -> CTDS_COR_W {
        CTDS_COR_W { w: self }
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_c6h(&mut self) -> CTDO_C6H_W {
        CTDO_C6H_W { w: self }
    }
    #[doc = "Bit 9 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_cos(&mut self) -> CTDO_COS_W {
        CTDO_COS_W { w: self }
    }
    #[doc = "Bit 10 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cob(&mut self) -> CTDH_COB_W {
        CTDH_COB_W { w: self }
    }
    #[doc = "Bit 12 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_chd(&mut self) -> CTDH_CHD_W {
        CTDH_CHD_W { w: self }
    }
    #[doc = "Bit 13 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ca0(&mut self) -> CTDH_CA0_W {
        CTDH_CA0_W { w: self }
    }
    #[doc = "Bit 14 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cis(&mut self) -> CTDH_CIS_W {
        CTDH_CIS_W { w: self }
    }
    #[doc = "Bit 15 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ilr(&mut self) -> CTDH_ILR_W {
        CTDH_ILR_W { w: self }
    }
}
