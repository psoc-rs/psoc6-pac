#[doc = "Reader of register CM0_CA_STATUS2"]
pub type R = crate::R<u32, super::CM0_CA_STATUS2>;
#[doc = "Reader of field `LRU`"]
pub type LRU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new((self.bits & 0x3f) as u8)
    }
}
