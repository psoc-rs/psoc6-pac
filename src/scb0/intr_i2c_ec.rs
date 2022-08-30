#[doc = "Register `INTR_I2C_EC` reader"]
pub struct R(crate::R<INTR_I2C_EC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_I2C_EC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_I2C_EC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_I2C_EC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_I2C_EC` writer"]
pub struct W(crate::W<INTR_I2C_EC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_I2C_EC_SPEC>;
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
impl From<crate::W<INTR_I2C_EC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_I2C_EC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_UP` reader - Wake up request. Active on incoming slave request (with address match). Only set when EC_AM is '1'."]
pub type WAKE_UP_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_UP` writer - Wake up request. Active on incoming slave request (with address match). Only set when EC_AM is '1'."]
pub type WAKE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_SPEC, bool, O>;
#[doc = "Field `EZ_STOP` reader - STOP detection. Activated on the end of a every transfer (I2C STOP). Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_STOP` writer - STOP detection. Activated on the end of a every transfer (I2C STOP). Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_SPEC, bool, O>;
#[doc = "Field `EZ_WRITE_STOP` reader - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_WRITE_STOP` writer - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_WRITE_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_SPEC, bool, O>;
#[doc = "Field `EZ_READ_STOP` reader - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_READ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_READ_STOP` writer - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EZ_READ_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only set when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only set when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W<0> {
        WAKE_UP_W::new(self)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&mut self) -> EZ_STOP_W<1> {
        EZ_STOP_W::new(self)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&mut self) -> EZ_WRITE_STOP_W<2> {
        EZ_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only set for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&mut self) -> EZ_READ_STOP_W<3> {
        EZ_READ_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Externally clocked I2C interrupt request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_i2c_ec](index.html) module"]
pub struct INTR_I2C_EC_SPEC;
impl crate::RegisterSpec for INTR_I2C_EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_i2c_ec::R](R) reader structure"]
impl crate::Readable for INTR_I2C_EC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_i2c_ec::W](W) writer structure"]
impl crate::Writable for INTR_I2C_EC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_I2C_EC to value 0"]
impl crate::Resettable for INTR_I2C_EC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
