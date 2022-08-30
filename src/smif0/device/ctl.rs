#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_EN` reader - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_R = crate::BitReader<bool>;
#[doc = "Field `WR_EN` writer - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CRYPTO_EN` reader - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_EN` writer - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DATA_SEL` reader - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_SEL` writer - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENABLED` reader - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&self) -> CRYPTO_EN_R {
        CRYPTO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W<0> {
        WR_EN_W::new(self)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&mut self) -> CRYPTO_EN_W<8> {
        CRYPTO_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&mut self) -> DATA_SEL_W<16> {
        DATA_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
