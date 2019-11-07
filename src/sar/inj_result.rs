#[doc = "Reader of register INJ_RESULT"]
pub type R = crate::R<u32, super::INJ_RESULT>;
#[doc = "Reader of field `INJ_RESULT`"]
pub type INJ_RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `INJ_NEWVALUE`"]
pub type INJ_NEWVALUE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_COLLISION_INTR_MIR`"]
pub type INJ_COLLISION_INTR_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_SATURATE_INTR_MIR`"]
pub type INJ_SATURATE_INTR_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_RANGE_INTR_MIR`"]
pub type INJ_RANGE_INTR_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `INJ_EOC_INTR_MIR`"]
pub type INJ_EOC_INTR_MIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel."]
    #[inline(always)]
    pub fn inj_result(&self) -> INJ_RESULT_R {
        INJ_RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - The data in this register received a new value (only relevant for UAB, this bit shows the value of the UAB valid bit)"]
    #[inline(always)]
    pub fn inj_newvalue(&self) -> INJ_NEWVALUE_R {
        INJ_NEWVALUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_collision_intr_mir(&self) -> INJ_COLLISION_INTR_MIR_R {
        INJ_COLLISION_INTR_MIR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_saturate_intr_mir(&self) -> INJ_SATURATE_INTR_MIR_R {
        INJ_SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_range_intr_mir(&self) -> INJ_RANGE_INTR_MIR_R {
        INJ_RANGE_INTR_MIR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_eoc_intr_mir(&self) -> INJ_EOC_INTR_MIR_R {
        INJ_EOC_INTR_MIR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
