#[doc = "Reader of register SPI_CTRL"]
pub type R = crate::R<u32, super::SPI_CTRL>;
#[doc = "Writer for register SPI_CTRL"]
pub type W = crate::W<u32, super::SPI_CTRL>;
#[doc = "Register SPI_CTRL `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::SPI_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_0000
    }
}
#[doc = "Reader of field `SSEL_CONTINUOUS`"]
pub type SSEL_CONTINUOUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_CONTINUOUS`"]
pub struct SSEL_CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_CONTINUOUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SELECT_PRECEDE`"]
pub type SELECT_PRECEDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELECT_PRECEDE`"]
pub struct SELECT_PRECEDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECT_PRECEDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LATE_MISO_SAMPLE`"]
pub type LATE_MISO_SAMPLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATE_MISO_SAMPLE`"]
pub struct LATE_MISO_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LATE_MISO_SAMPLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCLK_CONTINUOUS`"]
pub type SCLK_CONTINUOUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_CONTINUOUS`"]
pub struct SCLK_CONTINUOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_CONTINUOUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SSEL_POLARITY0`"]
pub type SSEL_POLARITY0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_POLARITY0`"]
pub struct SSEL_POLARITY0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SSEL_POLARITY1`"]
pub type SSEL_POLARITY1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_POLARITY1`"]
pub struct SSEL_POLARITY1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SSEL_POLARITY2`"]
pub type SSEL_POLARITY2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_POLARITY2`"]
pub struct SSEL_POLARITY2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SSEL_POLARITY3`"]
pub type SSEL_POLARITY3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEL_POLARITY3`"]
pub struct SSEL_POLARITY3_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_POLARITY3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPI Motorola submode. In master mode, when not transmitting data (Slave SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive."]
    SPI_MOTOROLA = 0,
    #[doc = "1: SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive; i.e. no pulse is generated."]
    SPI_TI = 1,
    #[doc = "2: SPI National Semiconducturs submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive."]
    SPI_NS = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::SPI_MOTOROLA),
            1 => Val(MODE_A::SPI_TI),
            2 => Val(MODE_A::SPI_NS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MOTOROLA`"]
    #[inline(always)]
    pub fn is_spi_motorola(&self) -> bool {
        *self == MODE_A::SPI_MOTOROLA
    }
    #[doc = "Checks if the value of the field is `SPI_TI`"]
    #[inline(always)]
    pub fn is_spi_ti(&self) -> bool {
        *self == MODE_A::SPI_TI
    }
    #[doc = "Checks if the value of the field is `SPI_NS`"]
    #[inline(always)]
    pub fn is_spi_ns(&self) -> bool {
        *self == MODE_A::SPI_NS
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Motorola submode. In master mode, when not transmitting data (Slave SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive."]
    #[inline(always)]
    pub fn spi_motorola(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MOTOROLA)
    }
    #[doc = "SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive; i.e. no pulse is generated."]
    #[inline(always)]
    pub fn spi_ti(self) -> &'a mut W {
        self.variant(MODE_A::SPI_TI)
    }
    #[doc = "SPI National Semiconducturs submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), Slave SELECT is inactive."]
    #[inline(always)]
    pub fn spi_ns(self) -> &'a mut W {
        self.variant(MODE_A::SPI_NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SSEL`"]
pub type SSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSEL`"]
pub struct SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MASTER_MODE`"]
pub type MASTER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_MODE`"]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are sent out with slave deselection."]
    #[inline(always)]
    pub fn ssel_continuous(&self) -> SSEL_CONTINUOUS_R {
        SSEL_CONTINUOUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&self) -> SELECT_PRECEDE_R {
        SELECT_PRECEDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&self) -> LATE_MISO_SAMPLE_R {
        LATE_MISO_SAMPLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity0(&self) -> SSEL_POLARITY0_R {
        SSEL_POLARITY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity1(&self) -> SSEL_POLARITY1_R {
        SSEL_POLARITY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity2(&self) -> SSEL_POLARITY2_R {
        SSEL_POLARITY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity3(&self) -> SSEL_POLARITY3_R {
        SSEL_POLARITY3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. SCB block should be disabled when changes are made to this field."]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are sent out with slave deselection."]
    #[inline(always)]
    pub fn ssel_continuous(&mut self) -> SSEL_CONTINUOUS_W {
        SSEL_CONTINUOUS_W { w: self }
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the Slave SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the Slave SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&mut self) -> SELECT_PRECEDE_W {
        SELECT_PRECEDE_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies ( for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&mut self) -> LATE_MISO_SAMPLE_W {
        LATE_MISO_SAMPLE_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W {
        SCLK_CONTINUOUS_W { w: self }
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity0(&mut self) -> SSEL_POLARITY0_W {
        SSEL_POLARITY0_W { w: self }
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity1(&mut self) -> SSEL_POLARITY1_W {
        SSEL_POLARITY1_W { w: self }
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity2(&mut self) -> SSEL_POLARITY2_W {
        SSEL_POLARITY2_W { w: self }
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity3(&mut self) -> SSEL_POLARITY3_W {
        SSEL_POLARITY3_W { w: self }
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': No local loopback '1': the SPI master MISO line is connected to the SPI master MOSI line. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. SCB block should be disabled when changes are made to this field."]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W { w: self }
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
}
