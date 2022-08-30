#[doc = "Register `RX_FIFO_RD_SILENT` reader"]
pub struct R(crate::R<RX_FIFO_RD_SILENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_RD_SILENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_RD_SILENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_RD_SILENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - N/A"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver FIFO read silent\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd_silent](index.html) module"]
pub struct RX_FIFO_RD_SILENT_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SILENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_rd_silent::R](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_RD_SILENT to value 0"]
impl crate::Resettable for RX_FIFO_RD_SILENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
