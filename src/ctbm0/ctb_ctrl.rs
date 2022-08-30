#[doc = "Register `CTB_CTRL` reader"]
pub struct R(crate::R<CTB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTB_CTRL` writer"]
pub struct W(crate::W<CTB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTB_CTRL_SPEC>;
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
impl From<crate::W<CTB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTB_CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTB_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W<30> {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
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
#[doc = "global CTB and power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_ctrl](index.html) module"]
pub struct CTB_CTRL_SPEC;
impl crate::RegisterSpec for CTB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctb_ctrl::R](R) reader structure"]
impl crate::Readable for CTB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctb_ctrl::W](W) writer structure"]
impl crate::Writable for CTB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTB_CTRL to value 0"]
impl crate::Resettable for CTB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
