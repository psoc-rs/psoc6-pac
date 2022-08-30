#[doc = "Register `BIST_ADDR` reader"]
pub struct R(crate::R<BIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COL_ADDR` reader - Current column address."]
pub type COL_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROW_ADDR` reader - Current row address."]
pub type ROW_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current column address."]
    #[inline(always)]
    pub fn col_addr(&self) -> COL_ADDR_R {
        COL_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current row address."]
    #[inline(always)]
    pub fn row_addr(&self) -> ROW_ADDR_R {
        ROW_ADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "BIST address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_addr](index.html) module"]
pub struct BIST_ADDR_SPEC;
impl crate::RegisterSpec for BIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_addr::R](R) reader structure"]
impl crate::Readable for BIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BIST_ADDR to value 0"]
impl crate::Resettable for BIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
