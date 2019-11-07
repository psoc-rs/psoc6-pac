#[doc = "Reader of register SW_HS_P_SEL"]
pub type R = crate::R<u32, super::SW_HS_P_SEL>;
#[doc = "Writer for register SW_HS_P_SEL"]
pub type W = crate::W<u32, super::SW_HS_P_SEL>;
#[doc = "Register SW_HS_P_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_HS_P_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_HMPM`"]
pub type SW_HMPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMPM`"]
pub struct SW_HMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMPM_W<'a> {
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
#[doc = "Reader of field `SW_HMPT`"]
pub type SW_HMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMPT`"]
pub struct SW_HMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMPT_W<'a> {
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
#[doc = "Reader of field `SW_HMPS`"]
pub type SW_HMPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMPS`"]
pub struct SW_HMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMPS_W<'a> {
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
#[doc = "Reader of field `SW_HMMA`"]
pub type SW_HMMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMMA`"]
pub struct SW_HMMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMMA_W<'a> {
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
#[doc = "Reader of field `SW_HMMB`"]
pub type SW_HMMB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMMB`"]
pub struct SW_HMMB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMMB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_HMCA`"]
pub type SW_HMCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMCA`"]
pub struct SW_HMCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SW_HMCB`"]
pub type SW_HMCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMCB`"]
pub struct SW_HMCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMCB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SW_HMRH`"]
pub type SW_HMRH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HMRH`"]
pub struct SW_HMRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HMRH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&self) -> SW_HMPM_R {
        SW_HMPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&self) -> SW_HMPT_R {
        SW_HMPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&self) -> SW_HMPS_R {
        SW_HMPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&self) -> SW_HMMA_R {
        SW_HMMA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&self) -> SW_HMMB_R {
        SW_HMMB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&self) -> SW_HMCA_R {
        SW_HMCA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&self) -> SW_HMCB_R {
        SW_HMCB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&self) -> SW_HMRH_R {
        SW_HMRH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&mut self) -> SW_HMPM_W {
        SW_HMPM_W { w: self }
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&mut self) -> SW_HMPT_W {
        SW_HMPT_W { w: self }
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&mut self) -> SW_HMPS_W {
        SW_HMPS_W { w: self }
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&mut self) -> SW_HMMA_W {
        SW_HMMA_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&mut self) -> SW_HMMB_W {
        SW_HMMB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&mut self) -> SW_HMCA_W {
        SW_HMCA_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&mut self) -> SW_HMCB_W {
        SW_HMCB_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&mut self) -> SW_HMRH_W {
        SW_HMRH_W { w: self }
    }
}
