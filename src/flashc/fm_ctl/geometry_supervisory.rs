#[doc = "Reader of register GEOMETRY_SUPERVISORY"]
pub type R = crate::R<u32, super::GEOMETRY_SUPERVISORY>;
#[doc = "Reader of field `WORD_SIZE_LOG2`"]
pub type WORD_SIZE_LOG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `PAGE_SIZE_LOG2`"]
pub type PAGE_SIZE_LOG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `ROW_COUNT`"]
pub type ROW_COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `BANK_COUNT`"]
pub type BANK_COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub fn word_size_log2(&self) -> WORD_SIZE_LOG2_R {
        WORD_SIZE_LOG2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub fn page_size_log2(&self) -> PAGE_SIZE_LOG2_R {
        PAGE_SIZE_LOG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub fn row_count(&self) -> ROW_COUNT_R {
        ROW_COUNT_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub fn bank_count(&self) -> BANK_COUNT_R {
        BANK_COUNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
