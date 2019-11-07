#[doc = "Reader of register SW_CMP_P_SEL"]
pub type R = crate::R<u32, super::SW_CMP_P_SEL>;
#[doc = "Writer for register SW_CMP_P_SEL"]
pub type W = crate::W<u32, super::SW_CMP_P_SEL>;
#[doc = "Register SW_CMP_P_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_CMP_P_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_SFPM`"]
pub type SW_SFPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_SFPM`"]
pub struct SW_SFPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SW_SFPT`"]
pub type SW_SFPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_SFPT`"]
pub struct SW_SFPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SW_SFPS`"]
pub type SW_SFPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_SFPS`"]
pub struct SW_SFPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_SFMA`"]
pub type SW_SFMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SFMA`"]
pub struct SW_SFMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFMA_W<'a> {
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
#[doc = "Reader of field `SW_SFMB`"]
pub type SW_SFMB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SFMB`"]
pub struct SW_SFMB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFMB_W<'a> {
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
#[doc = "Reader of field `SW_SFCA`"]
pub type SW_SFCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SFCA`"]
pub struct SW_SFCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFCA_W<'a> {
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
#[doc = "Reader of field `SW_SFCB`"]
pub type SW_SFCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_SFCB`"]
pub struct SW_SFCB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SFCB_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&self) -> SW_SFPM_R {
        SW_SFPM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&self) -> SW_SFPT_R {
        SW_SFPT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&self) -> SW_SFPS_R {
        SW_SFPS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&self) -> SW_SFMA_R {
        SW_SFMA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&self) -> SW_SFMB_R {
        SW_SFMB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&self) -> SW_SFCA_R {
        SW_SFCA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&self) -> SW_SFCB_R {
        SW_SFCB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&mut self) -> SW_SFPM_W {
        SW_SFPM_W { w: self }
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&mut self) -> SW_SFPT_W {
        SW_SFPT_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&mut self) -> SW_SFPS_W {
        SW_SFPS_W { w: self }
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&mut self) -> SW_SFMA_W {
        SW_SFMA_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&mut self) -> SW_SFMB_W {
        SW_SFMB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&mut self) -> SW_SFCA_W {
        SW_SFCA_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&mut self) -> SW_SFCB_W {
        SW_SFCB_W { w: self }
    }
}
