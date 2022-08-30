#[doc = "Register `EARLY_INTR_STATUS` reader"]
pub struct R(crate::R<EARLY_INTR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EARLY_INTR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EARLY_INTR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EARLY_INTR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONN_INDEX_FOR_EARLY_INTR` reader - Connection Index for which early interrupt is raised"]
pub type CONN_INDEX_FOR_EARLY_INTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONN_TYPE_FOR_EARLY_INTR` reader - Connection type for which early interrupt is raised."]
pub type CONN_TYPE_FOR_EARLY_INTR_R = crate::BitReader<bool>;
#[doc = "Field `US_FOR_EARLY_INTR` reader - US offset when early interrupt is raised"]
pub type US_FOR_EARLY_INTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:4 - Connection Index for which early interrupt is raised"]
    #[inline(always)]
    pub fn conn_index_for_early_intr(&self) -> CONN_INDEX_FOR_EARLY_INTR_R {
        CONN_INDEX_FOR_EARLY_INTR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type for which early interrupt is raised."]
    #[inline(always)]
    pub fn conn_type_for_early_intr(&self) -> CONN_TYPE_FOR_EARLY_INTR_R {
        CONN_TYPE_FOR_EARLY_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15 - US offset when early interrupt is raised"]
    #[inline(always)]
    pub fn us_for_early_intr(&self) -> US_FOR_EARLY_INTR_R {
        US_FOR_EARLY_INTR_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
#[doc = "Status when early interrupt is raised\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [early_intr_status](index.html) module"]
pub struct EARLY_INTR_STATUS_SPEC;
impl crate::RegisterSpec for EARLY_INTR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [early_intr_status::R](R) reader structure"]
impl crate::Readable for EARLY_INTR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EARLY_INTR_STATUS to value 0"]
impl crate::Resettable for EARLY_INTR_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
