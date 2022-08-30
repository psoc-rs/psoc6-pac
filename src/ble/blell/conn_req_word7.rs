#[doc = "Register `CONN_REQ_WORD7` reader"]
pub struct R(crate::R<CONN_REQ_WORD7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD7` writer"]
pub struct W(crate::W<CONN_REQ_WORD7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD7_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUPERVISION_TIMEOUT_VAL` reader - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
pub type SUPERVISION_TIMEOUT_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUPERVISION_TIMEOUT_VAL` writer - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
pub type SUPERVISION_TIMEOUT_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout_val(&self) -> SUPERVISION_TIMEOUT_VAL_R {
        SUPERVISION_TIMEOUT_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout_val(&mut self) -> SUPERVISION_TIMEOUT_VAL_W<0> {
        SUPERVISION_TIMEOUT_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word7](index.html) module"]
pub struct CONN_REQ_WORD7_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word7::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word7::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD7 to value 0"]
impl crate::Resettable for CONN_REQ_WORD7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
