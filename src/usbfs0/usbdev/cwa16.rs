#[doc = "Register `CWA16` reader"]
pub struct R(crate::R<CWA16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWA16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWA16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWA16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWA16` writer"]
pub struct W(crate::W<CWA16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWA16_SPEC>;
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
impl From<crate::W<CWA16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWA16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWA16` reader - Write Address for Common Area"]
pub type CWA16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CWA16` writer - Write Address for Common Area"]
pub type CWA16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWA16_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&self) -> CWA16_R {
        CWA16_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&mut self) -> CWA16_W<0> {
        CWA16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Area Write Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa16](index.html) module"]
pub struct CWA16_SPEC;
impl crate::RegisterSpec for CWA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwa16::R](R) reader structure"]
impl crate::Readable for CWA16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwa16::W](W) writer structure"]
impl crate::Writable for CWA16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWA16 to value 0"]
impl crate::Resettable for CWA16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
