#[doc = "Register `PENDING1` reader"]
pub struct R(crate::R<PENDING1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 18: Flash controller, main interface, bus error."]
pub type SOURCE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 18: Flash controller, main interface, bus error."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
#[doc = "Fault pending 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending1](index.html) module"]
pub struct PENDING1_SPEC;
impl crate::RegisterSpec for PENDING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending1::R](R) reader structure"]
impl crate::Readable for PENDING1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDING1 to value 0"]
impl crate::Resettable for PENDING1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
