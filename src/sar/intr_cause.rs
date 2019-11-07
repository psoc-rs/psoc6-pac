#[doc = "Reader of register INTR_CAUSE"]
pub type R = crate::R<u32, super::INTR_CAUSE>;
#[doc = "Reader of field `EOS_MASKED_MIR`"]
pub type EOS_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERFLOW_MASKED_MIR`"]
pub type OVERFLOW_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FW_COLLISION_MASKED_MIR`"]
pub type FW_COLLISION_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSI_COLLISION_MASKED_MIR`"]
pub type DSI_COLLISION_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_EOC_MASKED_MIR`"]
pub type INJ_EOC_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_SATURATE_MASKED_MIR`"]
pub type INJ_SATURATE_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_RANGE_MASKED_MIR`"]
pub type INJ_RANGE_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_COLLISION_MASKED_MIR`"]
pub type INJ_COLLISION_MASKED_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SATURATE_MASKED_RED`"]
pub type SATURATE_MASKED_RED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RANGE_MASKED_RED`"]
pub type RANGE_MASKED_RED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn eos_masked_mir(&self) -> EOS_MASKED_MIR_R {
        EOS_MASKED_MIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn overflow_masked_mir(&self) -> OVERFLOW_MASKED_MIR_R {
        OVERFLOW_MASKED_MIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn fw_collision_masked_mir(&self) -> FW_COLLISION_MASKED_MIR_R {
        FW_COLLISION_MASKED_MIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn dsi_collision_masked_mir(&self) -> DSI_COLLISION_MASKED_MIR_R {
        DSI_COLLISION_MASKED_MIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_eoc_masked_mir(&self) -> INJ_EOC_MASKED_MIR_R {
        INJ_EOC_MASKED_MIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_saturate_masked_mir(&self) -> INJ_SATURATE_MASKED_MIR_R {
        INJ_SATURATE_MASKED_MIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_range_masked_mir(&self) -> INJ_RANGE_MASKED_MIR_R {
        INJ_RANGE_MASKED_MIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_collision_masked_mir(&self) -> INJ_COLLISION_MASKED_MIR_R {
        INJ_COLLISION_MASKED_MIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn saturate_masked_red(&self) -> SATURATE_MASKED_RED_R {
        SATURATE_MASKED_RED_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn range_masked_red(&self) -> RANGE_MASKED_RED_R {
        RANGE_MASKED_RED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
