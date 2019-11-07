#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `RCB_DONE`"]
pub type RCB_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FIFO_TRIGGER`"]
pub type TX_FIFO_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FIFO_NOT_FULL`"]
pub type TX_FIFO_NOT_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FIFO_EMPTY`"]
pub type TX_FIFO_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FIFO_OVERFLOW`"]
pub type TX_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FIFO_UNDERFLOW`"]
pub type TX_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_TRIGGER`"]
pub type RX_FIFO_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_NOT_EMPTY`"]
pub type RX_FIFO_NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_FULL`"]
pub type RX_FIFO_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_OVERFLOW`"]
pub type RX_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_UNDERFLOW`"]
pub type RX_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rcb_done(&self) -> RCB_DONE_R {
        RCB_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_trigger(&self) -> TX_FIFO_TRIGGER_R {
        TX_FIFO_TRIGGER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TX_FIFO_NOT_FULL_R {
        TX_FIFO_NOT_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_trigger(&self) -> RX_FIFO_TRIGGER_R {
        RX_FIFO_TRIGGER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
