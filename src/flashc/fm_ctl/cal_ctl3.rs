#[doc = "Reader of register CAL_CTL3"]
pub type R = crate::R<u32, super::CAL_CTL3>;
#[doc = "Writer for register CAL_CTL3"]
pub type W = crate::W<u32, super::CAL_CTL3>;
#[doc = "Register CAL_CTL3 `reset()`'s with value 0xa504"]
impl crate::ResetValue for super::CAL_CTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa504
    }
}
#[doc = "Reader of field `OSC_TRIM_HV`"]
pub type OSC_TRIM_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSC_TRIM_HV`"]
pub struct OSC_TRIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_TRIM_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `OSC_RANGE_TRIM_HV`"]
pub type OSC_RANGE_TRIM_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC_RANGE_TRIM_HV`"]
pub struct OSC_RANGE_TRIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_RANGE_TRIM_HV_W<'a> {
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
#[doc = "Reader of field `IDAC_HV`"]
pub type IDAC_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDAC_HV`"]
pub struct IDAC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SDAC_HV`"]
pub type SDAC_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDAC_HV`"]
pub struct SDAC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAC_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `ITIM_HV`"]
pub type ITIM_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITIM_HV`"]
pub struct ITIM_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> ITIM_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `VDDHI_HV`"]
pub type VDDHI_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDHI_HV`"]
pub struct VDDHI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDHI_HV_W<'a> {
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
#[doc = "Reader of field `TURBO_PULSEW_HV`"]
pub type TURBO_PULSEW_HV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TURBO_PULSEW_HV`"]
pub struct TURBO_PULSEW_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> TURBO_PULSEW_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `BGLO_EN_HV`"]
pub type BGLO_EN_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGLO_EN_HV`"]
pub struct BGLO_EN_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> BGLO_EN_HV_W<'a> {
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
#[doc = "Reader of field `BGHI_EN_HV`"]
pub type BGHI_EN_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGHI_EN_HV`"]
pub struct BGHI_EN_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> BGHI_EN_HV_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&self) -> OSC_TRIM_HV_R {
        OSC_TRIM_HV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&self) -> OSC_RANGE_TRIM_HV_R {
        OSC_RANGE_TRIM_HV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&self) -> IDAC_HV_R {
        IDAC_HV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&self) -> SDAC_HV_R {
        SDAC_HV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&self) -> ITIM_HV_R {
        ITIM_HV_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&self) -> VDDHI_HV_R {
        VDDHI_HV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&self) -> TURBO_PULSEW_HV_R {
        TURBO_PULSEW_HV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&self) -> BGLO_EN_HV_R {
        BGLO_EN_HV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&self) -> BGHI_EN_HV_R {
        BGHI_EN_HV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&mut self) -> OSC_TRIM_HV_W {
        OSC_TRIM_HV_W { w: self }
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&mut self) -> OSC_RANGE_TRIM_HV_W {
        OSC_RANGE_TRIM_HV_W { w: self }
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&mut self) -> IDAC_HV_W {
        IDAC_HV_W { w: self }
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&mut self) -> SDAC_HV_W {
        SDAC_HV_W { w: self }
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&mut self) -> ITIM_HV_W {
        ITIM_HV_W { w: self }
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&mut self) -> VDDHI_HV_W {
        VDDHI_HV_W { w: self }
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&mut self) -> TURBO_PULSEW_HV_W {
        TURBO_PULSEW_HV_W { w: self }
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&mut self) -> BGLO_EN_HV_W {
        BGLO_EN_HV_W { w: self }
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&mut self) -> BGHI_EN_HV_W {
        BGHI_EN_HV_W { w: self }
    }
}
