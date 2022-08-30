#[doc = "Register `PWR_TRIM_WAKE_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_WAKE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_WAKE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_WAKE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_WAKE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_WAKE_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_WAKE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_WAKE_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_WAKE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_WAKE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_DELAY` reader - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WAKE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAKE_DELAY` writer - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WAKE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_WAKE_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&self) -> WAKE_DELAY_R {
        WAKE_DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&mut self) -> WAKE_DELAY_W<0> {
        WAKE_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_wake_ctl](index.html) module"]
pub struct PWR_TRIM_WAKE_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_WAKE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_wake_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_WAKE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_wake_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_WAKE_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_WAKE_CTL to value 0"]
impl crate::Resettable for PWR_TRIM_WAKE_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
