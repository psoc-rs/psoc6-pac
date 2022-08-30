#[doc = "Register `RX_DATA_FIFO_RD4` reader"]
pub struct R(crate::R<RX_DATA_FIFO_RD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_FIFO_RD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_FIFO_RD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_FIFO_RD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO, first byte)."]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` reader - RX data (read from RX data FIFO, second byte)."]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA2` reader - RX data (read from RX data FIFO, third byte)."]
pub type DATA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA3` reader - RX data (read from RX data FIFO, fourth byte)."]
pub type DATA3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX data (read from RX data FIFO, third byte)."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX data (read from RX data FIFO, fourth byte)."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_rd4](index.html) module"]
pub struct RX_DATA_FIFO_RD4_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_RD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data_fifo_rd4::R](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_RD4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD4 to value 0"]
impl crate::Resettable for RX_DATA_FIFO_RD4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
