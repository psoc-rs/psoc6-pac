#[doc = "Register `ADDR0` reader"]
pub struct R(crate::R<ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUBREGION_DISABLE` reader - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
pub type SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR24` reader - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': address of protected region. Note: this field is read-only. Its value is chip specific."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - See corresponding field for PPU structure with programmable address. 'ADDR_DEF1': address of protected region. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "PPU region address 0 (slave structure)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](index.html) module"]
pub struct ADDR0_SPEC;
impl crate::RegisterSpec for ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr0::R](R) reader structure"]
impl crate::Readable for ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
