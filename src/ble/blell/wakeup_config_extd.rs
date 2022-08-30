#[doc = "Register `WAKEUP_CONFIG_EXTD` reader"]
pub struct R(crate::R<WAKEUP_CONFIG_EXTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_CONFIG_EXTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_CONFIG_EXTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_CONFIG_EXTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_CONFIG_EXTD` writer"]
pub struct W(crate::W<WAKEUP_CONFIG_EXTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_CONFIG_EXTD_SPEC>;
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
impl From<crate::W<WAKEUP_CONFIG_EXTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_CONFIG_EXTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSM_LF_OFFSET` reader - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
pub type DSM_LF_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSM_LF_OFFSET` writer - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
pub type DSM_LF_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEUP_CONFIG_EXTD_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
    #[inline(always)]
    pub fn dsm_lf_offset(&self) -> DSM_LF_OFFSET_R {
        DSM_LF_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
    #[inline(always)]
    pub fn dsm_lf_offset(&mut self) -> DSM_LF_OFFSET_W<0> {
        DSM_LF_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup configuration extended\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_config_extd](index.html) module"]
pub struct WAKEUP_CONFIG_EXTD_SPEC;
impl crate::RegisterSpec for WAKEUP_CONFIG_EXTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_config_extd::R](R) reader structure"]
impl crate::Readable for WAKEUP_CONFIG_EXTD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_config_extd::W](W) writer structure"]
impl crate::Writable for WAKEUP_CONFIG_EXTD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_CONFIG_EXTD to value 0"]
impl crate::Resettable for WAKEUP_CONFIG_EXTD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
