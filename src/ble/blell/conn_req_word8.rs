#[doc = "Register `CONN_REQ_WORD8` reader"]
pub struct R(crate::R<CONN_REQ_WORD8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD8` writer"]
pub struct W(crate::W<CONN_REQ_WORD8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD8_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CHANNELS_LOWER` reader - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_LOWER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA_CHANNELS_LOWER` writer - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub type DATA_CHANNELS_LOWER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD8_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_lower(&self) -> DATA_CHANNELS_LOWER_R {
        DATA_CHANNELS_LOWER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. 1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_lower(&mut self) -> DATA_CHANNELS_LOWER_W<0> {
        DATA_CHANNELS_LOWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word8](index.html) module"]
pub struct CONN_REQ_WORD8_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word8::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word8::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD8 to value 0"]
impl crate::Resettable for CONN_REQ_WORD8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
