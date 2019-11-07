#[doc = "Reader of register RX_FIFO_STATUS"]
pub type R = crate::R<u32, super::RX_FIFO_STATUS>;
#[doc = "Reader of field `USED`"]
pub type USED_R = crate::R<u8, u8>;
#[doc = "Reader of field `RD_PTR`"]
pub type RD_PTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_PTR`"]
pub type WR_PTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of entries in the RX FIFO. The field value is in the range \\[0, 255\\]. When this is zero, the RX FIFO is empty."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX FIFO read pointer: RX FIFO location from which a data frame is read by the host.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX FIFO write pointer: RX FIFO location at which a new data frame is written by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
