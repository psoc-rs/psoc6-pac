#[doc = "Reader of register SENSE_DUTY"]
pub type R = crate::R<u32, super::SENSE_DUTY>;
#[doc = "Writer for register SENSE_DUTY"]
pub type W = crate::W<u32, super::SENSE_DUTY>;
#[doc = "Register SENSE_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSE_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSE_WIDTH`"]
pub type SENSE_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENSE_WIDTH`"]
pub struct SENSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `SENSE_POL`"]
pub type SENSE_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE_POL`"]
pub struct SENSE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_POL_W<'a> {
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
#[doc = "Reader of field `OVERLAP_PHI1`"]
pub type OVERLAP_PHI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERLAP_PHI1`"]
pub struct OVERLAP_PHI1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERLAP_PHI1_W<'a> {
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
#[doc = "Reader of field `OVERLAP_PHI2`"]
pub type OVERLAP_PHI2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERLAP_PHI2`"]
pub struct OVERLAP_PHI2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERLAP_PHI2_W<'a> {
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
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&self) -> SENSE_WIDTH_R {
        SENSE_WIDTH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&self) -> SENSE_POL_R {
        SENSE_POL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&self) -> OVERLAP_PHI1_R {
        OVERLAP_PHI1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&self) -> OVERLAP_PHI2_R {
        OVERLAP_PHI2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&mut self) -> SENSE_WIDTH_W {
        SENSE_WIDTH_W { w: self }
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&mut self) -> SENSE_POL_W {
        SENSE_POL_W { w: self }
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&mut self) -> OVERLAP_PHI1_W {
        OVERLAP_PHI1_W { w: self }
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&mut self) -> OVERLAP_PHI2_W {
        OVERLAP_PHI2_W { w: self }
    }
}
