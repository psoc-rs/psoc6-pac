#[doc = "Register `ADDR1` reader"]
pub struct R(crate::R<ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUBREGION_DISABLE` reader - See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
pub type SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR24` reader - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - See corresponding field for PPU structure with programmable address. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "PPU region address 1 (master structure)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](index.html) module"]
pub struct ADDR1_SPEC;
impl crate::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr1::R](R) reader structure"]
impl crate::Readable for ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
