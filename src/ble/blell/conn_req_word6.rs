#[doc = "Register `CONN_REQ_WORD6` reader"]
pub struct R(crate::R<CONN_REQ_WORD6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD6` writer"]
pub struct W(crate::W<CONN_REQ_WORD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD6_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_LATENCY_VAL` reader - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
pub type SLAVE_LATENCY_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLAVE_LATENCY_VAL` writer - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
pub type SLAVE_LATENCY_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency_val(&self) -> SLAVE_LATENCY_VAL_R {
        SLAVE_LATENCY_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency_val(&mut self) -> SLAVE_LATENCY_VAL_W<0> {
        SLAVE_LATENCY_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word6](index.html) module"]
pub struct CONN_REQ_WORD6_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word6::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word6::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD6 to value 0"]
impl crate::Resettable for CONN_REQ_WORD6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
