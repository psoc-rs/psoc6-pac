#[doc = "Reader of register GEOMETRY"]
pub type R = crate::R<u32, super::GEOMETRY>;
#[doc = "Reader of field `WORD_SIZE_LOG2`"]
pub type WORD_SIZE_LOG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `PAGE_SIZE_LOG2`"]
pub type PAGE_SIZE_LOG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `ROW_COUNT`"]
pub type ROW_COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `BANK_COUNT`"]
pub type BANK_COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '7': 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub fn word_size_log2(&self) -> WORD_SIZE_LOG2_R {
        WORD_SIZE_LOG2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Bytes per page (log 2): '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '15': 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub fn page_size_log2(&self) -> PAGE_SIZE_LOG2_R {
        PAGE_SIZE_LOG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Number of rows (minus 1): '0': 1 row '1': 2 rows '2': 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub fn row_count(&self) -> ROW_COUNT_R {
        ROW_COUNT_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Number of banks (minus 1): '0': 1 bank '1': 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub fn bank_count(&self) -> BANK_COUNT_R {
        BANK_COUNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
