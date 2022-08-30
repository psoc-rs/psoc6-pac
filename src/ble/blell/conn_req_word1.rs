#[doc = "Register `CONN_REQ_WORD1` reader"]
pub struct R(crate::R<CONN_REQ_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD1` writer"]
pub struct W(crate::W<CONN_REQ_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD1_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACCESS_ADDR_UPPER` reader - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
pub type ACCESS_ADDR_UPPER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACCESS_ADDR_UPPER` writer - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
pub type ACCESS_ADDR_UPPER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn access_addr_upper(&self) -> ACCESS_ADDR_UPPER_R {
        ACCESS_ADDR_UPPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the upper16 bits of the access address that is to be sent in the connect request packet of the initiator."]
    #[inline(always)]
    pub fn access_addr_upper(&mut self) -> ACCESS_ADDR_UPPER_W<0> {
        ACCESS_ADDR_UPPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word1](index.html) module"]
pub struct CONN_REQ_WORD1_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word1::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word1::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD1 to value 0"]
impl crate::Resettable for CONN_REQ_WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
