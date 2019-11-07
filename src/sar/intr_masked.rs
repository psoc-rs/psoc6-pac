#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `EOS_MASKED`"]
pub type EOS_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERFLOW_MASKED`"]
pub type OVERFLOW_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `FW_COLLISION_MASKED`"]
pub type FW_COLLISION_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSI_COLLISION_MASKED`"]
pub type DSI_COLLISION_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_EOC_MASKED`"]
pub type INJ_EOC_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_SATURATE_MASKED`"]
pub type INJ_SATURATE_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_RANGE_MASKED`"]
pub type INJ_RANGE_MASKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_COLLISION_MASKED`"]
pub type INJ_COLLISION_MASKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn eos_masked(&self) -> EOS_MASKED_R {
        EOS_MASKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow_masked(&self) -> OVERFLOW_MASKED_R {
        OVERFLOW_MASKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fw_collision_masked(&self) -> FW_COLLISION_MASKED_R {
        FW_COLLISION_MASKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dsi_collision_masked(&self) -> DSI_COLLISION_MASKED_R {
        DSI_COLLISION_MASKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_eoc_masked(&self) -> INJ_EOC_MASKED_R {
        INJ_EOC_MASKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_saturate_masked(&self) -> INJ_SATURATE_MASKED_R {
        INJ_SATURATE_MASKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_range_masked(&self) -> INJ_RANGE_MASKED_R {
        INJ_RANGE_MASKED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_collision_masked(&self) -> INJ_COLLISION_MASKED_R {
        INJ_COLLISION_MASKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
