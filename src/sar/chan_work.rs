#[doc = "Register `CHAN_WORK[%s]` reader"]
pub struct R(crate::R<CHAN_WORK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_WORK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_WORK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_WORK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WORK` reader - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
pub type WORK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHAN_WORK_NEWVALUE_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
pub type CHAN_WORK_NEWVALUE_MIR_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_WORK_UPDATED_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
pub type CHAN_WORK_UPDATED_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn work(&self) -> WORK_R {
        WORK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_work_newvalue_mir(&self) -> CHAN_WORK_NEWVALUE_MIR_R {
        CHAN_WORK_NEWVALUE_MIR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
    #[inline(always)]
    pub fn chan_work_updated_mir(&self) -> CHAN_WORK_UPDATED_MIR_R {
        CHAN_WORK_UPDATED_MIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel working data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work](index.html) module"]
pub struct CHAN_WORK_SPEC;
impl crate::RegisterSpec for CHAN_WORK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_work::R](R) reader structure"]
impl crate::Readable for CHAN_WORK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHAN_WORK[%s]
to value 0"]
impl crate::Resettable for CHAN_WORK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
