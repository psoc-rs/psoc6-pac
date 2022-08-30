#[doc = "Register `INTR_S_MASK` reader"]
pub struct R(crate::R<INTR_S_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_S_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_S_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_S_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_S_MASK` writer"]
pub struct W(crate::W<INTR_S_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_S_MASK_SPEC>;
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
impl From<crate::W<INTR_S_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_S_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ARB_LOST` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_NACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_ACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_WRITE_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_START` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_START_R = crate::BitReader<bool>;
#[doc = "Field `I2C_START` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_ADDR_MATCH` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ADDR_MATCH` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_ADDR_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_GENERAL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_GENERAL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_GENERAL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_GENERAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `I2C_BUS_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2C_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_EZ_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_EZ_WRITE_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `SPI_EZ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_EZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SPI_EZ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_EZ_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
#[doc = "Field `SPI_BUS_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BUS_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SPI_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_S_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SPI_EZ_WRITE_STOP_R {
        SPI_EZ_WRITE_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SPI_EZ_STOP_R {
        SPI_EZ_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SPI_BUS_ERROR_R {
        SPI_BUS_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W<0> {
        I2C_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W<1> {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W<2> {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W<3> {
        I2C_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W<4> {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W<5> {
        I2C_START_W::new(self)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W<6> {
        I2C_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W<7> {
        I2C_GENERAL_W::new(self)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W<8> {
        I2C_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&mut self) -> SPI_EZ_WRITE_STOP_W<9> {
        SPI_EZ_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_stop(&mut self) -> SPI_EZ_STOP_W<10> {
        SPI_EZ_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_bus_error(&mut self) -> SPI_BUS_ERROR_W<11> {
        SPI_BUS_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_s_mask](index.html) module"]
pub struct INTR_S_MASK_SPEC;
impl crate::RegisterSpec for INTR_S_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_s_mask::R](R) reader structure"]
impl crate::Readable for INTR_S_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_s_mask::W](W) writer structure"]
impl crate::Writable for INTR_S_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_S_MASK to value 0"]
impl crate::Resettable for INTR_S_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
