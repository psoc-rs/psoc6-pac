#[doc = "Register `RANGE_THRES` reader"]
pub struct R(crate::R<RANGE_THRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_THRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_THRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_THRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANGE_THRES` writer"]
pub struct W(crate::W<RANGE_THRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANGE_THRES_SPEC>;
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
impl From<crate::W<RANGE_THRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANGE_THRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANGE_LOW` reader - Low threshold for range detect."]
pub type RANGE_LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_LOW` writer - Low threshold for range detect."]
pub type RANGE_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RANGE_THRES_SPEC, u16, u16, 16, O>;
#[doc = "Field `RANGE_HIGH` reader - High threshold for range detect."]
pub type RANGE_HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_HIGH` writer - High threshold for range detect."]
pub type RANGE_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RANGE_THRES_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&self) -> RANGE_LOW_R {
        RANGE_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&self) -> RANGE_HIGH_R {
        RANGE_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&mut self) -> RANGE_LOW_W<0> {
        RANGE_LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&mut self) -> RANGE_HIGH_W<16> {
        RANGE_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global range detect threshold register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_thres](index.html) module"]
pub struct RANGE_THRES_SPEC;
impl crate::RegisterSpec for RANGE_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_thres::R](R) reader structure"]
impl crate::Readable for RANGE_THRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [range_thres::W](W) writer structure"]
impl crate::Writable for RANGE_THRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RANGE_THRES to value 0"]
impl crate::Resettable for RANGE_THRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
