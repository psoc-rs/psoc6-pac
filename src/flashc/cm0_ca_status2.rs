#[doc = "Register `CM0_CA_STATUS2` reader"]
pub struct R(crate::R<CM0_CA_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CA_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CA_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CA_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LRU` reader - Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
pub type LRU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CM0+ cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_status2](index.html) module"]
pub struct CM0_CA_STATUS2_SPEC;
impl crate::RegisterSpec for CM0_CA_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_ca_status2::R](R) reader structure"]
impl crate::Readable for CM0_CA_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM0_CA_STATUS2 to value 0"]
impl crate::Resettable for CM0_CA_STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
