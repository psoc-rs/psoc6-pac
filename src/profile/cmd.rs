#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_TR` reader - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type START_TR_R = crate::BitReader<bool>;
#[doc = "Field `START_TR` writer - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type START_TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STOP_TR` reader - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type STOP_TR_R = crate::BitReader<bool>;
#[doc = "Field `STOP_TR` writer - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type STOP_TR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLR_ALL_CNT` reader - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type CLR_ALL_CNT_R = crate::BitReader<bool>;
#[doc = "Field `CLR_ALL_CNT` writer - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type CLR_ALL_CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&self) -> START_TR_R {
        START_TR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&self) -> STOP_TR_R {
        STOP_TR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&self) -> CLR_ALL_CNT_R {
        CLR_ALL_CNT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&mut self) -> START_TR_W<0> {
        START_TR_W::new(self)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&mut self) -> STOP_TR_W<1> {
        STOP_TR_W::new(self)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&mut self) -> CLR_ALL_CNT_W<8> {
        CLR_ALL_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Profile command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
