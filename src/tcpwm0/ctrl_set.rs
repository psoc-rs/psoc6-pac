#[doc = "Register `CTRL_SET` reader"]
pub struct R(crate::R<CTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_SET` writer"]
pub struct W(crate::W<CTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SET_SPEC>;
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
impl From<crate::W<CTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_ENABLED` reader - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type COUNTER_ENABLED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNTER_ENABLED` writer - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type COUNTER_ENABLED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> COUNTER_ENABLED_R {
        COUNTER_ENABLED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows enabling of counters. A write access: '0': Does nothing. '1': Sets respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&mut self) -> COUNTER_ENABLED_W<0> {
        COUNTER_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM control set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](index.html) module"]
pub struct CTRL_SET_SPEC;
impl crate::RegisterSpec for CTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_set::R](R) reader structure"]
impl crate::Readable for CTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](W) writer structure"]
impl crate::Writable for CTRL_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_SET to value 0"]
impl crate::Resettable for CTRL_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
