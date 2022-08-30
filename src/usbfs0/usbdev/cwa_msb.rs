#[doc = "Register `CWA_MSB` reader"]
pub struct R(crate::R<CWA_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWA_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWA_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWA_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWA_MSB` writer"]
pub struct W(crate::W<CWA_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWA_MSB_SPEC>;
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
impl From<crate::W<CWA_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWA_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWA_MSB` reader - Write Address for Common Area"]
pub type CWA_MSB_R = crate::BitReader<bool>;
#[doc = "Field `CWA_MSB` writer - Write Address for Common Area"]
pub type CWA_MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CWA_MSB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&self) -> CWA_MSB_R {
        CWA_MSB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&mut self) -> CWA_MSB_W<0> {
        CWA_MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Read Address value *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa_msb](index.html) module"]
pub struct CWA_MSB_SPEC;
impl crate::RegisterSpec for CWA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwa_msb::R](R) reader structure"]
impl crate::Readable for CWA_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwa_msb::W](W) writer structure"]
impl crate::Writable for CWA_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWA_MSB to value 0"]
impl crate::Resettable for CWA_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
