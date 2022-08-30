#[doc = "Register `I2C_M_CMD` reader"]
pub struct R(crate::R<I2C_M_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_M_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_M_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_M_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_M_CMD` writer"]
pub struct W(crate::W<I2C_M_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_M_CMD_SPEC>;
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
impl From<crate::W<I2C_M_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_M_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_START` reader - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_R = crate::BitReader<bool>;
#[doc = "Field `M_START` writer - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_M_CMD_SPEC, bool, O>;
#[doc = "Field `M_START_ON_IDLE` reader - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_ON_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `M_START_ON_IDLE` writer - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_ON_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_M_CMD_SPEC, bool, O>;
#[doc = "Field `M_ACK` reader - N/A"]
pub type M_ACK_R = crate::BitReader<bool>;
#[doc = "Field `M_ACK` writer - N/A"]
pub type M_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_M_CMD_SPEC, bool, O>;
#[doc = "Field `M_NACK` reader - N/A"]
pub type M_NACK_R = crate::BitReader<bool>;
#[doc = "Field `M_NACK` writer - N/A"]
pub type M_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_M_CMD_SPEC, bool, O>;
#[doc = "Field `M_STOP` reader - N/A"]
pub type M_STOP_R = crate::BitReader<bool>;
#[doc = "Field `M_STOP` writer - N/A"]
pub type M_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_M_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&self) -> M_START_R {
        M_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&self) -> M_START_ON_IDLE_R {
        M_START_ON_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn m_ack(&self) -> M_ACK_R {
        M_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn m_nack(&self) -> M_NACK_R {
        M_NACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn m_stop(&self) -> M_STOP_R {
        M_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&mut self) -> M_START_W<0> {
        M_START_W::new(self)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&mut self) -> M_START_ON_IDLE_W<1> {
        M_START_ON_IDLE_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn m_ack(&mut self) -> M_ACK_W<2> {
        M_ACK_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn m_nack(&mut self) -> M_NACK_W<3> {
        M_NACK_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn m_stop(&mut self) -> M_STOP_W<4> {
        M_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C master command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_m_cmd](index.html) module"]
pub struct I2C_M_CMD_SPEC;
impl crate::RegisterSpec for I2C_M_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_m_cmd::R](R) reader structure"]
impl crate::Readable for I2C_M_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_m_cmd::W](W) writer structure"]
impl crate::Writable for I2C_M_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_M_CMD to value 0"]
impl crate::Resettable for I2C_M_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
