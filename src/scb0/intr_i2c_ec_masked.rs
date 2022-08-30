#[doc = "Register `INTR_I2C_EC_MASKED` reader"]
pub struct R(crate::R<INTR_I2C_EC_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_I2C_EC_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_I2C_EC_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_I2C_EC_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKE_UP` reader - Logical and of corresponding request and mask bits."]
pub type WAKE_UP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `EZ_READ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EZ_READ_STOP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Externally clocked I2C interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_i2c_ec_masked](index.html) module"]
pub struct INTR_I2C_EC_MASKED_SPEC;
impl crate::RegisterSpec for INTR_I2C_EC_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_i2c_ec_masked::R](R) reader structure"]
impl crate::Readable for INTR_I2C_EC_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_I2C_EC_MASKED to value 0"]
impl crate::Resettable for INTR_I2C_EC_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
