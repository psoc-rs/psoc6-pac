#[doc = "Register `CONN_PARAM_ACC_WIN_WIDEN` reader"]
pub struct R(crate::R<CONN_PARAM_ACC_WIN_WIDEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_PARAM_ACC_WIN_WIDEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_PARAM_ACC_WIN_WIDEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_PARAM_ACC_WIN_WIDEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_PARAM_ACC_WIN_WIDEN` writer"]
pub struct W(crate::W<CONN_PARAM_ACC_WIN_WIDEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_PARAM_ACC_WIN_WIDEN_SPEC>;
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
impl From<crate::W<CONN_PARAM_ACC_WIN_WIDEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_PARAM_ACC_WIN_WIDEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACC_WINDOW_WIDEN` reader - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
pub type ACC_WINDOW_WIDEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACC_WINDOW_WIDEN` writer - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
pub type ACC_WINDOW_WIDEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_PARAM_ACC_WIN_WIDEN_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn acc_window_widen(&self) -> ACC_WINDOW_WIDEN_R {
        ACC_WINDOW_WIDEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn acc_window_widen(&mut self) -> ACC_WINDOW_WIDEN_W<0> {
        ACC_WINDOW_WIDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to configure Accumulated window widening for next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param_acc_win_widen](index.html) module"]
pub struct CONN_PARAM_ACC_WIN_WIDEN_SPEC;
impl crate::RegisterSpec for CONN_PARAM_ACC_WIN_WIDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_param_acc_win_widen::R](R) reader structure"]
impl crate::Readable for CONN_PARAM_ACC_WIN_WIDEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_param_acc_win_widen::W](W) writer structure"]
impl crate::Writable for CONN_PARAM_ACC_WIN_WIDEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_PARAM_ACC_WIN_WIDEN to value 0"]
impl crate::Resettable for CONN_PARAM_ACC_WIN_WIDEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
