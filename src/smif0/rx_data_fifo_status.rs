#[doc = "Register `RX_DATA_FIFO_STATUS` reader"]
pub struct R(crate::R<RX_DATA_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED4` reader - Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
pub type USED4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn used4(&self) -> USED4_R {
        USED4_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Receiver data FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_status](index.html) module"]
pub struct RX_DATA_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data_fifo_status::R](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_DATA_FIFO_STATUS to value 0"]
impl crate::Resettable for RX_DATA_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
