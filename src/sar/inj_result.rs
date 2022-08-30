#[doc = "Register `INJ_RESULT` reader"]
pub struct R(crate::R<INJ_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INJ_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INJ_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INJ_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INJ_RESULT` reader - SAR conversion result of the channel."]
pub type INJ_RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INJ_NEWVALUE` reader - The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
pub type INJ_NEWVALUE_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_COLLISION_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_SATURATE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_RANGE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_INTR_MIR` reader - mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_EOC_INTR_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel."]
    #[inline(always)]
    pub fn inj_result(&self) -> INJ_RESULT_R {
        INJ_RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
    #[inline(always)]
    pub fn inj_newvalue(&self) -> INJ_NEWVALUE_R {
        INJ_NEWVALUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_collision_intr_mir(&self) -> INJ_COLLISION_INTR_MIR_R {
        INJ_COLLISION_INTR_MIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_saturate_intr_mir(&self) -> INJ_SATURATE_INTR_MIR_R {
        INJ_SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_range_intr_mir(&self) -> INJ_RANGE_INTR_MIR_R {
        INJ_RANGE_INTR_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_eoc_intr_mir(&self) -> INJ_EOC_INTR_MIR_R {
        INJ_EOC_INTR_MIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Injection channel result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_result](index.html) module"]
pub struct INJ_RESULT_SPEC;
impl crate::RegisterSpec for INJ_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inj_result::R](R) reader structure"]
impl crate::Readable for INJ_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INJ_RESULT to value 0"]
impl crate::Resettable for INJ_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
