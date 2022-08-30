#[doc = "Register `CONN_TXMEM_BASE_ADDR_DLE` reader"]
pub struct R(crate::R<CONN_TXMEM_BASE_ADDR_DLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_TXMEM_BASE_ADDR_DLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_TXMEM_BASE_ADDR_DLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_TXMEM_BASE_ADDR_DLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_TXMEM_BASE_ADDR_DLE` writer"]
pub struct W(crate::W<CONN_TXMEM_BASE_ADDR_DLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_TXMEM_BASE_ADDR_DLE_SPEC>;
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
impl From<crate::W<CONN_TXMEM_BASE_ADDR_DLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_TXMEM_BASE_ADDR_DLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_TX_MEM_BASE_ADDR_DLE` reader - Data to Tx memory are written as 32-bit wide data. This memory is valid only if DLE is set."]
pub type CONN_TX_MEM_BASE_ADDR_DLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONN_TX_MEM_BASE_ADDR_DLE` writer - Data to Tx memory are written as 32-bit wide data. This memory is valid only if DLE is set."]
pub type CONN_TX_MEM_BASE_ADDR_DLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_TXMEM_BASE_ADDR_DLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data to Tx memory are written as 32-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn conn_tx_mem_base_addr_dle(&self) -> CONN_TX_MEM_BASE_ADDR_DLE_R {
        CONN_TX_MEM_BASE_ADDR_DLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to Tx memory are written as 32-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn conn_tx_mem_base_addr_dle(&mut self) -> CONN_TX_MEM_BASE_ADDR_DLE_W<0> {
        CONN_TX_MEM_BASE_ADDR_DLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLE Connection TX memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_txmem_base_addr_dle](index.html) module"]
pub struct CONN_TXMEM_BASE_ADDR_DLE_SPEC;
impl crate::RegisterSpec for CONN_TXMEM_BASE_ADDR_DLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_txmem_base_addr_dle::R](R) reader structure"]
impl crate::Readable for CONN_TXMEM_BASE_ADDR_DLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_txmem_base_addr_dle::W](W) writer structure"]
impl crate::Writable for CONN_TXMEM_BASE_ADDR_DLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_TXMEM_BASE_ADDR_DLE to value 0"]
impl crate::Resettable for CONN_TXMEM_BASE_ADDR_DLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
