#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOS_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type EOS_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type OVERFLOW_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type FW_COLLISION_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type DSI_COLLISION_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type INJ_EOC_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type INJ_SATURATE_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type INJ_RANGE_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type INJ_COLLISION_MASKED_MIR_R = crate::BitReader<bool>;
#[doc = "Field `SATURATE_MASKED_RED` reader - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
pub type SATURATE_MASKED_RED_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_MASKED_RED` reader - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
pub type RANGE_MASKED_RED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn eos_masked_mir(&self) -> EOS_MASKED_MIR_R {
        EOS_MASKED_MIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn overflow_masked_mir(&self) -> OVERFLOW_MASKED_MIR_R {
        OVERFLOW_MASKED_MIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn fw_collision_masked_mir(&self) -> FW_COLLISION_MASKED_MIR_R {
        FW_COLLISION_MASKED_MIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn dsi_collision_masked_mir(&self) -> DSI_COLLISION_MASKED_MIR_R {
        DSI_COLLISION_MASKED_MIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_eoc_masked_mir(&self) -> INJ_EOC_MASKED_MIR_R {
        INJ_EOC_MASKED_MIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_saturate_masked_mir(&self) -> INJ_SATURATE_MASKED_MIR_R {
        INJ_SATURATE_MASKED_MIR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_range_masked_mir(&self) -> INJ_RANGE_MASKED_MIR_R {
        INJ_RANGE_MASKED_MIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_collision_masked_mir(&self) -> INJ_COLLISION_MASKED_MIR_R {
        INJ_COLLISION_MASKED_MIR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 30 - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn saturate_masked_red(&self) -> SATURATE_MASKED_RED_R {
        SATURATE_MASKED_RED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn range_masked_red(&self) -> RANGE_MASKED_RED_R {
        RANGE_MASKED_RED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
