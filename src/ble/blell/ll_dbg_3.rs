#[doc = "Register `LL_DBG_3` reader"]
pub struct R(crate::R<LL_DBG_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONN_RX_WR_PTR_STORE` reader - Connection receive FIFO stored write pointer for pointer restore"]
pub type CONN_RX_WR_PTR_STORE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Connection receive FIFO stored write pointer for pointer restore"]
    #[inline(always)]
    pub fn conn_rx_wr_ptr_store(&self) -> CONN_RX_WR_PTR_STORE_R {
        CONN_RX_WR_PTR_STORE_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "LL debug register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_3](index.html) module"]
pub struct LL_DBG_3_SPEC;
impl crate::RegisterSpec for LL_DBG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_3::R](R) reader structure"]
impl crate::Readable for LL_DBG_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_3 to value 0"]
impl crate::Resettable for LL_DBG_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
