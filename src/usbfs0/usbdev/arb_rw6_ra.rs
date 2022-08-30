#[doc = "Register `ARB_RW6_RA` reader"]
pub struct R(crate::R<ARB_RW6_RA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW6_RA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW6_RA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW6_RA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW6_RA` writer"]
pub struct W(crate::W<ARB_RW6_RA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW6_RA_SPEC>;
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
impl From<crate::W<ARB_RW6_RA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW6_RA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Read Address for EP"]
pub type RA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RA` writer - Read Address for EP"]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARB_RW6_RA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<0> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw6_ra](index.html) module"]
pub struct ARB_RW6_RA_SPEC;
impl crate::RegisterSpec for ARB_RW6_RA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw6_ra::R](R) reader structure"]
impl crate::Readable for ARB_RW6_RA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw6_ra::W](W) writer structure"]
impl crate::Writable for ARB_RW6_RA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_RW6_RA to value 0"]
impl crate::Resettable for ARB_RW6_RA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
