#[doc = "Reader of register INTR_CAUSE"]
pub type R = crate::R<u32, super::INTR_CAUSE>;
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<bool, bool>;
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C_EC`"]
pub type I2C_EC_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_EC`"]
pub type SPI_EC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Master interrupt active ('interrupt_master'): INTR_M_MASKED != 0."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave interrupt active ('interrupt_slave'): INTR_S_MASKED != 0."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter interrupt active ('interrupt_tx'): INTR_TX_MASKED != 0."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver interrupt active ('interrupt_rx'): INTR_RX_MASKED != 0."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Externally clock I2C interrupt active ('interrupt_i2c_ec'): INTR_I2C_EC_MASKED != 0."]
    #[inline(always)]
    pub fn i2c_ec(&self) -> I2C_EC_R {
        I2C_EC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Externally clocked SPI interrupt active ('interrupt_spi_ec'): INTR_SPI_EC_MASKED != 0."]
    #[inline(always)]
    pub fn spi_ec(&self) -> SPI_EC_R {
        SPI_EC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
