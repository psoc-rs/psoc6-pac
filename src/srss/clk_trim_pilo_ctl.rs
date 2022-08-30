#[doc = "Register `CLK_TRIM_PILO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_PILO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_PILO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_PILO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_PILO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_PILO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_PILO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_PILO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_PILO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_PILO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PILO_CFREQ` reader - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PILO_CFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_CFREQ` writer - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PILO_CFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `PILO_OSC_TRIM` reader - Trim for current in oscillator block."]
pub type PILO_OSC_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_OSC_TRIM` writer - Trim for current in oscillator block."]
pub type PILO_OSC_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PILO_COMP_TRIM` reader - Trim for comparator bias current."]
pub type PILO_COMP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_COMP_TRIM` writer - Trim for comparator bias current."]
pub type PILO_COMP_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PILO_NBIAS_TRIM` reader - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PILO_NBIAS_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_NBIAS_TRIM` writer - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PILO_NBIAS_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PILO_RES_TRIM` reader - Trim for beta-multiplier branch current"]
pub type PILO_RES_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_RES_TRIM` writer - Trim for beta-multiplier branch current"]
pub type PILO_RES_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PILO_ISLOPE_TRIM` reader - Trim for beta-multiplier current slope"]
pub type PILO_ISLOPE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_ISLOPE_TRIM` writer - Trim for beta-multiplier current slope"]
pub type PILO_ISLOPE_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PILO_VTDIFF_TRIM` reader - Trim for VT-DIFF output (internal power supply)"]
pub type PILO_VTDIFF_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PILO_VTDIFF_TRIM` writer - Trim for VT-DIFF output (internal power supply)"]
pub type PILO_VTDIFF_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&self) -> PILO_CFREQ_R {
        PILO_CFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&self) -> PILO_OSC_TRIM_R {
        PILO_OSC_TRIM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&self) -> PILO_COMP_TRIM_R {
        PILO_COMP_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&self) -> PILO_NBIAS_TRIM_R {
        PILO_NBIAS_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&self) -> PILO_RES_TRIM_R {
        PILO_RES_TRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&self) -> PILO_ISLOPE_TRIM_R {
        PILO_ISLOPE_TRIM_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&self) -> PILO_VTDIFF_TRIM_R {
        PILO_VTDIFF_TRIM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&mut self) -> PILO_CFREQ_W<0> {
        PILO_CFREQ_W::new(self)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&mut self) -> PILO_OSC_TRIM_W<12> {
        PILO_OSC_TRIM_W::new(self)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&mut self) -> PILO_COMP_TRIM_W<16> {
        PILO_COMP_TRIM_W::new(self)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&mut self) -> PILO_NBIAS_TRIM_W<18> {
        PILO_NBIAS_TRIM_W::new(self)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&mut self) -> PILO_RES_TRIM_W<20> {
        PILO_RES_TRIM_W::new(self)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&mut self) -> PILO_ISLOPE_TRIM_W<26> {
        PILO_ISLOPE_TRIM_W::new(self)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&mut self) -> PILO_VTDIFF_TRIM_W<28> {
        PILO_VTDIFF_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PILO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl](index.html) module"]
pub struct CLK_TRIM_PILO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_pilo_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL to value 0x0108_500f"]
impl crate::Resettable for CLK_TRIM_PILO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0108_500f
    }
}
