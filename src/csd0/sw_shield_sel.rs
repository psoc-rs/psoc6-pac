#[doc = "Reader of register SW_SHIELD_SEL"]
pub type R = crate::R<u32, super::SW_SHIELD_SEL>;
#[doc = "Writer for register SW_SHIELD_SEL"]
pub type W = crate::W<u32, super::SW_SHIELD_SEL>;
#[doc = "Register SW_SHIELD_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_SHIELD_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_HCAV`"]
pub type SW_HCAV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCAV`"]
pub struct SW_HCAV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SW_HCAG`"]
pub type SW_HCAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCAG`"]
pub struct SW_HCAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SW_HCBV`"]
pub type SW_HCBV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCBV`"]
pub struct SW_HCBV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_HCBG`"]
pub type SW_HCBG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCBG`"]
pub struct SW_HCBG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SW_HCCV`"]
pub type SW_HCCV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HCCV`"]
pub struct SW_HCCV_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCV_W<'a> {
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
#[doc = "Reader of field `SW_HCCG`"]
pub type SW_HCCG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HCCG`"]
pub struct SW_HCCG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCG_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&self) -> SW_HCAV_R {
        SW_HCAV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&self) -> SW_HCAG_R {
        SW_HCAG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&self) -> SW_HCBV_R {
        SW_HCBV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&self) -> SW_HCBG_R {
        SW_HCBG_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&self) -> SW_HCCV_R {
        SW_HCCV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&self) -> SW_HCCG_R {
        SW_HCCG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&mut self) -> SW_HCAV_W {
        SW_HCAV_W { w: self }
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&mut self) -> SW_HCAG_W {
        SW_HCAG_W { w: self }
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&mut self) -> SW_HCBV_W {
        SW_HCBV_W { w: self }
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&mut self) -> SW_HCBG_W {
        SW_HCBG_W { w: self }
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&mut self) -> SW_HCCV_W {
        SW_HCCV_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&mut self) -> SW_HCCG_W {
        SW_HCCG_W { w: self }
    }
}
