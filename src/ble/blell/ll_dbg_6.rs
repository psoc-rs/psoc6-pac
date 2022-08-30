#[doc = "Register `LL_DBG_6` reader"]
pub struct R(crate::R<LL_DBG_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_DBG_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_DBG_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_DBG_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADV_TX_WR_PTR` reader - Advertiser Transmit FIFO write pointer"]
pub type ADV_TX_WR_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCAN_RSP_TX_WR_PTR` reader - Scan Response Transmit FIFO write pointer"]
pub type SCAN_RSP_TX_WR_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADV_TX_RD_PTR` reader - Advertiser/ Scan Response FIFO read pointer"]
pub type ADV_TX_RD_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Advertiser Transmit FIFO write pointer"]
    #[inline(always)]
    pub fn adv_tx_wr_ptr(&self) -> ADV_TX_WR_PTR_R {
        ADV_TX_WR_PTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Scan Response Transmit FIFO write pointer"]
    #[inline(always)]
    pub fn scan_rsp_tx_wr_ptr(&self) -> SCAN_RSP_TX_WR_PTR_R {
        SCAN_RSP_TX_WR_PTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Advertiser/ Scan Response FIFO read pointer"]
    #[inline(always)]
    pub fn adv_tx_rd_ptr(&self) -> ADV_TX_RD_PTR_R {
        ADV_TX_RD_PTR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[doc = "LL debug register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_dbg_6](index.html) module"]
pub struct LL_DBG_6_SPEC;
impl crate::RegisterSpec for LL_DBG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_dbg_6::R](R) reader structure"]
impl crate::Readable for LL_DBG_6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_DBG_6 to value 0"]
impl crate::Resettable for LL_DBG_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
