#[doc = "Reader of register RX_FIFO_RD"]
pub type R = crate::R<u32, super::RX_FIFO_RD>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'. When this register is read through the debugger, the data frame will not be removed from the FIFO. Similar in operation to RX_FIFO_RD_SILENT"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
