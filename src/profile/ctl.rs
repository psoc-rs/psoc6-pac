#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN_MODE` reader - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub type WIN_MODE_R = crate::BitReader<bool>;
#[doc = "Field `WIN_MODE` writer - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
pub type WIN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Enables the profiling block: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&self) -> WIN_MODE_R {
        WIN_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&mut self) -> WIN_MODE_W<0> {
        WIN_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Profile control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
