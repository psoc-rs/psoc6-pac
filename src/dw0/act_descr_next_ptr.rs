#[doc = "Register `ACT_DESCR_NEXT_PTR` reader"]
pub struct R(crate::R<ACT_DESCR_NEXT_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_DESCR_NEXT_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_DESCR_NEXT_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_DESCR_NEXT_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Copy of DESCR_NEXT_PTR of the currently active descriptor."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 2:31 - Copy of DESCR_NEXT_PTR of the currently active descriptor."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
#[doc = "Active descriptor next pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_next_ptr](index.html) module"]
pub struct ACT_DESCR_NEXT_PTR_SPEC;
impl crate::RegisterSpec for ACT_DESCR_NEXT_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_descr_next_ptr::R](R) reader structure"]
impl crate::Readable for ACT_DESCR_NEXT_PTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_DESCR_NEXT_PTR to value 0"]
impl crate::Resettable for ACT_DESCR_NEXT_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
