#[doc = "Reader of register I2C_CTRL"]
pub type R = crate::R<u32, super::I2C_CTRL>;
#[doc = "Writer for register I2C_CTRL"]
pub type W = crate::W<u32, super::I2C_CTRL>;
#[doc = "Register I2C_CTRL `reset()`'s with value 0xfb88"]
impl crate::ResetValue for super::I2C_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfb88
    }
}
#[doc = "Reader of field `HIGH_PHASE_OVS`"]
pub type HIGH_PHASE_OVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIGH_PHASE_OVS`"]
pub struct HIGH_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LOW_PHASE_OVS`"]
pub type LOW_PHASE_OVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOW_PHASE_OVS`"]
pub struct LOW_PHASE_OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_PHASE_OVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `M_READY_DATA_ACK`"]
pub type M_READY_DATA_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_READY_DATA_ACK`"]
pub struct M_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_READY_DATA_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `M_NOT_READY_DATA_NACK`"]
pub type M_NOT_READY_DATA_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M_NOT_READY_DATA_NACK`"]
pub struct M_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> M_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `S_GENERAL_IGNORE`"]
pub type S_GENERAL_IGNORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_GENERAL_IGNORE`"]
pub struct S_GENERAL_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> S_GENERAL_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `S_READY_ADDR_ACK`"]
pub type S_READY_ADDR_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_READY_ADDR_ACK`"]
pub struct S_READY_ADDR_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_ADDR_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `S_READY_DATA_ACK`"]
pub type S_READY_DATA_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_READY_DATA_ACK`"]
pub struct S_READY_DATA_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_READY_DATA_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `S_NOT_READY_ADDR_NACK`"]
pub type S_NOT_READY_ADDR_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_NOT_READY_ADDR_NACK`"]
pub struct S_NOT_READY_ADDR_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_ADDR_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `S_NOT_READY_DATA_NACK`"]
pub type S_NOT_READY_DATA_NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S_NOT_READY_DATA_NACK`"]
pub struct S_NOT_READY_DATA_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> S_NOT_READY_DATA_NACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_MODE`"]
pub type SLAVE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_MODE`"]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MASTER_MODE`"]
pub type MASTER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER_MODE`"]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 SCB clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\] with input signal median filtering and \\[4, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the SCB clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 SCB clock cycles and <= 16 SCB clock cycles. Without input signal median filtering, the IF high time should be >= 5 SCB clock cycles and <= 16 SCB clock cycles."]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HIGH_PHASE_OVS_R {
        HIGH_PHASE_OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 SCB clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\] with input signal median filtering and \\[6, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the SCB clock wrt. the regular (no stretching) interface (IF) low time to guarantee functionally correct behavior. With input signal median filtering, the IF low time should be >= 8 SCB clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 SCB clock cycles and <= 16 SCB clock cycles."]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LOW_PHASE_OVS_R {
        LOW_PHASE_OVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> M_READY_DATA_ACK_R {
        M_READY_DATA_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> M_NOT_READY_DATA_NACK_R {
        M_NOT_READY_DATA_NACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> S_GENERAL_IGNORE_R {
        S_GENERAL_IGNORE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> S_READY_ADDR_ACK_R {
        S_READY_ADDR_ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> S_READY_DATA_ACK_R {
        S_READY_DATA_ACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This field is used during an address match or general call address in internally clocked mode Only used when: - EC_AM_MODE is '0', EC_OP_MODE is '0', S_GENERAL_IGNORE is '0\\] and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the SCB clock is available (in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2).SCB clock is not present (in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the SCB clock is available). The logic will handle the ongoing transfer as soon as the clock is enabled."]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> S_NOT_READY_ADDR_NACK_R {
        S_NOT_READY_ADDR_NACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Only used when: - non EZ mode Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> S_NOT_READY_DATA_NACK_R {
        S_NOT_READY_DATA_NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 SCB clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\] with input signal median filtering and \\[4, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the SCB clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 SCB clock cycles and <= 16 SCB clock cycles. Without input signal median filtering, the IF high time should be >= 5 SCB clock cycles and <= 16 SCB clock cycles."]
    #[inline(always)]
    pub fn high_phase_ovs(&mut self) -> HIGH_PHASE_OVS_W {
        HIGH_PHASE_OVS_W { w: self }
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 SCB clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\] with input signal median filtering and \\[6, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the SCB clock wrt. the regular (no stretching) interface (IF) low time to guarantee functionally correct behavior. With input signal median filtering, the IF low time should be >= 8 SCB clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 SCB clock cycles and <= 16 SCB clock cycles."]
    #[inline(always)]
    pub fn low_phase_ovs(&mut self) -> LOW_PHASE_OVS_W {
        LOW_PHASE_OVS_W { w: self }
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn m_ready_data_ack(&mut self) -> M_READY_DATA_ACK_W {
        M_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&mut self) -> M_NOT_READY_DATA_NACK_W {
        M_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn s_general_ignore(&mut self) -> S_GENERAL_IGNORE_W {
        S_GENERAL_IGNORE_W { w: self }
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&mut self) -> S_READY_ADDR_ACK_W {
        S_READY_ADDR_ACK_W { w: self }
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn s_ready_data_ack(&mut self) -> S_READY_DATA_ACK_W {
        S_READY_DATA_ACK_W { w: self }
    }
    #[doc = "Bit 14 - This field is used during an address match or general call address in internally clocked mode Only used when: - EC_AM_MODE is '0', EC_OP_MODE is '0', S_GENERAL_IGNORE is '0\\] and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the SCB clock is available (in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2).SCB clock is not present (in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the SCB clock is available). The logic will handle the ongoing transfer as soon as the clock is enabled."]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&mut self) -> S_NOT_READY_ADDR_NACK_W {
        S_NOT_READY_ADDR_NACK_W { w: self }
    }
    #[doc = "Bit 15 - Only used when: - non EZ mode Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&mut self) -> S_NOT_READY_DATA_NACK_W {
        S_NOT_READY_DATA_NACK_W { w: self }
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
}
