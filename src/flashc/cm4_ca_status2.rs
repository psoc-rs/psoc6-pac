#[doc = "Register `CM4_CA_STATUS2` reader"]
pub struct R(crate::R<CM4_CA_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_CA_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_CA_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_CA_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LRU` reader - See CM0_CA_STATUS2."]
pub type LRU_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - See CM0_CA_STATUS2."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CM4 cache status 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status2](index.html) module"]
pub struct CM4_CA_STATUS2_SPEC;
impl crate::RegisterSpec for CM4_CA_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_ca_status2::R](R) reader structure"]
impl crate::Readable for CM4_CA_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_CA_STATUS2 to value 0"]
impl crate::Resettable for CM4_CA_STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
