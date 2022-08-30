#[doc = "Register `I2C_S_CMD` reader"]
pub struct R(crate::R<I2C_S_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_S_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_S_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_S_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_S_CMD` writer"]
pub struct W(crate::W<I2C_S_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_S_CMD_SPEC>;
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
impl From<crate::W<I2C_S_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_S_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S_ACK` reader - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
pub type S_ACK_R = crate::BitReader<bool>;
#[doc = "Field `S_ACK` writer - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
pub type S_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_S_CMD_SPEC, bool, O>;
#[doc = "Field `S_NACK` reader - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type S_NACK_R = crate::BitReader<bool>;
#[doc = "Field `S_NACK` writer - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type S_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_S_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
    #[inline(always)]
    pub fn s_ack(&self) -> S_ACK_R {
        S_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&self) -> S_NACK_R {
        S_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode)."]
    #[inline(always)]
    pub fn s_ack(&mut self) -> S_ACK_W<0> {
        S_ACK_W::new(self)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ and CMD_RESP mode, this field should be set to '0' (it is only to be used in FIFO mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&mut self) -> S_NACK_W<1> {
        S_NACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C slave command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_s_cmd](index.html) module"]
pub struct I2C_S_CMD_SPEC;
impl crate::RegisterSpec for I2C_S_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_s_cmd::R](R) reader structure"]
impl crate::Readable for I2C_S_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_s_cmd::W](W) writer structure"]
impl crate::Writable for I2C_S_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_S_CMD to value 0"]
impl crate::Resettable for I2C_S_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
