#[doc = "Register `LL_DBG_7` reader"]
pub struct R(crate::R<LL_DBG_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADV_RX_WR_PTR` reader - Advertiser Receive FIFO write pointer"]
pub type ADV_RX_WR_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADV_RX_RD_PTR` reader - Advertiser Receive FIFO read pointer"]
pub type ADV_RX_RD_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Advertiser Receive FIFO write pointer"]
    #[inline(always)]
    pub fn adv_rx_wr_ptr(&self) -> ADV_RX_WR_PTR_R {
        ADV_RX_WR_PTR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Advertiser Receive FIFO read pointer"]
    #[inline(always)]
    pub fn adv_rx_rd_ptr(&self) -> ADV_RX_RD_PTR_R {
        ADV_RX_RD_PTR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[doc = "LL debug register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_7](index.html) module"]
pub struct LL_DBG_7_SPEC;
impl crate::RegisterSpec for LL_DBG_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_7::R](R) reader structure"]
impl crate::Readable for LL_DBG_7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_7 to value 0"]
impl crate::Resettable for LL_DBG_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
