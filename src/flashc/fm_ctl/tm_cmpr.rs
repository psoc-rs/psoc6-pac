#[doc = "Reader of register TM_CMPR[%s]"]
pub type R = crate::R<u32, super::TM_CMPR>;
#[doc = "Reader of field `DATA_COMP_RESULT`"]
pub type DATA_COMP_RESULT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - The result of a comparison between the flash macro data output and the content of the high voltage page latches. The comparison result for a given column 'Column_Number' is updated in this register field on a read to address: 0x100+4*Column_Number. The number of wait states is given by WAIT_CTL.WAIT_FM_HV_RD. '0': FALSE (not equal) '1': TRUE (equal)"]
    #[inline(always)]
    pub fn data_comp_result(&self) -> DATA_COMP_RESULT_R {
        DATA_COMP_RESULT_R::new((self.bits & 0x01) != 0)
    }
}
