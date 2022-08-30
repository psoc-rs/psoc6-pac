#[doc = "Register `HV_CTL` reader"]
pub struct R(crate::R<HV_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HV_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HV_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HV_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HV_CTL` writer"]
pub struct W(crate::W<HV_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HV_CTL_SPEC>;
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
impl From<crate::W<HV_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HV_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_CLOCK_FREQ` reader - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
pub type TIMER_CLOCK_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_CLOCK_FREQ` writer - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
pub type TIMER_CLOCK_FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HV_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub fn timer_clock_freq(&self) -> TIMER_CLOCK_FREQ_R {
        TIMER_CLOCK_FREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the frequency in MHz of the timer clock 'clk_t' as provide to the flash macro. E.g., if '4', the timer clock 'clk_t' has a frequency of 4 MHz."]
    #[inline(always)]
    pub fn timer_clock_freq(&mut self) -> TIMER_CLOCK_FREQ_W<0> {
        TIMER_CLOCK_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High voltage control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hv_ctl](index.html) module"]
pub struct HV_CTL_SPEC;
impl crate::RegisterSpec for HV_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hv_ctl::R](R) reader structure"]
impl crate::Readable for HV_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hv_ctl::W](W) writer structure"]
impl crate::Writable for HV_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HV_CTL to value 0x32"]
impl crate::Resettable for HV_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x32
    }
}
