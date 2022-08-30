#[doc = "Register `ENC_MEM_BASE_ADDR` reader"]
pub struct R(crate::R<ENC_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_MEM_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_MEM_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_MEM_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_MEM_BASE_ADDR` writer"]
pub struct W(crate::W<ENC_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_MEM_BASE_ADDR_SPEC>;
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
impl From<crate::W<ENC_MEM_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_MEM_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC_MEM` reader - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
pub type ENC_MEM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENC_MEM` writer - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
pub type ENC_MEM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENC_MEM_BASE_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn enc_mem(&self) -> ENC_MEM_R {
        ENC_MEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn enc_mem(&mut self) -> ENC_MEM_W<0> {
        ENC_MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_mem_base_addr](index.html) module"]
pub struct ENC_MEM_BASE_ADDR_SPEC;
impl crate::RegisterSpec for ENC_MEM_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_mem_base_addr::R](R) reader structure"]
impl crate::Readable for ENC_MEM_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_mem_base_addr::W](W) writer structure"]
impl crate::Writable for ENC_MEM_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_MEM_BASE_ADDR to value 0"]
impl crate::Resettable for ENC_MEM_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
