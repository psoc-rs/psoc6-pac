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
#[doc = "Field `TX_TRIGGER` reader - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
pub type TX_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TX_TRIGGER` writer - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
pub type TX_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_NOT_FULL` reader - TX FIFO is not full."]
pub type TX_NOT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_NOT_FULL` writer - TX FIFO is not full."]
pub type TX_NOT_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_EMPTY` reader - TX FIFO is empty; i.e. it has 0 entries."]
pub type TX_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_EMPTY` writer - TX FIFO is empty; i.e. it has 0 entries."]
pub type TX_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_OVERFLOW` reader - Attempt to write to a full TX FIFO."]
pub type TX_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_OVERFLOW` writer - Attempt to write to a full TX FIFO."]
pub type TX_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
pub type TX_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
pub type TX_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_WD` reader - Triggers (sets to '1') when the Tx watchdog event occurs."]
pub type TX_WD_R = crate::BitReader<bool>;
#[doc = "Field `TX_WD` writer - Triggers (sets to '1') when the Tx watchdog event occurs."]
pub type TX_WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_TRIGGER` reader - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
pub type RX_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_TRIGGER` writer - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
pub type RX_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_NOT_EMPTY` reader - RX FIFO is not empty."]
pub type RX_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOT_EMPTY` writer - RX FIFO is not empty."]
pub type RX_NOT_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_FULL` reader - RX FIFO is full."]
pub type RX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL` writer - RX FIFO is full."]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_OVERFLOW` reader - Attempt to write to a full RX FIFO."]
pub type RX_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW` writer - Attempt to write to a full RX FIFO."]
pub type RX_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_UNDERFLOW` reader - Attempt to read from an empty RX FIFO."]
pub type RX_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW` writer - Attempt to read from an empty RX FIFO."]
pub type RX_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_WD` reader - Triggers (sets to '1') when the Rx watchdog event occurs."]
pub type RX_WD_R = crate::BitReader<bool>;
#[doc = "Field `RX_WD` writer - Triggers (sets to '1') when the Rx watchdog event occurs."]
pub type RX_WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TX_TRIGGER_R {
        TX_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TX_NOT_FULL_R {
        TX_NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TX_OVERFLOW_R {
        TX_OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TX_WD_R {
        TX_WD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX FIFO is full."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RX_WD_R {
        RX_WD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TRIGGER_LEVEL in TX_FIFO_CTRL."]
    #[inline(always)]
    pub fn tx_trigger(&mut self) -> TX_TRIGGER_W<0> {
        TX_TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO is not full."]
    #[inline(always)]
    pub fn tx_not_full(&mut self) -> TX_NOT_FULL_W<1> {
        TX_NOT_FULL_W::new(self)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries."]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W<4> {
        TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO."]
    #[inline(always)]
    pub fn tx_overflow(&mut self) -> TX_OVERFLOW_W<5> {
        TX_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and TX_EMPTY is '1'."]
    #[inline(always)]
    pub fn tx_underflow(&mut self) -> TX_UNDERFLOW_W<6> {
        TX_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Triggers (sets to '1') when the Tx watchdog event occurs."]
    #[inline(always)]
    pub fn tx_wd(&mut self) -> TX_WD_W<8> {
        TX_WD_W::new(self)
    }
    #[doc = "Bit 16 - More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in RX_FIFO_CTRL."]
    #[inline(always)]
    pub fn rx_trigger(&mut self) -> RX_TRIGGER_W<16> {
        RX_TRIGGER_W::new(self)
    }
    #[doc = "Bit 18 - RX FIFO is not empty."]
    #[inline(always)]
    pub fn rx_not_empty(&mut self) -> RX_NOT_EMPTY_W<18> {
        RX_NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 19 - RX FIFO is full."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W<19> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 21 - Attempt to write to a full RX FIFO."]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W<21> {
        RX_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 22 - Attempt to read from an empty RX FIFO."]
    #[inline(always)]
    pub fn rx_underflow(&mut self) -> RX_UNDERFLOW_W<22> {
        RX_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 24 - Triggers (sets to '1') when the Rx watchdog event occurs."]
    #[inline(always)]
    pub fn rx_wd(&mut self) -> RX_WD_W<24> {
        RX_WD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
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
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
