#[doc = "Reader of register RX_DATA_FIFO_STATUS"]
pub type R = crate::R<u32, super::RX_DATA_FIFO_STATUS>;
#[doc = "Reader of field `USED4`"]
pub type USED4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn used4(&self) -> USED4_R {
        USED4_R::new((self.bits & 0x0f) as u8)
    }
}
