#[doc = "Register `SLAVE_LATENCY` reader"]
pub struct R(crate::R<SLAVE_LATENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_LATENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_LATENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_LATENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_LATENCY` writer"]
pub struct W(crate::W<SLAVE_LATENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_LATENCY_SPEC>;
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
impl From<crate::W<SLAVE_LATENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_LATENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_LATENCY` reader - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
pub type SLAVE_LATENCY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLAVE_LATENCY` writer - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
pub type SLAVE_LATENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE_LATENCY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency(&self) -> SLAVE_LATENCY_R {
        SLAVE_LATENCY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value configured in this field defines the number of consecutive connection events that the slave device is not required to listen for master. The value of connSlaveLatency should not cause a Supervision Timeout. This shall be an integer in the range of 0 to ((connSupervision Timeout/connInterval)-1). connSlaveLatency shall also be less than 500."]
    #[inline(always)]
    pub fn slave_latency(&mut self) -> SLAVE_LATENCY_W<0> {
        SLAVE_LATENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Latency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_latency](index.html) module"]
pub struct SLAVE_LATENCY_SPEC;
impl crate::RegisterSpec for SLAVE_LATENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_latency::R](R) reader structure"]
impl crate::Readable for SLAVE_LATENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_latency::W](W) writer structure"]
impl crate::Writable for SLAVE_LATENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE_LATENCY to value 0"]
impl crate::Resettable for SLAVE_LATENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
