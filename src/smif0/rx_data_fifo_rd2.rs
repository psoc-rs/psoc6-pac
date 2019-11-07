#[doc = "Reader of register RX_DATA_FIFO_RD2"]
pub type R = crate::R<u32, super::RX_DATA_FIFO_RD2>;
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATA1`"]
pub type DATA1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
