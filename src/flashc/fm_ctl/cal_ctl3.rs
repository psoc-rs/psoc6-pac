#[doc = "Register `CAL_CTL3` reader"]
pub struct R(crate::R<CAL_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL3` writer"]
pub struct W(crate::W<CAL_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL3_SPEC>;
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
impl From<crate::W<CAL_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC_TRIM_HV` reader - Flash macro pump clock trim control."]
pub type OSC_TRIM_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC_TRIM_HV` writer - Flash macro pump clock trim control."]
pub type OSC_TRIM_HV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `OSC_RANGE_TRIM_HV` reader - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OSC_RANGE_TRIM_HV_R = crate::BitReader<bool>;
#[doc = "Field `OSC_RANGE_TRIM_HV` writer - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
pub type OSC_RANGE_TRIM_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL3_SPEC, bool, O>;
#[doc = "Field `IDAC_HV` reader - N/A"]
pub type IDAC_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDAC_HV` writer - N/A"]
pub type IDAC_HV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDAC_HV` reader - N/A"]
pub type SDAC_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDAC_HV` writer - N/A"]
pub type SDAC_HV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ITIM_HV` reader - Trimming of timing current"]
pub type ITIM_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITIM_HV` writer - Trimming of timing current"]
pub type ITIM_HV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL3_SPEC, u8, u8, 4, O>;
#[doc = "Field `VDDHI_HV` reader - 0': vdd<2.3V '1': vdd>=2.3V"]
pub type VDDHI_HV_R = crate::BitReader<bool>;
#[doc = "Field `VDDHI_HV` writer - 0': vdd<2.3V '1': vdd>=2.3V"]
pub type VDDHI_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL3_SPEC, bool, O>;
#[doc = "Field `TURBO_PULSEW_HV` reader - Turbo pulse width trim"]
pub type TURBO_PULSEW_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TURBO_PULSEW_HV` writer - Turbo pulse width trim"]
pub type TURBO_PULSEW_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `BGLO_EN_HV` reader - LO Bandgap Enable"]
pub type BGLO_EN_HV_R = crate::BitReader<bool>;
#[doc = "Field `BGLO_EN_HV` writer - LO Bandgap Enable"]
pub type BGLO_EN_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL3_SPEC, bool, O>;
#[doc = "Field `BGHI_EN_HV` reader - HI Bandgap Enable"]
pub type BGHI_EN_HV_R = crate::BitReader<bool>;
#[doc = "Field `BGHI_EN_HV` writer - HI Bandgap Enable"]
pub type BGHI_EN_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&self) -> OSC_TRIM_HV_R {
        OSC_TRIM_HV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&self) -> OSC_RANGE_TRIM_HV_R {
        OSC_RANGE_TRIM_HV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&self) -> IDAC_HV_R {
        IDAC_HV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&self) -> SDAC_HV_R {
        SDAC_HV_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&self) -> ITIM_HV_R {
        ITIM_HV_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&self) -> VDDHI_HV_R {
        VDDHI_HV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&self) -> TURBO_PULSEW_HV_R {
        TURBO_PULSEW_HV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&self) -> BGLO_EN_HV_R {
        BGLO_EN_HV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&self) -> BGHI_EN_HV_R {
        BGHI_EN_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro pump clock trim control."]
    #[inline(always)]
    pub fn osc_trim_hv(&mut self) -> OSC_TRIM_HV_W<0> {
        OSC_TRIM_HV_W::new(self)
    }
    #[doc = "Bit 4 - 0: Oscillator High Frequency Range 1: Oscillator Low Frequency range"]
    #[inline(always)]
    pub fn osc_range_trim_hv(&mut self) -> OSC_RANGE_TRIM_HV_W<4> {
        OSC_RANGE_TRIM_HV_W::new(self)
    }
    #[doc = "Bits 5:8 - N/A"]
    #[inline(always)]
    pub fn idac_hv(&mut self) -> IDAC_HV_W<5> {
        IDAC_HV_W::new(self)
    }
    #[doc = "Bits 9:10 - N/A"]
    #[inline(always)]
    pub fn sdac_hv(&mut self) -> SDAC_HV_W<9> {
        SDAC_HV_W::new(self)
    }
    #[doc = "Bits 11:14 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_hv(&mut self) -> ITIM_HV_W<11> {
        ITIM_HV_W::new(self)
    }
    #[doc = "Bit 15 - 0': vdd<2.3V '1': vdd>=2.3V"]
    #[inline(always)]
    pub fn vddhi_hv(&mut self) -> VDDHI_HV_W<15> {
        VDDHI_HV_W::new(self)
    }
    #[doc = "Bits 16:17 - Turbo pulse width trim"]
    #[inline(always)]
    pub fn turbo_pulsew_hv(&mut self) -> TURBO_PULSEW_HV_W<16> {
        TURBO_PULSEW_HV_W::new(self)
    }
    #[doc = "Bit 18 - LO Bandgap Enable"]
    #[inline(always)]
    pub fn bglo_en_hv(&mut self) -> BGLO_EN_HV_W<18> {
        BGLO_EN_HV_W::new(self)
    }
    #[doc = "Bit 19 - HI Bandgap Enable"]
    #[inline(always)]
    pub fn bghi_en_hv(&mut self) -> BGHI_EN_HV_W<19> {
        BGHI_EN_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control osc trim bits, idac, sdac, itim, bdac.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl3](index.html) module"]
pub struct CAL_CTL3_SPEC;
impl crate::RegisterSpec for CAL_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl3::R](R) reader structure"]
impl crate::Readable for CAL_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl3::W](W) writer structure"]
impl crate::Writable for CAL_CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_CTL3 to value 0xa504"]
impl crate::Resettable for CAL_CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa504
    }
}
