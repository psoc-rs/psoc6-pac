#[doc = "Register `RX_DATA_FIFO_RD1` reader"]
pub struct R(crate::R<RX_DATA_FIFO_RD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_FIFO_RD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_FIFO_RD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_FIFO_RD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO)."]
pub type DATA0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd1](index.html) module"]
pub struct RX_DATA_FIFO_RD1_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_RD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data_fifo_rd1::R](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD1 to value 0"]
impl crate::Resettable for RX_DATA_FIFO_RD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
