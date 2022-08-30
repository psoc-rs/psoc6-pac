#[doc = "Register `CHAN_RESULT_NEWVALUE` reader"]
pub struct R(crate::R<CHAN_RESULT_NEWVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_RESULT_NEWVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_RESULT_NEWVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_RESULT_NEWVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN_RESULT_NEWVALUE` reader - If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
pub type CHAN_RESULT_NEWVALUE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub fn chan_result_newvalue(&self) -> CHAN_RESULT_NEWVALUE_R {
        CHAN_RESULT_NEWVALUE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel result data register 'new value' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result_newvalue](index.html) module"]
pub struct CHAN_RESULT_NEWVALUE_SPEC;
impl crate::RegisterSpec for CHAN_RESULT_NEWVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_result_newvalue::R](R) reader structure"]
impl crate::Readable for CHAN_RESULT_NEWVALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHAN_RESULT_NEWVALUE to value 0"]
impl crate::Resettable for CHAN_RESULT_NEWVALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
