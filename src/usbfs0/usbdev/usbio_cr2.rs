#[doc = "Register `USBIO_CR2` reader"]
pub struct R(crate::R<USBIO_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIO_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIO_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIO_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIO_CR2` writer"]
pub struct W(crate::W<USBIO_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIO_CR2_SPEC>;
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
impl From<crate::W<USBIO_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIO_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD_5_0` reader - N/A"]
pub type RSVD_5_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_PKT` reader - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TEST_PKT_R = crate::BitReader<bool>;
#[doc = "Field `TEST_PKT` writer - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TEST_PKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBIO_CR2_SPEC, bool, O>;
#[doc = "Field `RSVD_7` reader - N/A"]
pub type RSVD_7_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_7` writer - N/A"]
pub type RSVD_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBIO_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5_0(&self) -> RSVD_5_0_R {
        RSVD_5_0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TEST_PKT_R {
        TEST_PKT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&mut self) -> TEST_PKT_W<6> {
        TEST_PKT_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> RSVD_7_W<7> {
        RSVD_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBIO control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr2](index.html) module"]
pub struct USBIO_CR2_SPEC;
impl crate::RegisterSpec for USBIO_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbio_cr2::R](R) reader structure"]
impl crate::Readable for USBIO_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbio_cr2::W](W) writer structure"]
impl crate::Writable for USBIO_CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIO_CR2 to value 0"]
impl crate::Resettable for USBIO_CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
