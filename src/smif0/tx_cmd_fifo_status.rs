#[doc = "Reader of register TX_CMD_FIFO_STATUS"]
pub type R = crate::R<u32, super::TX_CMD_FIFO_STATUS>;
#[doc = "Reader of field `USED3`"]
pub type USED3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
    #[inline(always)]
    pub fn used3(&self) -> USED3_R {
        USED3_R::new((self.bits & 0x07) as u8)
    }
}
