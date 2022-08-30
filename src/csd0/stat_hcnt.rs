#[doc = "Register `STAT_HCNT` reader"]
pub struct R(crate::R<STAT_HCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_HCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_HCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_HCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Current value of HSCMP counter"]
pub type CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of HSCMP counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current count of the HSCMP counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_hcnt](index.html) module"]
pub struct STAT_HCNT_SPEC;
impl crate::RegisterSpec for STAT_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat_hcnt::R](R) reader structure"]
impl crate::Readable for STAT_HCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT_HCNT to value 0"]
impl crate::Resettable for STAT_HCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
