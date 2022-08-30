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
#[doc = "Field `SUBREGION_DISABLE` reader - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
pub type SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR24` reader - This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "SMPU region address 1 (master structure)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](index.html) module"]
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
