#[doc = "Register `PDU_ACCESS_ADDR_H_REGISTER` reader"]
pub struct R(crate::R<PDU_ACCESS_ADDR_H_REGISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDU_ACCESS_ADDR_H_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDU_ACCESS_ADDR_H_REGISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDU_ACCESS_ADDR_H_REGISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDU_ACCESS_ADDR_H_REGISTER` writer"]
pub struct W(crate::W<PDU_ACCESS_ADDR_H_REGISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDU_ACCESS_ADDR_H_REGISTER_SPEC>;
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
impl From<crate::W<PDU_ACCESS_ADDR_H_REGISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDU_ACCESS_ADDR_H_REGISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDU_ACCESS_ADDRESS_HIGHER_BITS` reader - This field defines the higher 16 bits of the access address for each Link layer connection between any two devices."]
pub type PDU_ACCESS_ADDRESS_HIGHER_BITS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PDU_ACCESS_ADDRESS_HIGHER_BITS` writer - This field defines the higher 16 bits of the access address for each Link layer connection between any two devices."]
pub type PDU_ACCESS_ADDRESS_HIGHER_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDU_ACCESS_ADDR_H_REGISTER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field defines the higher 16 bits of the access address for each Link layer connection between any two devices."]
    #[inline(always)]
    pub fn pdu_access_address_higher_bits(&self) -> PDU_ACCESS_ADDRESS_HIGHER_BITS_R {
        PDU_ACCESS_ADDRESS_HIGHER_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the higher 16 bits of the access address for each Link layer connection between any two devices."]
    #[inline(always)]
    pub fn pdu_access_address_higher_bits(&mut self) -> PDU_ACCESS_ADDRESS_HIGHER_BITS_W<0> {
        PDU_ACCESS_ADDRESS_HIGHER_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access address (upper)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdu_access_addr_h_register](index.html) module"]
pub struct PDU_ACCESS_ADDR_H_REGISTER_SPEC;
impl crate::RegisterSpec for PDU_ACCESS_ADDR_H_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdu_access_addr_h_register::R](R) reader structure"]
impl crate::Readable for PDU_ACCESS_ADDR_H_REGISTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdu_access_addr_h_register::W](W) writer structure"]
impl crate::Writable for PDU_ACCESS_ADDR_H_REGISTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDU_ACCESS_ADDR_H_REGISTER to value 0"]
impl crate::Resettable for PDU_ACCESS_ADDR_H_REGISTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
