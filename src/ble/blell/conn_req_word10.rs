#[doc = "Register `CONN_REQ_WORD10` reader"]
pub struct R(crate::R<CONN_REQ_WORD10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD10` writer"]
pub struct W(crate::W<CONN_REQ_WORD10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD10_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CHANNELS_UPPER` reader - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_UPPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_CHANNELS_UPPER` writer - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_UPPER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD10_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_upper(&self) -> DATA_CHANNELS_UPPER_R {
        DATA_CHANNELS_UPPER_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_upper(&mut self) -> DATA_CHANNELS_UPPER_W<0> {
        DATA_CHANNELS_UPPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word10](index.html) module"]
pub struct CONN_REQ_WORD10_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word10::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word10::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD10 to value 0"]
impl crate::Resettable for CONN_REQ_WORD10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
