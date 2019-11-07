#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR_TX_REQ`"]
pub type TR_TX_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_TX_REQ`"]
pub struct TR_TX_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_TX_REQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TR_RX_REQ`"]
pub type TR_RX_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_RX_REQ`"]
pub struct TR_RX_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_RX_REQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `XIP_ALIGNMENT_ERROR`"]
pub type XIP_ALIGNMENT_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIP_ALIGNMENT_ERROR`"]
pub struct XIP_ALIGNMENT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_ALIGNMENT_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TX_CMD_FIFO_OVERFLOW`"]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CMD_FIFO_OVERFLOW`"]
pub struct TX_CMD_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CMD_FIFO_OVERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TX_DATA_FIFO_OVERFLOW`"]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DATA_FIFO_OVERFLOW`"]
pub struct TX_DATA_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_FIFO_OVERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RX_DATA_FIFO_UNDERFLOW`"]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_FIFO_UNDERFLOW`"]
pub struct RX_DATA_FIFO_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_FIFO_UNDERFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W {
        TR_TX_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W {
        TR_RX_REQ_W { w: self }
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W {
        XIP_ALIGNMENT_ERROR_W { w: self }
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W {
        TX_CMD_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W {
        TX_DATA_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&mut self) -> RX_DATA_FIFO_UNDERFLOW_W {
        RX_DATA_FIFO_UNDERFLOW_W { w: self }
    }
}
