#[doc = "Register `EFUSE_RDATA_L` reader"]
pub struct R(crate::R<EFUSE_RDATA_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_RDATA_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_RDATA_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_RDATA_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - This register has the read value from the Efuse macro, fuse bits\\[31:0\\]"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register has the read value from the Efuse macro, fuse bits\\[31:0\\]"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "EFUSE Lower read data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_rdata_l](index.html) module"]
pub struct EFUSE_RDATA_L_SPEC;
impl crate::RegisterSpec for EFUSE_RDATA_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_rdata_l::R](R) reader structure"]
impl crate::Readable for EFUSE_RDATA_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EFUSE_RDATA_L to value 0"]
impl crate::Resettable for EFUSE_RDATA_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
