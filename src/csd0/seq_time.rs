#[doc = "Register `SEQ_TIME` reader"]
pub struct R(crate::R<SEQ_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_TIME` writer"]
pub struct W(crate::W<SEQ_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_TIME_SPEC>;
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
impl From<crate::W<SEQ_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AZ_TIME` reader - Define Auto-Zero time in csd_sense cycles -1."]
pub type AZ_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AZ_TIME` writer - Define Auto-Zero time in csd_sense cycles -1."]
pub type AZ_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ_TIME_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&self) -> AZ_TIME_R {
        AZ_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&mut self) -> AZ_TIME_W<0> {
        AZ_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_time](index.html) module"]
pub struct SEQ_TIME_SPEC;
impl crate::RegisterSpec for SEQ_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_time::R](R) reader structure"]
impl crate::Readable for SEQ_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_time::W](W) writer structure"]
impl crate::Writable for SEQ_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_TIME to value 0"]
impl crate::Resettable for SEQ_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
