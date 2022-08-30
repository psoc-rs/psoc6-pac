#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START` reader - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TX_START_R = crate::BitReader<bool>;
#[doc = "Field `TX_START` writer - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TX_PAUSE` reader - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TX_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `TX_PAUSE` writer - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TX_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RX_START` reader - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RX_START_R = crate::BitReader<bool>;
#[doc = "Field `RX_START` writer - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RX_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&self) -> TX_PAUSE_R {
        TX_PAUSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<0> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&mut self) -> TX_PAUSE_W<8> {
        TX_PAUSE_W::new(self)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W<16> {
        RX_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
