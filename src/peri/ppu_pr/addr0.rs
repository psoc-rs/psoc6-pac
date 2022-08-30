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
#[doc = "Register `ADDR0` writer"]
pub struct W(crate::W<ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBREGION_DISABLE` reader - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBREGION_DISABLE` writer - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
pub type SUBREGION_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR24` reader - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR24` writer - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
pub type ADDR24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR0_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SUBREGION_DISABLE_R {
        SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. E.g., a 64 KByte address region (REGION_SIZE is '15') has eight 8 KByte subregions. The access control as defined by PPU_REGION_ATT applies if the bus transfer address is within the address region AND the addressed subregion is NOT disabled. Note that the smallest region size is 256 B and the smallest subregion size is 32 B."]
    #[inline(always)]
    pub fn subregion_disable(&mut self) -> SUBREGION_DISABLE_W<0> {
        SUBREGION_DISABLE_W::new(self)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. The region size is defined by PPU_REGION_ATT.REGION_SIZE. A region of n Byte is always n Byte aligned. As a result, some of the lesser significant address bits of ADDR24 may be ignored in determining whether a bus transfer address is within an address region. E.g., a 64 KByte address region (REGION_SIZE is '15') is 64 KByte aligned, and ADDR24\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn addr24(&mut self) -> ADDR24_W<8> {
        ADDR24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU region address 0 (slave structure)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](index.html) module"]
pub struct ADDR0_SPEC;
impl crate::RegisterSpec for ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr0::R](R) reader structure"]
impl crate::Readable for ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr0::W](W) writer structure"]
impl crate::Writable for ADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
