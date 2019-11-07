#[doc = "Reader of register MMMS_SLAVE_CREATE_US_CAPT"]
pub type R = crate::R<u32, super::MMMS_SLAVE_CREATE_US_CAPT>;
#[doc = "Reader of field `US_OFFSET_SLAVE_CREATED`"]
pub type US_OFFSET_SLAVE_CREATED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register captures the us when slave connection is created, granularity is 1us"]
    #[inline(always)]
    pub fn us_offset_slave_created(&self) -> US_OFFSET_SLAVE_CREATED_R {
        US_OFFSET_SLAVE_CREATED_R::new((self.bits & 0xffff) as u16)
    }
}
