#[doc = "Reader of register I2C_M_CMD"]
pub type R = crate::R<u32, super::I2C_M_CMD>;
#[doc = "Writer for register I2C_M_CMD"]
pub type W = crate::W<u32, super::I2C_M_CMD>;
#[doc = "Register I2C_M_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_M_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M_START`"]
pub type M_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_START`"]
pub struct M_START_W<'a> {
    w: &'a mut W,
}
impl<'a> M_START_W<'a> {
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
#[doc = "Reader of field `M_START_ON_IDLE`"]
pub type M_START_ON_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_START_ON_IDLE`"]
pub struct M_START_ON_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> M_START_ON_IDLE_W<'a> {
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
#[doc = "Reader of field `M_ACK`"]
pub type M_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_ACK`"]
pub struct M_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_ACK_W<'a> {
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
#[doc = "Reader of field `M_NACK`"]
pub type M_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_NACK`"]
pub struct M_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_NACK_W<'a> {
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
#[doc = "Reader of field `M_STOP`"]
pub type M_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_STOP`"]
pub struct M_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> M_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&self) -> M_START_R {
        M_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&self) -> M_START_ON_IDLE_R {
        M_START_ON_IDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_ack(&self) -> M_ACK_R {
        M_ACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_nack(&self) -> M_NACK_R {
        M_NACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    pub fn m_stop(&self) -> M_STOP_R {
        M_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&mut self) -> M_START_W {
        M_START_W { w: self }
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&mut self) -> M_START_ON_IDLE_W {
        M_START_ON_IDLE_W { w: self }
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_ack(&mut self) -> M_ACK_W {
        M_ACK_W { w: self }
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_nack(&mut self) -> M_NACK_W {
        M_NACK_W { w: self }
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    pub fn m_stop(&mut self) -> M_STOP_W {
        M_STOP_W { w: self }
    }
}
