#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `RCB_LL_DONE`"]
pub type RCB_LL_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLE_WRITE_DONE`"]
pub type SINGLE_WRITE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLE_READ_DONE`"]
pub type SINGLE_READ_DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rcb_ll_done(&self) -> RCB_LL_DONE_R {
        RCB_LL_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&self) -> SINGLE_WRITE_DONE_R {
        SINGLE_WRITE_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&self) -> SINGLE_READ_DONE_R {
        SINGLE_READ_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
