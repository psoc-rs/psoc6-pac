#[doc = "Reader of register RX_DATA_FIFO_RD1_SILENT"]
pub type R = crate::R<u32, super::RX_DATA_FIFO_RD1_SILENT>;
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
}
