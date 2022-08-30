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
#[doc = "Field `TR_TX_REQ` reader - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
pub type TR_TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_TX_REQ` writer - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
pub type TR_TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TR_RX_REQ` reader - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
pub type TR_RX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TR_RX_REQ` writer - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
pub type TR_RX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
pub type XIP_ALIGNMENT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
pub type XIP_ALIGNMENT_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
pub type TX_CMD_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
pub type TX_CMD_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
pub type TX_DATA_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
pub type TX_DATA_FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RX_DATA_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RX_DATA_FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TR_TX_REQ_R {
        TR_TX_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TR_RX_REQ_R {
        TR_RX_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XIP_ALIGNMENT_ERROR_R {
        XIP_ALIGNMENT_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TX_CMD_FIFO_OVERFLOW_R {
        TX_CMD_FIFO_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TX_DATA_FIFO_OVERFLOW_R {
        TX_DATA_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RX_DATA_FIFO_UNDERFLOW_R {
        RX_DATA_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&mut self) -> TR_TX_REQ_W<0> {
        TR_TX_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&mut self) -> TR_RX_REQ_W<1> {
        TR_RX_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn xip_alignment_error(&mut self) -> XIP_ALIGNMENT_ERROR_W<2> {
        XIP_ALIGNMENT_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TX_CMD_FIFO_OVERFLOW_W<3> {
        TX_CMD_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&mut self) -> TX_DATA_FIFO_OVERFLOW_W<4> {
        TX_DATA_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&mut self) -> RX_DATA_FIFO_UNDERFLOW_W<5> {
        RX_DATA_FIFO_UNDERFLOW_W::new(self)
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
