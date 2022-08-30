#[doc = "Register `WINDOW_WIDEN_INTVL` reader"]
pub struct R(crate::R<WINDOW_WIDEN_INTVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINDOW_WIDEN_INTVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINDOW_WIDEN_INTVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINDOW_WIDEN_INTVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINDOW_WIDEN_INTVL` writer"]
pub struct W(crate::W<WINDOW_WIDEN_INTVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINDOW_WIDEN_INTVL_SPEC>;
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
impl From<crate::W<WINDOW_WIDEN_INTVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINDOW_WIDEN_INTVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINDOW_WIDEN_INTVL` reader - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
pub type WINDOW_WIDEN_INTVL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINDOW_WIDEN_INTVL` writer - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
pub type WINDOW_WIDEN_INTVL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WINDOW_WIDEN_INTVL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
    #[inline(always)]
    pub fn window_widen_intvl(&self) -> WINDOW_WIDEN_INTVL_R {
        WINDOW_WIDEN_INTVL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
    #[inline(always)]
    pub fn window_widen_intvl(&mut self) -> WINDOW_WIDEN_INTVL_W<0> {
        WINDOW_WIDEN_INTVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window widen for interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [window_widen_intvl](index.html) module"]
pub struct WINDOW_WIDEN_INTVL_SPEC;
impl crate::RegisterSpec for WINDOW_WIDEN_INTVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [window_widen_intvl::R](R) reader structure"]
impl crate::Readable for WINDOW_WIDEN_INTVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [window_widen_intvl::W](W) writer structure"]
impl crate::Writable for WINDOW_WIDEN_INTVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINDOW_WIDEN_INTVL to value 0x0a"]
impl crate::Resettable for WINDOW_WIDEN_INTVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
