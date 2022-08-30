#[doc = "Register `TX_CMD_FIFO_WR` writer"]
pub struct W(crate::W<TX_CMD_FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CMD_FIFO_WR_SPEC>;
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
impl From<crate::W<TX_CMD_FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CMD_FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA20` writer - Command data. The higher two bits DATA\\[19:18\\]
specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\]
specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\]
specifies which of the four devices are selected. DATA\\[11:8\\]
are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\]
specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\]
specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
pub type DATA20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_CMD_FIFO_WR_SPEC, u32, u32, 20, O>;
impl W {
    #[doc = "Bits 0:19 - Command data. The higher two bits DATA\\[19:18\\]
specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\]
specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\]
specifies which of the four devices are selected. DATA\\[11:8\\]
are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\]
specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\]
specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
    #[inline(always)]
    pub fn data20(&mut self) -> DATA20_W<0> {
        DATA20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter command FIFO write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_cmd_fifo_wr](index.html) module"]
pub struct TX_CMD_FIFO_WR_SPEC;
impl crate::RegisterSpec for TX_CMD_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tx_cmd_fifo_wr::W](W) writer structure"]
impl crate::Writable for TX_CMD_FIFO_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CMD_FIFO_WR to value 0"]
impl crate::Resettable for TX_CMD_FIFO_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
