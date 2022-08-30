#[doc = "Register `CM4_CA_STATUS0` reader"]
pub struct R(crate::R<CM4_CA_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_CA_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_CA_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_CA_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID16` reader - See CM0_CA_STATUS0."]
pub type VALID16_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn valid16(&self) -> VALID16_R {
        VALID16_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CM4 cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status0](index.html) module"]
pub struct CM4_CA_STATUS0_SPEC;
impl crate::RegisterSpec for CM4_CA_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_ca_status0::R](R) reader structure"]
impl crate::Readable for CM4_CA_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_CA_STATUS0 to value 0"]
impl crate::Resettable for CM4_CA_STATUS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
