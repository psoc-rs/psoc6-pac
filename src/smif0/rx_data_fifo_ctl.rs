#[doc = "Register `RX_DATA_FIFO_CTL` reader"]
pub struct R(crate::R<RX_DATA_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DATA_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DATA_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DATA_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DATA_FIFO_CTL` writer"]
pub struct W(crate::W<RX_DATA_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DATA_FIFO_CTL_SPEC>;
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
impl From<crate::W<RX_DATA_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DATA_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_DATA_FIFO_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines when RX data FIFO 'tr_rx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when RX_DATA_FIFO_STATUS.USED > TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver data FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_data_fifo_ctl](index.html) module"]
pub struct RX_DATA_FIFO_CTL_SPEC;
impl crate::RegisterSpec for RX_DATA_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_data_fifo_ctl::R](R) reader structure"]
impl crate::Readable for RX_DATA_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_data_fifo_ctl::W](W) writer structure"]
impl crate::Writable for RX_DATA_FIFO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DATA_FIFO_CTL to value 0"]
impl crate::Resettable for RX_DATA_FIFO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
