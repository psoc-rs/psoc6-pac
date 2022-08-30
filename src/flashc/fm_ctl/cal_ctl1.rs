#[doc = "Register `CAL_CTL1` reader"]
pub struct R(crate::R<CAL_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL1` writer"]
pub struct W(crate::W<CAL_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL1_SPEC>;
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
impl From<crate::W<CAL_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCT_TRIM_HI_HV` reader - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VCT_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCT_TRIM_HI_HV` writer - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VCT_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CDAC_HI_HV` reader - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CDAC_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CDAC_HI_HV` writer - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
pub type CDAC_HI_HV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `VBG_TRIM_HI_HV` reader - HI Bandgap Voltage trim control."]
pub type VBG_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_TRIM_HI_HV` writer - HI Bandgap Voltage trim control."]
pub type VBG_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `VBG_TC_TRIM_HI_HV` reader - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VBG_TC_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBG_TC_TRIM_HI_HV` writer - HI Bandgap Voltage Temperature Compensation trim control."]
pub type VBG_TC_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `IPREF_TRIM_HI_HV` reader - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPREF_TRIM_HI_HV` writer - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_hi_hv(&self) -> VCT_TRIM_HI_HV_R {
        VCT_TRIM_HI_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_hi_hv(&self) -> CDAC_HI_HV_R {
        CDAC_HI_HV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - HI Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_hi_hv(&self) -> VBG_TRIM_HI_HV_R {
        VBG_TRIM_HI_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vbg_tc_trim_hi_hv(&self) -> VBG_TC_TRIM_HI_HV_R {
        VBG_TC_TRIM_HI_HV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&self) -> IPREF_TRIM_HI_HV_R {
        IPREF_TRIM_HI_HV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vct_trim_hi_hv(&mut self) -> VCT_TRIM_HI_HV_W<0> {
        VCT_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bits 5:7 - HI Temperature compensated trim DAC. To control Vcstat slope for Vpos."]
    #[inline(always)]
    pub fn cdac_hi_hv(&mut self) -> CDAC_HI_HV_W<5> {
        CDAC_HI_HV_W::new(self)
    }
    #[doc = "Bits 8:12 - HI Bandgap Voltage trim control."]
    #[inline(always)]
    pub fn vbg_trim_hi_hv(&mut self) -> VBG_TRIM_HI_HV_W<8> {
        VBG_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bits 13:15 - HI Bandgap Voltage Temperature Compensation trim control."]
    #[inline(always)]
    pub fn vbg_tc_trim_hi_hv(&mut self) -> VBG_TC_TRIM_HI_HV_W<13> {
        VBG_TC_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bits 16:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&mut self) -> IPREF_TRIM_HI_HV_W<16> {
        IPREF_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control BG HI trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl1](index.html) module"]
pub struct CAL_CTL1_SPEC;
impl crate::RegisterSpec for CAL_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl1::R](R) reader structure"]
impl crate::Readable for CAL_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl1::W](W) writer structure"]
impl crate::Writable for CAL_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_CTL1 to value 0x0008_8f8f"]
impl crate::Resettable for CAL_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_8f8f
    }
}
