#[doc = "Reader of register INTR_S_MASKED"]
pub type R = crate::R<u32, super::INTR_S_MASKED>;
#[doc = "Reader of field `I2C_ARB_LOST`"]
pub type I2C_ARB_LOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_NACK`"]
pub type I2C_NACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_ACK`"]
pub type I2C_ACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_WRITE_STOP`"]
pub type I2C_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_STOP`"]
pub type I2C_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_START`"]
pub type I2C_START_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_ADDR_MATCH`"]
pub type I2C_ADDR_MATCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_GENERAL`"]
pub type I2C_GENERAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_BUS_ERROR`"]
pub type I2C_BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_EZ_WRITE_STOP`"]
pub type SPI_EZ_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_EZ_STOP`"]
pub type SPI_EZ_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_BUS_ERROR`"]
pub type SPI_BUS_ERROR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
