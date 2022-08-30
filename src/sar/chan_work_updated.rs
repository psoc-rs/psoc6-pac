#[doc = "Register `CHAN_WORK_UPDATED` reader"]
pub struct R(crate::R<CHAN_WORK_UPDATED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_WORK_UPDATED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_WORK_UPDATED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_WORK_UPDATED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN_WORK_UPDATED` reader - If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
pub type CHAN_WORK_UPDATED_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_work_updated(&self) -> CHAN_WORK_UPDATED_R {
        CHAN_WORK_UPDATED_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel working data register 'updated' bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work_updated](index.html) module"]
pub struct CHAN_WORK_UPDATED_SPEC;
impl crate::RegisterSpec for CHAN_WORK_UPDATED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_work_updated::R](R) reader structure"]
impl crate::Readable for CHAN_WORK_UPDATED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHAN_WORK_UPDATED to value 0"]
impl crate::Resettable for CHAN_WORK_UPDATED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
