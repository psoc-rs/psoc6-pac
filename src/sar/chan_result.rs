#[doc = "Reader of register CHAN_RESULT[%s]"]
pub type R = crate::R<u32, super::CHAN_RESULT>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CHAN_RESULT_NEWVALUE_MIR`"]
pub type CHAN_RESULT_NEWVALUE_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SATURATE_INTR_MIR`"]
pub type SATURATE_INTR_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RANGE_INTR_MIR`"]
pub type RANGE_INTR_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHAN_RESULT_UPDATED_MIR`"]
pub type CHAN_RESULT_UPDATED_MIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_result_newvalue_mir(&self) -> CHAN_RESULT_NEWVALUE_MIR_R {
        CHAN_RESULT_NEWVALUE_MIR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn saturate_intr_mir(&self) -> SATURATE_INTR_MIR_R {
        SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RANGE_INTR_MIR_R {
        RANGE_INTR_MIR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub fn chan_result_updated_mir(&self) -> CHAN_RESULT_UPDATED_MIR_R {
        CHAN_RESULT_UPDATED_MIR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
