#[doc = "Register `SCRATCH_CTL` reader"]
pub struct R(crate::R<SCRATCH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRATCH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRATCH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRATCH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRATCH_CTL` writer"]
pub struct W(crate::W<SCRATCH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRATCH_CTL_SPEC>;
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
impl From<crate::W<SCRATCH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRATCH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMMY32` reader - Scratchpad register fields. Provided for test purposes."]
pub type DUMMY32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUMMY32` writer - Scratchpad register fields. Provided for test purposes."]
pub type DUMMY32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCRATCH_CTL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Scratchpad register fields. Provided for test purposes."]
    #[inline(always)]
    pub fn dummy32(&self) -> DUMMY32_R {
        DUMMY32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratchpad register fields. Provided for test purposes."]
    #[inline(always)]
    pub fn dummy32(&mut self) -> DUMMY32_W<0> {
        DUMMY32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scratch Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch_ctl](index.html) module"]
pub struct SCRATCH_CTL_SPEC;
impl crate::RegisterSpec for SCRATCH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scratch_ctl::R](R) reader structure"]
impl crate::Readable for SCRATCH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scratch_ctl::W](W) writer structure"]
impl crate::Writable for SCRATCH_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRATCH_CTL to value 0"]
impl crate::Resettable for SCRATCH_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
