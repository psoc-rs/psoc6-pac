#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<8> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device region base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
