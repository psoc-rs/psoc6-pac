#[doc = "Reader of register CHAN_WORK_NEWVALUE"]
pub type R = crate::R<u32, super::CHAN_WORK_NEWVALUE>;
#[doc = "Reader of field `CHAN_WORK_NEWVALUE`"]
pub type CHAN_WORK_NEWVALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding WORK data received a new value, i.e. was already sampled during the current scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub fn chan_work_newvalue(&self) -> CHAN_WORK_NEWVALUE_R {
        CHAN_WORK_NEWVALUE_R::new((self.bits & 0xffff) as u16)
    }
}
