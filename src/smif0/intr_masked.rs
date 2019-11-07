#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `TR_TX_REQ`"]
pub type TR_TX_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR_RX_REQ`"]
pub type TR_RX_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `XIP_ALIGNMENT_ERROR`"]
pub type XIP_ALIGNMENT_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_CMD_FIFO_OVERFLOW`"]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_DATA_FIFO_OVERFLOW`"]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_DATA_FIFO_UNDERFLOW`"]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
