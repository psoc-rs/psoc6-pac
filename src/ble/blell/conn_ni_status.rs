#[doc = "Register `CONN_NI_STATUS` reader"]
pub struct R(crate::R<CONN_NI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_NI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_NI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_NI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONN_NI` reader - HW updates this register with the next Connection Instant for current serviced connection, granularity is 625us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
pub type CONN_NI_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW updates this register with the next Connection Instant for current serviced connection, granularity is 625us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
    #[inline(always)]
    pub fn conn_ni(&self) -> CONN_NI_R {
        CONN_NI_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Connection NI Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ni_status](index.html) module"]
pub struct CONN_NI_STATUS_SPEC;
impl crate::RegisterSpec for CONN_NI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ni_status::R](R) reader structure"]
impl crate::Readable for CONN_NI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONN_NI_STATUS to value 0xffff"]
impl crate::Resettable for CONN_NI_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
