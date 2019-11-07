#[doc = "Reader of register INTR_I2C_EC_MASKED"]
pub type R = crate::R<u32, super::INTR_I2C_EC_MASKED>;
#[doc = "Reader of field `WAKE_UP`"]
pub type WAKE_UP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EZ_STOP`"]
pub type EZ_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EZ_WRITE_STOP`"]
pub type EZ_WRITE_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EZ_READ_STOP`"]
pub type EZ_READ_STOP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EZ_STOP_R {
        EZ_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EZ_WRITE_STOP_R {
        EZ_WRITE_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EZ_READ_STOP_R {
        EZ_READ_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
