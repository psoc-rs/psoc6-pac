#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
