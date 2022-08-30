#[doc = "Register `GEOMETRY` reader"]
pub struct R(crate::R<GEOMETRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEOMETRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEOMETRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEOMETRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WORD_SIZE_LOG2` reader - Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '7': 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
pub type WORD_SIZE_LOG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAGE_SIZE_LOG2` reader - Number of Bytes per page (log 2): '0': 1 Byte '1': 2 Bytes '2': 4 Bytes ... '15': 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
pub type PAGE_SIZE_LOG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROW_COUNT` reader - Number of rows (minus 1): '0': 1 row '1': 2 rows '2': 3 rows ... '65535': 65536 rows"]
pub type ROW_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BANK_COUNT` reader - Number of banks (minus 1): '0': 1 bank '1': 2 banks ... '255': 256 banks"]
pub type BANK_COUNT_R = crate::FieldReader<u8, u8>;
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
#[doc = "Regular flash geometry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry](index.html) module"]
pub struct GEOMETRY_SPEC;
impl crate::RegisterSpec for GEOMETRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [geometry::R](R) reader structure"]
impl crate::Readable for GEOMETRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GEOMETRY to value 0"]
impl crate::Resettable for GEOMETRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
