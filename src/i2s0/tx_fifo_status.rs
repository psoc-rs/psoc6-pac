#[doc = "Register `TX_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
pub type USED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RD_PTR` reader - TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
pub type RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_PTR` reader - TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
pub type WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:8 - Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "TX FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_status](index.html) module"]
pub struct TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
