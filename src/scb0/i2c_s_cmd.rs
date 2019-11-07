#[doc = "Reader of register I2C_S_CMD"]
pub type R = crate::R<u32, super::I2C_S_CMD>;
#[doc = "Writer for register I2C_S_CMD"]
pub type W = crate::W<u32, super::I2C_S_CMD>;
#[doc = "Register I2C_S_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_S_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S_ACK`"]
pub type S_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_ACK`"]
pub struct S_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_ACK_W<'a> {
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
#[doc = "Reader of field `S_NACK`"]
pub type S_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_NACK`"]
pub struct S_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NACK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    pub fn s_ack(&self) -> S_ACK_R {
        S_ACK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&self) -> S_NACK_R {
        S_NACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    pub fn s_ack(&mut self) -> S_ACK_W {
        S_ACK_W { w: self }
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&mut self) -> S_NACK_W {
        S_NACK_W { w: self }
    }
}
