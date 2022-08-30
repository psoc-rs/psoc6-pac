#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCB_DONE` reader - Logical and of corresponding request and mask bits."]
pub type RCB_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_NOT_FULL` reader - Logical and of corresponding request and mask bits."]
pub type TX_FIFO_NOT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type TX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type RX_FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type RX_FIFO_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_FULL` reader - Logical and of corresponding request and mask bits."]
pub type RX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rcb_done(&self) -> RCB_DONE_R {
        RCB_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TX_FIFO_NOT_FULL_R {
        TX_FIFO_NOT_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Master interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
