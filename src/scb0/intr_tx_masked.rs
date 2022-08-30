#[doc = "Register `INTR_TX_MASKED` reader"]
pub struct R(crate::R<INTR_TX_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `NOT_FULL` reader - Logical and of corresponding request and mask bits."]
pub type NOT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `BLOCKED` reader - Logical and of corresponding request and mask bits."]
pub type BLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `UART_NACK` reader - Logical and of corresponding request and mask bits."]
pub type UART_NACK_R = crate::BitReader<bool>;
#[doc = "Field `UART_DONE` reader - Logical and of corresponding request and mask bits."]
pub type UART_DONE_R = crate::BitReader<bool>;
#[doc = "Field `UART_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type UART_ARB_LOST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Transmitter interrupt masked request\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx_masked](index.html) module"]
pub struct INTR_TX_MASKED_SPEC;
impl crate::RegisterSpec for INTR_TX_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx_masked::R](R) reader structure"]
impl crate::Readable for INTR_TX_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_TX_MASKED to value 0"]
impl crate::Resettable for INTR_TX_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
