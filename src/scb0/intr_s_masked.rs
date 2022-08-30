#[doc = "Register `INTR_S_MASKED` reader"]
pub struct R(crate::R<INTR_S_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_S_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_S_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_S_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` reader - Logical and of corresponding request and mask bits."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2C_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_START` reader - Logical and of corresponding request and mask bits."]
pub type I2C_START_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ADDR_MATCH` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_GENERAL` reader - Logical and of corresponding request and mask bits."]
pub type I2C_GENERAL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type I2C_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type SPI_EZ_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SPI_EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type SPI_EZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type SPI_BUS_ERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Slave interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_s_masked](index.html) module"]
pub struct INTR_S_MASKED_SPEC;
impl crate::RegisterSpec for INTR_S_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_s_masked::R](R) reader structure"]
impl crate::Readable for INTR_S_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_S_MASKED to value 0"]
impl crate::Resettable for INTR_S_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
