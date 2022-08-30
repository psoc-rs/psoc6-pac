#[doc = "Register `SLAVE_TIMING_CONTROL` reader"]
pub struct R(crate::R<SLAVE_TIMING_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_TIMING_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_TIMING_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_TIMING_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_TIMING_CONTROL` writer"]
pub struct W(crate::W<SLAVE_TIMING_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_TIMING_CONTROL_SPEC>;
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
impl From<crate::W<SLAVE_TIMING_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_TIMING_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_TIME_SET_VAL` reader - Programmable adjust value to the clock counter when slave is connected"]
pub type SLAVE_TIME_SET_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_TIME_SET_VAL` writer - Programmable adjust value to the clock counter when slave is connected"]
pub type SLAVE_TIME_SET_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE_TIMING_CONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLAVE_TIME_ADJ_VAL` reader - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
pub type SLAVE_TIME_ADJ_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_TIME_ADJ_VAL` writer - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
pub type SLAVE_TIME_ADJ_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLAVE_TIMING_CONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Programmable adjust value to the clock counter when slave is connected"]
    #[inline(always)]
    pub fn slave_time_set_val(&self) -> SLAVE_TIME_SET_VAL_R {
        SLAVE_TIME_SET_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
    #[inline(always)]
    pub fn slave_time_adj_val(&self) -> SLAVE_TIME_ADJ_VAL_R {
        SLAVE_TIME_ADJ_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable adjust value to the clock counter when slave is connected"]
    #[inline(always)]
    pub fn slave_time_set_val(&mut self) -> SLAVE_TIME_SET_VAL_W<0> {
        SLAVE_TIME_SET_VAL_W::new(self)
    }
    #[doc = "Bits 8:15 - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
    #[inline(always)]
    pub fn slave_time_adj_val(&mut self) -> SLAVE_TIME_ADJ_VAL_W<8> {
        SLAVE_TIME_ADJ_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave timing control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_timing_control](index.html) module"]
pub struct SLAVE_TIMING_CONTROL_SPEC;
impl crate::RegisterSpec for SLAVE_TIMING_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_timing_control::R](R) reader structure"]
impl crate::Readable for SLAVE_TIMING_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_timing_control::W](W) writer structure"]
impl crate::Writable for SLAVE_TIMING_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE_TIMING_CONTROL to value 0xbe96"]
impl crate::Resettable for SLAVE_TIMING_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbe96
    }
}
