#[doc = "Reader of register DATA[%s]"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Captured fault source data. Note: the fault source index STATUS.IDX specifies the format of the DATA registers."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
