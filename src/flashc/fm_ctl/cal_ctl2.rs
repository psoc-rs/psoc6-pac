#[doc = "Register `CAL_CTL2` reader"]
pub struct R(crate::R<CAL_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL2` writer"]
pub struct W(crate::W<CAL_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL2_SPEC>;
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
impl From<crate::W<CAL_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICREF_TRIM_LO_HV` reader - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICREF_TRIM_LO_HV` writer - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` reader - LO Bandgap Current Temperature Compensation trim control"]
pub type ICREF_TC_TRIM_LO_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICREF_TC_TRIM_LO_HV` writer - LO Bandgap Current Temperature Compensation trim control"]
pub type ICREF_TC_TRIM_LO_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `ICREF_TRIM_HI_HV` reader - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICREF_TRIM_HI_HV` writer - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `ICREF_TC_TRIM_HI_HV` reader - HI Bandgap Current Temperature Compensation trim control."]
pub type ICREF_TC_TRIM_HI_HV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICREF_TC_TRIM_HI_HV` writer - HI Bandgap Current Temperature Compensation trim control."]
pub type ICREF_TC_TRIM_HI_HV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_CTL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `VREF_SEL_HV` reader - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
pub type VREF_SEL_HV_R = crate::BitReader<bool>;
#[doc = "Field `VREF_SEL_HV` writer - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
pub type VREF_SEL_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL2_SPEC, bool, O>;
#[doc = "Field `IREF_SEL_HV` reader - Current reference: '0': internal current reference '1': external current reference"]
pub type IREF_SEL_HV_R = crate::BitReader<bool>;
#[doc = "Field `IREF_SEL_HV` writer - Current reference: '0': internal current reference '1': external current reference"]
pub type IREF_SEL_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL2_SPEC, bool, O>;
#[doc = "Field `FM_ACTIVE_HV` reader - 0: No Action 1: Forces FM SYS in active mode"]
pub type FM_ACTIVE_HV_R = crate::BitReader<bool>;
#[doc = "Field `FM_ACTIVE_HV` writer - 0: No Action 1: Forces FM SYS in active mode"]
pub type FM_ACTIVE_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL2_SPEC, bool, O>;
#[doc = "Field `TURBO_EXT_HV` reader - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
pub type TURBO_EXT_HV_R = crate::BitReader<bool>;
#[doc = "Field `TURBO_EXT_HV` writer - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
pub type TURBO_EXT_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> ICREF_TRIM_LO_HV_R {
        ICREF_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&self) -> ICREF_TC_TRIM_LO_HV_R {
        ICREF_TC_TRIM_LO_HV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> ICREF_TRIM_HI_HV_R {
        ICREF_TRIM_HI_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn icref_tc_trim_hi_hv(&self) -> ICREF_TC_TRIM_HI_HV_R {
        ICREF_TC_TRIM_HI_HV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&self) -> VREF_SEL_HV_R {
        VREF_SEL_HV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&self) -> IREF_SEL_HV_R {
        IREF_SEL_HV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&self) -> FM_ACTIVE_HV_R {
        FM_ACTIVE_HV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub fn turbo_ext_hv(&self) -> TURBO_EXT_HV_R {
        TURBO_EXT_HV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&mut self) -> ICREF_TRIM_LO_HV_W<0> {
        ICREF_TRIM_LO_HV_W::new(self)
    }
    #[doc = "Bits 5:7 - LO Bandgap Current Temperature Compensation trim control"]
    #[inline(always)]
    pub fn icref_tc_trim_lo_hv(&mut self) -> ICREF_TC_TRIM_LO_HV_W<5> {
        ICREF_TC_TRIM_LO_HV_W::new(self)
    }
    #[doc = "Bits 8:12 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&mut self) -> ICREF_TRIM_HI_HV_W<8> {
        ICREF_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bits 13:15 - HI Bandgap Current Temperature Compensation trim control."]
    #[inline(always)]
    pub fn icref_tc_trim_hi_hv(&mut self) -> ICREF_TC_TRIM_HI_HV_W<13> {
        ICREF_TC_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bit 16 - Voltage reference: '0': internal bandgap reference '1': external voltage reference"]
    #[inline(always)]
    pub fn vref_sel_hv(&mut self) -> VREF_SEL_HV_W<16> {
        VREF_SEL_HV_W::new(self)
    }
    #[doc = "Bit 17 - Current reference: '0': internal current reference '1': external current reference"]
    #[inline(always)]
    pub fn iref_sel_hv(&mut self) -> IREF_SEL_HV_W<17> {
        IREF_SEL_HV_W::new(self)
    }
    #[doc = "Bit 18 - 0: No Action 1: Forces FM SYS in active mode"]
    #[inline(always)]
    pub fn fm_active_hv(&mut self) -> FM_ACTIVE_HV_W<18> {
        FM_ACTIVE_HV_W::new(self)
    }
    #[doc = "Bit 19 - 0: turbo signal generated internally 1: turbo cleared by clk_pump_ext HI"]
    #[inline(always)]
    pub fn turbo_ext_hv(&mut self) -> TURBO_EXT_HV_W<19> {
        TURBO_EXT_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control BG LO&HI ipref trim, ref sel, fm_active, turbo_ext\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl2](index.html) module"]
pub struct CAL_CTL2_SPEC;
impl crate::RegisterSpec for CAL_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl2::R](R) reader structure"]
impl crate::Readable for CAL_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl2::W](W) writer structure"]
impl crate::Writable for CAL_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_CTL2 to value 0x7070"]
impl crate::Resettable for CAL_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7070
    }
}
