#[doc = "Register `INTR_I2C_EC_MASK` reader"]
pub struct R(crate::R<INTR_I2C_EC_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_I2C_EC_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_I2C_EC_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_I2C_EC_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_I2C_EC_MASK` writer"]
pub struct W(crate::W<INTR_I2C_EC_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_I2C_EC_MASK_SPEC>;
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
impl From<crate::W<INTR_I2C_EC_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_I2C_EC_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_UP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type WAKE_UP_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_UP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type WAKE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_MASK_SPEC, bool, O>;
#[doc = "Field `EZ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_I2C_EC_MASK_SPEC, bool, O>;
#[doc = "Field `EZ_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_WRITE_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_I2C_EC_MASK_SPEC, bool, O>;
#[doc = "Field `EZ_READ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_READ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_READ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EZ_READ_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_I2C_EC_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W<0> {
        WAKE_UP_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_stop(&mut self) -> EZ_STOP_W<1> {
        EZ_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_write_stop(&mut self) -> EZ_WRITE_STOP_W<2> {
        EZ_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
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
#[doc = "Externally clocked I2C interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_i2c_ec_mask](index.html) module"]
pub struct INTR_I2C_EC_MASK_SPEC;
impl crate::RegisterSpec for INTR_I2C_EC_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_i2c_ec_mask::R](R) reader structure"]
impl crate::Readable for INTR_I2C_EC_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_i2c_ec_mask::W](W) writer structure"]
impl crate::Writable for INTR_I2C_EC_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_I2C_EC_MASK to value 0"]
impl crate::Resettable for INTR_I2C_EC_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
