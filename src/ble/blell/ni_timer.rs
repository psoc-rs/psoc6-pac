#[doc = "Register `NI_TIMER` reader"]
pub struct R(crate::R<NI_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NI_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NI_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NI_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NI_TIMER` writer"]
pub struct W(crate::W<NI_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NI_TIMER_SPEC>;
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
impl From<crate::W<NI_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NI_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NI_TIMER` reader - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
pub type NI_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NI_TIMER` writer - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
pub type NI_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NI_TIMER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
    #[inline(always)]
    pub fn ni_timer(&self) -> NI_TIMER_R {
        NI_TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BT Slot at which the next connection has to be serviced, granularity is 625us. The NI timer has to be programmed 1.25ms before the connection event"]
    #[inline(always)]
    pub fn ni_timer(&mut self) -> NI_TIMER_W<0> {
        NI_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Instant Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ni_timer](index.html) module"]
pub struct NI_TIMER_SPEC;
impl crate::RegisterSpec for NI_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ni_timer::R](R) reader structure"]
impl crate::Readable for NI_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ni_timer::W](W) writer structure"]
impl crate::Writable for NI_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NI_TIMER to value 0"]
impl crate::Resettable for NI_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
