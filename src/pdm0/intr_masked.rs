#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `RX_TRIGGER`"]
pub type RX_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_NOT_EMPTY`"]
pub type RX_NOT_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_OVERFLOW`"]
pub type RX_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_UNDERFLOW`"]
pub type RX_UNDERFLOW_R = crate::R<bool, bool>;
impl R {
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
}
