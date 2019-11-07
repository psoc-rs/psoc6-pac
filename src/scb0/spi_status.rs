#[doc = "Reader of register SPI_STATUS"]
pub type R = crate::R<u32, super::SPI_STATUS>;
#[doc = "Reader of field `BUS_BUSY`"]
pub type BUS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI_EC_BUSY`"]
pub type SPI_EC_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURR_EZ_ADDR`"]
pub type CURR_EZ_ADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `BASE_EZ_ADDR`"]
pub type BASE_EZ_ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub fn spi_ec_busy(&self) -> SPI_EC_BUSY_R {
        SPI_EC_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
