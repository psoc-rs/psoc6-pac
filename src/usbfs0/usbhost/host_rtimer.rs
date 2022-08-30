#[doc = "Register `HOST_RTIMER` reader"]
pub struct R(crate::R<HOST_RTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_RTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_RTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_RTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_RTIMER` writer"]
pub struct W(crate::W<HOST_RTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_RTIMER_SPEC>;
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
impl From<crate::W<HOST_RTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_RTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTIMER` reader - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RTIMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTIMER` writer - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RTIMER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_RTIMER_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&self) -> RTIMER_R {
        RTIMER_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing is ended. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&mut self) -> RTIMER_W<0> {
        RTIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Retry Timer Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_rtimer](index.html) module"]
pub struct HOST_RTIMER_SPEC;
impl crate::RegisterSpec for HOST_RTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_rtimer::R](R) reader structure"]
impl crate::Readable for HOST_RTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_rtimer::W](W) writer structure"]
impl crate::Writable for HOST_RTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_RTIMER to value 0"]
impl crate::Resettable for HOST_RTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
