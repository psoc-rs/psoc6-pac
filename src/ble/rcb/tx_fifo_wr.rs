#[doc = "Register `TX_FIFO_WR` writer"]
pub struct W(crate::W<TX_FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_WR_SPEC>;
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
impl From<crate::W<TX_FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_FIFO_WR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter FIFO write register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_wr](index.html) module"]
pub struct TX_FIFO_WR_SPEC;
impl crate::RegisterSpec for TX_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_wr::W](W) writer structure"]
impl crate::Writable for TX_FIFO_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_FIFO_WR to value 0"]
impl crate::Resettable for TX_FIFO_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
