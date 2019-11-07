#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WR_EN`"]
pub type WR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_EN`"]
pub struct WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_EN_W<'a> {
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
#[doc = "Reader of field `CRYPTO_EN`"]
pub type CRYPTO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_EN`"]
pub struct CRYPTO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_EN_W<'a> {
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
#[doc = "Reader of field `DATA_SEL`"]
pub type DATA_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_SEL`"]
pub struct DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&self) -> CRYPTO_EN_R {
        CRYPTO_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\] = IO0, spi_data\\[1\\] = IO1, ..., spi_data\\[7\\] = IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\] = IO0, spi_data\\[3\\] = IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\] = IO0, spi_data\\[5\\] = IO1, ..., spi_data\\[7\\] = IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\] = IO0, spi_data\\[7\\] = IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W {
        WR_EN_W { w: self }
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&mut self) -> CRYPTO_EN_W {
        CRYPTO_EN_W { w: self }
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\] = IO0, spi_data\\[1\\] = IO1, ..., spi_data\\[7\\] = IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\] = IO0, spi_data\\[3\\] = IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\] = IO0, spi_data\\[5\\] = IO1, ..., spi_data\\[7\\] = IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\] = IO0, spi_data\\[7\\] = IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&mut self) -> DATA_SEL_W {
        DATA_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
