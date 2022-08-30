#[doc = "Register `CONN_PARAM2` reader"]
pub struct R(crate::R<CONN_PARAM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_PARAM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_PARAM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_PARAM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_PARAM2` writer"]
pub struct W(crate::W<CONN_PARAM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_PARAM2_SPEC>;
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
impl From<crate::W<CONN_PARAM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_PARAM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_INIT_H` reader - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
pub type CRC_INIT_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC_INIT_H` writer - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
pub type CRC_INIT_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_PARAM2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_h(&self) -> CRC_INIT_H_R {
        CRC_INIT_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper two bytes (23:8) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_h(&mut self) -> CRC_INIT_H_W<0> {
        CRC_INIT_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection parameter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param2](index.html) module"]
pub struct CONN_PARAM2_SPEC;
impl crate::RegisterSpec for CONN_PARAM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_param2::R](R) reader structure"]
impl crate::Readable for CONN_PARAM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_param2::W](W) writer structure"]
impl crate::Writable for CONN_PARAM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_PARAM2 to value 0"]
impl crate::Resettable for CONN_PARAM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
