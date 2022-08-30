#[doc = "Register `I2C_STATUS` reader"]
pub struct R(crate::R<I2C_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUS_BUSY` reader - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the SCB is disabled, BUS_BUSY is '0'. After enabling the SCB, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
pub type BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EC_BUSY` reader - Inidicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ and CMD_RESP mode). This bit can be used by the CPU to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
pub type I2C_EC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `S_READ` reader - N/A"]
pub type S_READ_R = crate::BitReader<bool>;
#[doc = "Field `M_READ` reader - N/A"]
pub type M_READ_R = crate::BitReader<bool>;
#[doc = "Field `CURR_EZ_ADDR` reader - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1')."]
pub type CURR_EZ_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASE_EZ_ADDR` reader - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable."]
pub type BASE_EZ_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the SCB is disabled, BUS_BUSY is '0'. After enabling the SCB, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ and CMD_RESP mode). This bit can be used by the CPU to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
    #[inline(always)]
    pub fn i2c_ec_busy(&self) -> I2C_EC_BUSY_R {
        I2C_EC_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn s_read(&self) -> S_READ_R {
        S_READ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn m_read(&self) -> M_READ_R {
        M_READ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1')."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "I2C status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status](index.html) module"]
pub struct I2C_STATUS_SPEC;
impl crate::RegisterSpec for I2C_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_status::R](R) reader structure"]
impl crate::Readable for I2C_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_STATUS to value 0"]
impl crate::Resettable for I2C_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
