#[doc = "Register `ACT_SRC` reader"]
pub struct R(crate::R<ACT_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC_ADDR` reader - Current address of source location."]
pub type SRC_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn src_addr(&self) -> SRC_ADDR_R {
        SRC_ADDR_R::new(self.bits)
    }
}
#[doc = "Active source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_src](index.html) module"]
pub struct ACT_SRC_SPEC;
impl crate::RegisterSpec for ACT_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_src::R](R) reader structure"]
impl crate::Readable for ACT_SRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_SRC to value 0"]
impl crate::Resettable for ACT_SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
