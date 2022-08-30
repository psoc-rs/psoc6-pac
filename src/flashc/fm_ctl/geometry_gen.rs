#[doc = "Register `GEOMETRY_GEN` reader"]
pub struct R(crate::R<GEOMETRY_GEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEOMETRY_GEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEOMETRY_GEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEOMETRY_GEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DNU_0X20_1` reader - N/A"]
pub type DNU_0X20_1_R = crate::BitReader<bool>;
#[doc = "Field `DNU_0X20_2` reader - N/A"]
pub type DNU_0X20_2_R = crate::BitReader<bool>;
#[doc = "Field `DNU_0X20_3` reader - N/A"]
pub type DNU_0X20_3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_1(&self) -> DNU_0X20_1_R {
        DNU_0X20_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_2(&self) -> DNU_0X20_2_R {
        DNU_0X20_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn dnu_0x20_3(&self) -> DNU_0X20_3_R {
        DNU_0X20_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "N/A, DNU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry_gen](index.html) module"]
pub struct GEOMETRY_GEN_SPEC;
impl crate::RegisterSpec for GEOMETRY_GEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [geometry_gen::R](R) reader structure"]
impl crate::Readable for GEOMETRY_GEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GEOMETRY_GEN to value 0"]
impl crate::Resettable for GEOMETRY_GEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
