#[doc = "Register `ADDR_CTL` reader"]
pub struct R(crate::R<ADDR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_CTL` writer"]
pub struct W(crate::W<ADDR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_CTL_SPEC>;
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
impl From<crate::W<ADDR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE2` reader - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type SIZE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE2` writer - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type SIZE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIV2` reader - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_R = crate::BitReader<bool>;
#[doc = "Field `DIV2` writer - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
pub type DIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn size2(&self) -> SIZE2_R {
        SIZE2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies the size of the XIP device address in Bytes: '0': 1 Byte address. '1': 2 Byte address. '2': 3 Byte address. '3': 4 Byte address. The lower significant address Bytes of the transfer request are used as XIP address to the external device. Note that for dual quad SPI data transfer, the transfer request address is divided by 2. Therefore, the transfer request address needs to be a multiple of 2. If the trasnfer requestaddress is NOT a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn size2(&mut self) -> SIZE2_W<0> {
        SIZE2_W::new(self)
    }
    #[doc = "Bit 8 - Specifies if the AHB-Lite bus transfer address is divided by 2 or not: '0': No divide by 2. '1': Divide by 2. This functionality is used for read and write operation in XIP, dual quad SPI mode; i.e. this DIV2 must be set to '1' in dual quad SPI mode. If the transfer request address is NOT a multiple of 2 or the requested number of Bytes is not a multiple of 2, the XIP_ALIGNMENT_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn div2(&mut self) -> DIV2_W<8> {
        DIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_ctl](index.html) module"]
pub struct ADDR_CTL_SPEC;
impl crate::RegisterSpec for ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_ctl::R](R) reader structure"]
impl crate::Readable for ADDR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_ctl::W](W) writer structure"]
impl crate::Writable for ADDR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR_CTL to value 0"]
impl crate::Resettable for ADDR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
