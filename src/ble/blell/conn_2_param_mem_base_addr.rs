#[doc = "Register `CONN_2_PARAM_MEM_BASE_ADDR` reader"]
pub struct R(crate::R<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_2_PARAM_MEM_BASE_ADDR` writer"]
pub struct W(crate::W<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>;
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
impl From<crate::W<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_2_PARAM_MEM_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_2_PARAM` reader - N/A"]
pub type CONN_2_PARAM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONN_2_PARAM` writer - N/A"]
pub type CONN_2_PARAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_2_PARAM_MEM_BASE_ADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_2_param(&self) -> CONN_2_PARAM_R {
        CONN_2_PARAM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_2_param(&mut self) -> CONN_2_PARAM_W<0> {
        CONN_2_PARAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection Parameter memory base address for connection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_2_param_mem_base_addr](index.html) module"]
pub struct CONN_2_PARAM_MEM_BASE_ADDR_SPEC;
impl crate::RegisterSpec for CONN_2_PARAM_MEM_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_2_param_mem_base_addr::R](R) reader structure"]
impl crate::Readable for CONN_2_PARAM_MEM_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_2_param_mem_base_addr::W](W) writer structure"]
impl crate::Writable for CONN_2_PARAM_MEM_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_2_PARAM_MEM_BASE_ADDR to value 0"]
impl crate::Resettable for CONN_2_PARAM_MEM_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
