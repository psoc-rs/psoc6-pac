#[doc = "Register `SAMPLE_TIME23` reader"]
pub struct R(crate::R<SAMPLE_TIME23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_TIME23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_TIME23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_TIME23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_TIME23` writer"]
pub struct W(crate::W<SAMPLE_TIME23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_TIME23_SPEC>;
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
impl From<crate::W<SAMPLE_TIME23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_TIME23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_TIME2` reader - Sample time2"]
pub type SAMPLE_TIME2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_TIME2` writer - Sample time2"]
pub type SAMPLE_TIME2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_TIME23_SPEC, u16, u16, 10, O>;
#[doc = "Field `SAMPLE_TIME3` reader - Sample time3"]
pub type SAMPLE_TIME3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_TIME3` writer - Sample time3"]
pub type SAMPLE_TIME3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_TIME23_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&self) -> SAMPLE_TIME2_R {
        SAMPLE_TIME2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&self) -> SAMPLE_TIME3_R {
        SAMPLE_TIME3_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&mut self) -> SAMPLE_TIME2_W<0> {
        SAMPLE_TIME2_W::new(self)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&mut self) -> SAMPLE_TIME3_W<16> {
        SAMPLE_TIME3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample time specification ST2 and ST3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_time23](index.html) module"]
pub struct SAMPLE_TIME23_SPEC;
impl crate::RegisterSpec for SAMPLE_TIME23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_time23::R](R) reader structure"]
impl crate::Readable for SAMPLE_TIME23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_time23::W](W) writer structure"]
impl crate::Writable for SAMPLE_TIME23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLE_TIME23 to value 0x0003_0003"]
impl crate::Resettable for SAMPLE_TIME23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
