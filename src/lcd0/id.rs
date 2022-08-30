#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - the ID of LCD controller peripheral is 0xF0F0"]
pub type ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REVISION` reader - the version number is 0x0001"]
pub type REVISION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - the ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the version number is 0x0001"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ID & Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID to value 0x0001_f0f0"]
impl crate::Resettable for ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_f0f0
    }
}
