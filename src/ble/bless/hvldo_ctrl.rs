#[doc = "Reader of register HVLDO_CTRL"]
pub type R = crate::R<u32, super::HVLDO_CTRL>;
#[doc = "Writer for register HVLDO_CTRL"]
pub type W = crate::W<u32, super::HVLDO_CTRL>;
#[doc = "Register HVLDO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::HVLDO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADFT_EN`"]
pub type ADFT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADFT_EN`"]
pub struct ADFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_EN_W<'a> {
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
#[doc = "Reader of field `ADFT_CTRL`"]
pub type ADFT_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADFT_CTRL`"]
pub struct ADFT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFT_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `VREF_EXT_EN`"]
pub type VREF_EXT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREF_EXT_EN`"]
pub struct VREF_EXT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_EXT_EN_W<'a> {
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
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&self) -> ADFT_EN_R {
        ADFT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&self) -> ADFT_CTRL_R {
        ADFT_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&self) -> VREF_EXT_EN_R {
        VREF_EXT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 31 - hvldo LV detect status"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADFT enable"]
    #[inline(always)]
    pub fn adft_en(&mut self) -> ADFT_EN_W {
        ADFT_EN_W { w: self }
    }
    #[doc = "Bits 1:4 - ADFT select"]
    #[inline(always)]
    pub fn adft_ctrl(&mut self) -> ADFT_CTRL_W {
        ADFT_CTRL_W { w: self }
    }
    #[doc = "Bit 6 - Vref ext input enable."]
    #[inline(always)]
    pub fn vref_ext_en(&mut self) -> VREF_EXT_EN_W {
        VREF_EXT_EN_W { w: self }
    }
}
