#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REG_ENABLE`"]
pub type REG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG_ENABLE`"]
pub struct REG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ENABLE_W<'a> {
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
#[doc = "Reader of field `ENABLE_LOCK`"]
pub type ENABLE_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_LOCK`"]
pub struct ENABLE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LOCK_W<'a> {
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
#[doc = "Reader of field `BUS_ACTIVITY`"]
pub type BUS_ACTIVITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_ACTIVITY`"]
pub struct BUS_ACTIVITY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ACTIVITY_W<'a> {
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
#[doc = "Reader of field `RSVD_3`"]
pub type RSVD_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_3`"]
pub struct RSVD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn reg_enable(&self) -> REG_ENABLE_R {
        REG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn enable_lock(&self) -> ENABLE_LOCK_R {
        ENABLE_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn bus_activity(&self) -> BUS_ACTIVITY_R {
        BUS_ACTIVITY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> RSVD_3_R {
        RSVD_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn reg_enable(&mut self) -> REG_ENABLE_W {
        REG_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn enable_lock(&mut self) -> ENABLE_LOCK_W {
        ENABLE_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn bus_activity(&mut self) -> BUS_ACTIVITY_W {
        BUS_ACTIVITY_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> RSVD_3_W {
        RSVD_3_W { w: self }
    }
}
