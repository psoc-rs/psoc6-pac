#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_TX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_TX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TR_RX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_RX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_RX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TR_RX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type XIP_ALIGNMENT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_CMD_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TX_DATA_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_DATA_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W<0> {
        TR_TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W<1> {
        TR_RX_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W<2> {
        XIP_ALIGNMENT_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W<3> {
        TX_CMD_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W<4> {
        TX_DATA_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&mut self) -> RX_DATA_FIFO_UNDERFLOW_W<5> {
        RX_DATA_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
