#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOS_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type EOS_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type OVERFLOW_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type FW_COLLISION_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type DSI_COLLISION_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_EOC_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_SATURATE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_RANGE_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type INJ_COLLISION_MASKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn eos_masked(&self) -> EOS_MASKED_R {
        EOS_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow_masked(&self) -> OVERFLOW_MASKED_R {
        OVERFLOW_MASKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fw_collision_masked(&self) -> FW_COLLISION_MASKED_R {
        FW_COLLISION_MASKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dsi_collision_masked(&self) -> DSI_COLLISION_MASKED_R {
        DSI_COLLISION_MASKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_eoc_masked(&self) -> INJ_EOC_MASKED_R {
        INJ_EOC_MASKED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_saturate_masked(&self) -> INJ_SATURATE_MASKED_R {
        INJ_SATURATE_MASKED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_range_masked(&self) -> INJ_RANGE_MASKED_R {
        INJ_RANGE_MASKED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_collision_masked(&self) -> INJ_COLLISION_MASKED_R {
        INJ_COLLISION_MASKED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
