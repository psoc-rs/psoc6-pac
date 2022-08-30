#[doc = "Register `COMP_STAT` reader"]
pub struct R(crate::R<COMP_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OA0_COMP` reader - Opamp0 current comparator status"]
pub type OA0_COMP_R = crate::BitReader<bool>;
#[doc = "Field `OA1_COMP` reader - Opamp1 current comparator status"]
pub type OA1_COMP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Opamp0 current comparator status"]
    #[inline(always)]
    pub fn oa0_comp(&self) -> OA0_COMP_R {
        OA0_COMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Opamp1 current comparator status"]
    #[inline(always)]
    pub fn oa1_comp(&self) -> OA1_COMP_R {
        OA1_COMP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Comparator status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_stat](index.html) module"]
pub struct COMP_STAT_SPEC;
impl crate::RegisterSpec for COMP_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_stat::R](R) reader structure"]
impl crate::Readable for COMP_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP_STAT to value 0"]
impl crate::Resettable for COMP_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
