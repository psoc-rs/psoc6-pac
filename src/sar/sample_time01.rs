#[doc = "Register `SAMPLE_TIME01` reader"]
pub struct R(crate::R<SAMPLE_TIME01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_TIME01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_TIME01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_TIME01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_TIME01` writer"]
pub struct W(crate::W<SAMPLE_TIME01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_TIME01_SPEC>;
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
impl From<crate::W<SAMPLE_TIME01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_TIME01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_TIME0` reader - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SAMPLE_TIME0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_TIME0` writer - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SAMPLE_TIME0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_TIME01_SPEC, u16, u16, 10, O>;
#[doc = "Field `SAMPLE_TIME1` reader - Sample time1"]
pub type SAMPLE_TIME1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_TIME1` writer - Sample time1"]
pub type SAMPLE_TIME1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_TIME01_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&self) -> SAMPLE_TIME0_R {
        SAMPLE_TIME0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&self) -> SAMPLE_TIME1_R {
        SAMPLE_TIME1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&mut self) -> SAMPLE_TIME0_W<0> {
        SAMPLE_TIME0_W::new(self)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&mut self) -> SAMPLE_TIME1_W<16> {
        SAMPLE_TIME1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample time specification ST0 and ST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_time01](index.html) module"]
pub struct SAMPLE_TIME01_SPEC;
impl crate::RegisterSpec for SAMPLE_TIME01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_time01::R](R) reader structure"]
impl crate::Readable for SAMPLE_TIME01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_time01::W](W) writer structure"]
impl crate::Writable for SAMPLE_TIME01_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLE_TIME01 to value 0x0003_0003"]
impl crate::Resettable for SAMPLE_TIME01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
