#[doc = "Reader of register CHAN_WORK[%s]"]
pub type R = crate::R<u32, super::CHAN_WORK>;
#[doc = "Reader of field `WORK`"]
pub type WORK_R = crate::R<u16, u16>;
#[doc = "Reader of field `CHAN_WORK_NEWVALUE_MIR`"]
pub type CHAN_WORK_NEWVALUE_MIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHAN_WORK_UPDATED_MIR`"]
pub type CHAN_WORK_UPDATED_MIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn work(&self) -> WORK_R {
        WORK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_work_newvalue_mir(&self) -> CHAN_WORK_NEWVALUE_MIR_R {
        CHAN_WORK_NEWVALUE_MIR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
    #[inline(always)]
    pub fn chan_work_updated_mir(&self) -> CHAN_WORK_UPDATED_MIR_R {
        CHAN_WORK_UPDATED_MIR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
