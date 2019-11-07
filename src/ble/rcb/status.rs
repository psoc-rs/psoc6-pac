#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUS_BUSY`"]
pub type BUS_BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RCB bus is busy. The bus is considered busy ('1') during an ongoing transaction."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
