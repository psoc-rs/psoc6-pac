#[doc = "Register `TX_CMD_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_CMD_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CMD_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CMD_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CMD_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED3` reader - Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
pub type USED3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
    #[inline(always)]
    pub fn used3(&self) -> USED3_R {
        USED3_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Transmitter command FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cmd_fifo_status](index.html) module"]
pub struct TX_CMD_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_CMD_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_cmd_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_CMD_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CMD_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_CMD_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
