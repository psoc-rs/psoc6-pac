#[doc = "Writer for register TX_CMD_FIFO_WR"]
pub type W = crate::W<u32, super::TX_CMD_FIFO_WR>;
#[doc = "Register TX_CMD_FIFO_WR `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CMD_FIFO_WR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA20`"]
pub struct DATA20_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:19 - Command data. The higher two bits DATA\\[19:18\\] specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\] specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\] specifies which of the four devices are selected. DATA\\[11:8\\] are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\] specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\] specifies the width of the transfer. - DATA\\[15:0\\] specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\] specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
    #[inline(always)]
    pub fn data20(&mut self) -> DATA20_W {
        DATA20_W { w: self }
    }
}
