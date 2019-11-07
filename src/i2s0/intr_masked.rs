#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `TX_TRIGGER`"]
pub type TX_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_NOT_FULL`"]
pub type TX_NOT_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_EMPTY`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_OVERFLOW`"]
pub type TX_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_UNDERFLOW`"]
pub type TX_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_WD`"]
pub type TX_WD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_TRIGGER`"]
pub type RX_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_NOT_EMPTY`"]
pub type RX_NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FULL`"]
pub type RX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_OVERFLOW`"]
pub type RX_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_UNDERFLOW`"]
pub type RX_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_WD`"]
pub type RX_WD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_trigger(&self) -> TX_TRIGGER_R {
        TX_TRIGGER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_not_full(&self) -> TX_NOT_FULL_R {
        TX_NOT_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_overflow(&self) -> TX_OVERFLOW_R {
        TX_OVERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_wd(&self) -> TX_WD_R {
        TX_WD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_wd(&self) -> RX_WD_R {
        RX_WD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
