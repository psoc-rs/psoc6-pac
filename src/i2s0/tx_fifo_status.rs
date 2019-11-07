#[doc = "Reader of register TX_FIFO_STATUS"]
pub type R = crate::R<u32, super::TX_FIFO_STATUS>;
#[doc = "Reader of field `USED`"]
pub type USED_R = crate::R<u16, u16>;
#[doc = "Reader of field `RD_PTR`"]
pub type RD_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_PTR`"]
pub type WR_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:8 - Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
