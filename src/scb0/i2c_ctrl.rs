#[doc = "Register `I2C_CTRL` reader"]
pub struct R(crate::R<I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTRL` writer"]
pub struct W(crate::W<I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTRL_SPEC>;
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
impl From<crate::W<I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIGH_PHASE_OVS` reader - Serial I2C interface high phase oversampling factor. (HIGH_PHASE_OVS + 1) * clk_scb constitutes the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
pub type HIGH_PHASE_OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGH_PHASE_OVS` writer - Serial I2C interface high phase oversampling factor. (HIGH_PHASE_OVS + 1) * clk_scb constitutes the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
pub type HIGH_PHASE_OVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOW_PHASE_OVS` reader - Serial I2C interface low phase oversampling factor. (LOW_PHASE_OVS + 1) * clk_scb constitutes the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
pub type LOW_PHASE_OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOW_PHASE_OVS` writer - Serial I2C interface low phase oversampling factor. (LOW_PHASE_OVS + 1) * clk_scb constitutes the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
pub type LOW_PHASE_OVS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `M_READY_DATA_ACK` reader - When '1', a received data element by the master is immediately ACK'd when the RX FIFO is not full. When '0' the CPU is responsible for ACK/NACKing the received data frame using I2C_M_CMD.M_ACK or I2C_M_CMD.M_NACK"]
pub type M_READY_DATA_ACK_R = crate::BitReader<bool>;
#[doc = "Field `M_READY_DATA_ACK` writer - When '1', a received data element by the master is immediately ACK'd when the RX FIFO is not full. When '0' the CPU is responsible for ACK/NACKing the received data frame using I2C_M_CMD.M_ACK or I2C_M_CMD.M_NACK"]
pub type M_READY_DATA_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `M_NOT_READY_DATA_NACK` reader - When '1', a received data element by the master is immediately NACK'd when the RX FIFO is full. When '0', clock stretching is used instead (till the RX FIFO is no longer full)."]
pub type M_NOT_READY_DATA_NACK_R = crate::BitReader<bool>;
#[doc = "Field `M_NOT_READY_DATA_NACK` writer - When '1', a received data element by the master is immediately NACK'd when the RX FIFO is full. When '0', clock stretching is used instead (till the RX FIFO is no longer full)."]
pub type M_NOT_READY_DATA_NACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `S_GENERAL_IGNORE` reader - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure. When '0' the general call address is accepted and follows S_READY_ADDR_ACK and S_NOT_READY_ADDR_NACK"]
pub type S_GENERAL_IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `S_GENERAL_IGNORE` writer - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure. When '0' the general call address is accepted and follows S_READY_ADDR_ACK and S_NOT_READY_ADDR_NACK"]
pub type S_GENERAL_IGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `S_READY_ADDR_ACK` reader - When '1', a received (matching) slave address is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the address must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
pub type S_READY_ADDR_ACK_R = crate::BitReader<bool>;
#[doc = "Field `S_READY_ADDR_ACK` writer - When '1', a received (matching) slave address is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the address must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
pub type S_READY_ADDR_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `S_READY_DATA_ACK` reader - When '1', a received data element by the slave is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the data must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
pub type S_READY_DATA_ACK_R = crate::BitReader<bool>;
#[doc = "Field `S_READY_DATA_ACK` writer - When '1', a received data element by the slave is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the data must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
pub type S_READY_DATA_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `S_NOT_READY_ADDR_NACK` reader - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: In Active/Sleep mode a received (matching) slave address is immediately NACK'd when the RX FIFO is full In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the incoming address will be NACK'd until the clock is avaliable. Once clk_scb is avaliable the address ACK will follow S_READY_ADDR_ACK - 0: in Active/Sleep mode clock stretching is performed when the RX FIFO is full, the strech is released when the RX FIFO is no longer full. In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the clocked will be streched on an incoming address until clk_scb is avaliable. After clk_scb is avalaible the address ACK will follow S_READY_ADDR_ACK"]
pub type S_NOT_READY_ADDR_NACK_R = crate::BitReader<bool>;
#[doc = "Field `S_NOT_READY_ADDR_NACK` writer - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: In Active/Sleep mode a received (matching) slave address is immediately NACK'd when the RX FIFO is full In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the incoming address will be NACK'd until the clock is avaliable. Once clk_scb is avaliable the address ACK will follow S_READY_ADDR_ACK - 0: in Active/Sleep mode clock stretching is performed when the RX FIFO is full, the strech is released when the RX FIFO is no longer full. In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the clocked will be streched on an incoming address until clk_scb is avaliable. After clk_scb is avalaible the address ACK will follow S_READY_ADDR_ACK"]
pub type S_NOT_READY_ADDR_NACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `S_NOT_READY_DATA_NACK` reader - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type S_NOT_READY_DATA_NACK_R = crate::BitReader<bool>;
#[doc = "Field `S_NOT_READY_DATA_NACK` writer - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type S_NOT_READY_DATA_NACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `SLAVE_MODE` reader - N/A"]
pub type SLAVE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_MODE` writer - N/A"]
pub type SLAVE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
#[doc = "Field `MASTER_MODE` reader - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the SCB to address itself."]
pub type MASTER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_MODE` writer - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the SCB to address itself."]
pub type MASTER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2C_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. (HIGH_PHASE_OVS + 1) * clk_scb constitutes the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HIGH_PHASE_OVS_R {
        HIGH_PHASE_OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. (LOW_PHASE_OVS + 1) * clk_scb constitutes the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LOW_PHASE_OVS_R {
        LOW_PHASE_OVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the RX FIFO is not full. When '0' the CPU is responsible for ACK/NACKing the received data frame using I2C_M_CMD.M_ACK or I2C_M_CMD.M_NACK"]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> M_READY_DATA_ACK_R {
        M_READY_DATA_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When '1', a received data element by the master is immediately NACK'd when the RX FIFO is full. When '0', clock stretching is used instead (till the RX FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> M_NOT_READY_DATA_NACK_R {
        M_NOT_READY_DATA_NACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure. When '0' the general call address is accepted and follows S_READY_ADDR_ACK and S_NOT_READY_ADDR_NACK"]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> S_GENERAL_IGNORE_R {
        S_GENERAL_IGNORE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the address must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> S_READY_ADDR_ACK_R {
        S_READY_ADDR_ACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the data must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> S_READY_DATA_ACK_R {
        S_READY_DATA_ACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: In Active/Sleep mode a received (matching) slave address is immediately NACK'd when the RX FIFO is full In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the incoming address will be NACK'd until the clock is avaliable. Once clk_scb is avaliable the address ACK will follow S_READY_ADDR_ACK - 0: in Active/Sleep mode clock stretching is performed when the RX FIFO is full, the strech is released when the RX FIFO is no longer full. In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the clocked will be streched on an incoming address until clk_scb is avaliable. After clk_scb is avalaible the address ACK will follow S_READY_ADDR_ACK"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> S_NOT_READY_ADDR_NACK_R {
        S_NOT_READY_ADDR_NACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> S_NOT_READY_DATA_NACK_R {
        S_NOT_READY_DATA_NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the SCB to address itself."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. (HIGH_PHASE_OVS + 1) * clk_scb constitutes the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
    #[inline(always)]
    pub fn high_phase_ovs(&mut self) -> HIGH_PHASE_OVS_W<0> {
        HIGH_PHASE_OVS_W::new(self)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. (LOW_PHASE_OVS + 1) * clk_scb constitutes the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. See architecture TRM for information on slave data rate requirments."]
    #[inline(always)]
    pub fn low_phase_ovs(&mut self) -> LOW_PHASE_OVS_W<4> {
        LOW_PHASE_OVS_W::new(self)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the RX FIFO is not full. When '0' the CPU is responsible for ACK/NACKing the received data frame using I2C_M_CMD.M_ACK or I2C_M_CMD.M_NACK"]
    #[inline(always)]
    pub fn m_ready_data_ack(&mut self) -> M_READY_DATA_ACK_W<8> {
        M_READY_DATA_ACK_W::new(self)
    }
    #[doc = "Bit 9 - When '1', a received data element by the master is immediately NACK'd when the RX FIFO is full. When '0', clock stretching is used instead (till the RX FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&mut self) -> M_NOT_READY_DATA_NACK_W<9> {
        M_NOT_READY_DATA_NACK_W::new(self)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure. When '0' the general call address is accepted and follows S_READY_ADDR_ACK and S_NOT_READY_ADDR_NACK"]
    #[inline(always)]
    pub fn s_general_ignore(&mut self) -> S_GENERAL_IGNORE_W<11> {
        S_GENERAL_IGNORE_W::new(self)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the address must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&mut self) -> S_READY_ADDR_ACK_W<12> {
        S_READY_ADDR_ACK_W::new(self)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the RX FIFO is not full. In EZ and CMD_RESP mode, this field should be set to '1'. When '0' the data must be ACK/NACK'd by the CPU using I2C_S_CMD.S_ACK or I2C_S_CMD.S_NACK"]
    #[inline(always)]
    pub fn s_ready_data_ack(&mut self) -> S_READY_DATA_ACK_W<13> {
        S_READY_DATA_ACK_W::new(self)
    }
    #[doc = "Bit 14 - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: In Active/Sleep mode a received (matching) slave address is immediately NACK'd when the RX FIFO is full In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the incoming address will be NACK'd until the clock is avaliable. Once clk_scb is avaliable the address ACK will follow S_READY_ADDR_ACK - 0: in Active/Sleep mode clock stretching is performed when the RX FIFO is full, the strech is released when the RX FIFO is no longer full. In DeepSleep power mode when EC_AM = '1' and EC_OP = '0' clk_scb is not avaliable, so the clocked will be streched on an incoming address until clk_scb is avaliable. After clk_scb is avalaible the address ACK will follow S_READY_ADDR_ACK"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&mut self) -> S_NOT_READY_ADDR_NACK_W<14> {
        S_NOT_READY_ADDR_NACK_W::new(self)
    }
    #[doc = "Bit 15 - Only used for FIFO mode, NOT EZ or CMD_RESP mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&mut self) -> S_NOT_READY_DATA_NACK_W<15> {
        S_NOT_READY_DATA_NACK_W::new(self)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', no loopback When '1', loopback is enabled internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<16> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W<30> {
        SLAVE_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the SCB to address itself."]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W<31> {
        MASTER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl](index.html) module"]
pub struct I2C_CTRL_SPEC;
impl crate::RegisterSpec for I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctrl::R](R) reader structure"]
impl crate::Readable for I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl::W](W) writer structure"]
impl crate::Writable for I2C_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTRL to value 0xfb88"]
impl crate::Resettable for I2C_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfb88
    }
}
