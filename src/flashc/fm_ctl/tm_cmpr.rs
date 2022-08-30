#[doc = "Register `TM_CMPR[%s]` reader"]
pub struct R(crate::R<TM_CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TM_CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TM_CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TM_CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_COMP_RESULT` reader - The result of a comparison between the flash macro data output and the content of the high voltage page latches. The comparison result for a given column 'Column_Number' is updated in this register field on a read to address: 0x100+4*Column_Number. The number of wait states is given by WAIT_CTL.WAIT_FM_HV_RD. '0': FALSE (not equal) '1': TRUE (equal)"]
pub type DATA_COMP_RESULT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The result of a comparison between the flash macro data output and the content of the high voltage page latches. The comparison result for a given column 'Column_Number' is updated in this register field on a read to address: 0x100+4*Column_Number. The number of wait states is given by WAIT_CTL.WAIT_FM_HV_RD. '0': FALSE (not equal) '1': TRUE (equal)"]
    #[inline(always)]
    pub fn data_comp_result(&self) -> DATA_COMP_RESULT_R {
        DATA_COMP_RESULT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Do Not Use\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tm_cmpr](index.html) module"]
pub struct TM_CMPR_SPEC;
impl crate::RegisterSpec for TM_CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tm_cmpr::R](R) reader structure"]
impl crate::Readable for TM_CMPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TM_CMPR[%s]
to value 0"]
impl crate::Resettable for TM_CMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
