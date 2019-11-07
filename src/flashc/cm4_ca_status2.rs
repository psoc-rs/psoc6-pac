#[doc = "Reader of register CM4_CA_STATUS2"]
pub type R = crate::R<u32, super::CM4_CA_STATUS2>;
#[doc = "Reader of field `LRU`"]
pub type LRU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - See CM0_CA_STATUS2."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new((self.bits & 0x3f) as u8)
    }
}
