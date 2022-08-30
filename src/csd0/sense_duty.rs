#[doc = "Register `SENSE_DUTY` reader"]
pub struct R(crate::R<SENSE_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSE_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSE_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSE_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSE_DUTY` writer"]
pub struct W(crate::W<SENSE_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSE_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SENSE_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSE_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSE_WIDTH` reader - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub type SENSE_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SENSE_WIDTH` writer - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
pub type SENSE_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSE_DUTY_SPEC, u16, u16, 12, O>;
#[doc = "Field `SENSE_POL` reader - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub type SENSE_POL_R = crate::BitReader<bool>;
#[doc = "Field `SENSE_POL` writer - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
pub type SENSE_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSE_DUTY_SPEC, bool, O>;
#[doc = "Field `OVERLAP_PHI1` reader - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub type OVERLAP_PHI1_R = crate::BitReader<bool>;
#[doc = "Field `OVERLAP_PHI1` writer - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
pub type OVERLAP_PHI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSE_DUTY_SPEC, bool, O>;
#[doc = "Field `OVERLAP_PHI2` reader - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub type OVERLAP_PHI2_R = crate::BitReader<bool>;
#[doc = "Field `OVERLAP_PHI2` writer - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
pub type OVERLAP_PHI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSE_DUTY_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&self) -> SENSE_WIDTH_R {
        SENSE_WIDTH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&self) -> SENSE_POL_R {
        SENSE_POL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&self) -> OVERLAP_PHI1_R {
        OVERLAP_PHI1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&self) -> OVERLAP_PHI2_R {
        OVERLAP_PHI2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap, if used), if this rule is violated the result is undefined. Note that this feature is not available when SEL_LFSR_MSB (PRS) is selected."]
    #[inline(always)]
    pub fn sense_width(&mut self) -> SENSE_WIDTH_W<0> {
        SENSE_WIDTH_W::new(self)
    }
    #[doc = "Bit 16 - Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn sense_pol(&mut self) -> SENSE_POL_W<16> {
        SENSE_POL_W::new(self)
    }
    #[doc = "Bit 18 - NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn overlap_phi1(&mut self) -> OVERLAP_PHI1_W<18> {
        OVERLAP_PHI1_W::new(self)
    }
    #[doc = "Bit 19 - Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn overlap_phi2(&mut self) -> OVERLAP_PHI2_W<19> {
        OVERLAP_PHI2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sense clock duty cycle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sense_duty](index.html) module"]
pub struct SENSE_DUTY_SPEC;
impl crate::RegisterSpec for SENSE_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sense_duty::R](R) reader structure"]
impl crate::Readable for SENSE_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sense_duty::W](W) writer structure"]
impl crate::Writable for SENSE_DUTY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSE_DUTY to value 0"]
impl crate::Resettable for SENSE_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
