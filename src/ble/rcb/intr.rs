#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB_DONE` reader - N/A"]
pub type RCB_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RCB_DONE` writer - N/A"]
pub type RCB_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_TRIGGER` reader - N/A"]
pub type TX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_TRIGGER` writer - N/A"]
pub type TX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_NOT_FULL` reader - N/A"]
pub type TX_FIFO_NOT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_NOT_FULL` writer - N/A"]
pub type TX_FIFO_NOT_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_EMPTY` reader - N/A"]
pub type TX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_EMPTY` writer - N/A"]
pub type TX_FIFO_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_OVERFLOW` reader - N/A"]
pub type TX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_OVERFLOW` writer - N/A"]
pub type TX_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type TX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type TX_FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_TRIGGER` reader - N/A"]
pub type RX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_TRIGGER` writer - N/A"]
pub type RX_FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - N/A"]
pub type RX_FIFO_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_NOT_EMPTY` writer - N/A"]
pub type RX_FIFO_NOT_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_FULL` reader - N/A"]
pub type RX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_FULL` writer - N/A"]
pub type RX_FIFO_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_OVERFLOW` reader - N/A"]
pub type RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_OVERFLOW` writer - N/A"]
pub type RX_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_UNDERFLOW` reader - N/A"]
pub type RX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_UNDERFLOW` writer - N/A"]
pub type RX_FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rcb_done(&self) -> RCB_DONE_R {
        RCB_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TX_FIFO_NOT_FULL_R {
        TX_FIFO_NOT_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Attempt to read from an empty TX FIFO. This happens when SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rcb_done(&mut self) -> RCB_DONE_W<0> {
        RCB_DONE_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_trigger(&mut self) -> TX_FIFO_TRIGGER_W<8> {
        TX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&mut self) -> TX_FIFO_NOT_FULL_W<9> {
        TX_FIFO_NOT_FULL_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TX_FIFO_EMPTY_W<10> {
        TX_FIFO_EMPTY_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&mut self) -> TX_FIFO_OVERFLOW_W<11> {
        TX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 12 - Attempt to read from an empty TX FIFO. This happens when SCB is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TX_FIFO_UNDERFLOW_W<12> {
        TX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_trigger(&mut self) -> RX_FIFO_TRIGGER_W<16> {
        RX_FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RX_FIFO_NOT_EMPTY_W<17> {
        RX_FIFO_NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_full(&mut self) -> RX_FIFO_FULL_W<18> {
        RX_FIFO_FULL_W::new(self)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W<19> {
        RX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&mut self) -> RX_FIFO_UNDERFLOW_W<20> {
        RX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0x0600"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
