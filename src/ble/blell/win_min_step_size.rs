#[doc = "Register `WIN_MIN_STEP_SIZE` reader"]
pub struct R(crate::R<WIN_MIN_STEP_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIN_MIN_STEP_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIN_MIN_STEP_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIN_MIN_STEP_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIN_MIN_STEP_SIZE` writer"]
pub struct W(crate::W<WIN_MIN_STEP_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIN_MIN_STEP_SIZE_SPEC>;
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
impl From<crate::W<WIN_MIN_STEP_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIN_MIN_STEP_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEPDN` reader - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
pub type STEPDN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPDN` writer - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
pub type STEPDN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIN_MIN_STEP_SIZE_SPEC, u8, u8, 4, O>;
#[doc = "Field `STEPUP` reader - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
pub type STEPUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPUP` writer - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
pub type STEPUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIN_MIN_STEP_SIZE_SPEC, u8, u8, 4, O>;
#[doc = "Field `WINDOW_MIN_FW` reader - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
pub type WINDOW_MIN_FW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WINDOW_MIN_FW` writer - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
pub type WINDOW_MIN_FW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIN_MIN_STEP_SIZE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepdn(&self) -> STEPDN_R {
        STEPDN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepup(&self) -> STEPUP_R {
        STEPUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
    #[inline(always)]
    pub fn window_min_fw(&self) -> WINDOW_MIN_FW_R {
        WINDOW_MIN_FW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepdn(&mut self) -> STEPDN_W<0> {
        STEPDN_W::new(self)
    }
    #[doc = "Bits 4:7 - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepup(&mut self) -> STEPUP_W<4> {
        STEPUP_W::new(self)
    }
    #[doc = "Bits 8:15 - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
    #[inline(always)]
    pub fn window_min_fw(&mut self) -> WINDOW_MIN_FW_W<8> {
        WINDOW_MIN_FW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window minimum step size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win_min_step_size](index.html) module"]
pub struct WIN_MIN_STEP_SIZE_SPEC;
impl crate::RegisterSpec for WIN_MIN_STEP_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [win_min_step_size::R](R) reader structure"]
impl crate::Readable for WIN_MIN_STEP_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [win_min_step_size::W](W) writer structure"]
impl crate::Writable for WIN_MIN_STEP_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIN_MIN_STEP_SIZE to value 0x2064"]
impl crate::Resettable for WIN_MIN_STEP_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2064
    }
}
