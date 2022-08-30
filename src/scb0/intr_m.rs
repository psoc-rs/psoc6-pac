#[doc = "Register `INTR_M` reader"]
pub struct R(crate::R<INTR_M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_M` writer"]
pub struct W(crate::W<INTR_M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_M_SPEC>;
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
impl From<crate::W<INTR_M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - N/A"]
pub type I2C_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ARB_LOST` writer - N/A"]
pub type I2C_ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
#[doc = "Field `I2C_NACK` reader - N/A"]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - N/A"]
pub type I2C_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
#[doc = "Field `I2C_ACK` reader - N/A"]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - N/A"]
pub type I2C_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
#[doc = "Field `I2C_STOP` reader - N/A"]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - N/A"]
pub type I2C_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
#[doc = "Field `I2C_BUS_ERROR` reader - N/A"]
pub type I2C_BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERROR` writer - N/A"]
pub type I2C_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
#[doc = "Field `SPI_DONE` reader - N/A"]
pub type SPI_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DONE` writer - N/A"]
pub type SPI_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_M_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2C_BUS_ERROR_R {
        I2C_BUS_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn spi_done(&self) -> SPI_DONE_R {
        SPI_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W<0> {
        I2C_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W<1> {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W<2> {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W<4> {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn i2c_bus_error(&mut self) -> I2C_BUS_ERROR_W<8> {
        I2C_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn spi_done(&mut self) -> SPI_DONE_W<9> {
        SPI_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_m](index.html) module"]
pub struct INTR_M_SPEC;
impl crate::RegisterSpec for INTR_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_m::R](R) reader structure"]
impl crate::Readable for INTR_M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_m::W](W) writer structure"]
impl crate::Writable for INTR_M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_M to value 0"]
impl crate::Resettable for INTR_M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
