#[doc = "Register `MMMS_ADVCH_NI_ENABLE` reader"]
pub struct R(crate::R<MMMS_ADVCH_NI_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_ADVCH_NI_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_ADVCH_NI_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_ADVCH_NI_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_ADVCH_NI_ENABLE` writer"]
pub struct W(crate::W<MMMS_ADVCH_NI_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_ADVCH_NI_ENABLE_SPEC>;
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
impl From<crate::W<MMMS_ADVCH_NI_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_ADVCH_NI_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_NI_ENABLE` reader - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
pub type ADV_NI_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ADV_NI_ENABLE` writer - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
pub type ADV_NI_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_ADVCH_NI_ENABLE_SPEC, bool, O>;
#[doc = "Field `SCAN_NI_ENABLE` reader - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
pub type SCAN_NI_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_NI_ENABLE` writer - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
pub type SCAN_NI_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_ADVCH_NI_ENABLE_SPEC, bool, O>;
#[doc = "Field `INIT_NI_ENABLE` reader - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
pub type INIT_NI_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INIT_NI_ENABLE` writer - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
pub type INIT_NI_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_ADVCH_NI_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn adv_ni_enable(&self) -> ADV_NI_ENABLE_R {
        ADV_NI_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn scan_ni_enable(&self) -> SCAN_NI_ENABLE_R {
        SCAN_NI_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn init_ni_enable(&self) -> INIT_NI_ENABLE_R {
        INIT_NI_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the Advertisement NI timer and is valid when MMMS_ENABLE=1. 0 - ADV_NI timer is disabled 1 - ADV_NI timer is enabled In this mode, the adv engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn adv_ni_enable(&mut self) -> ADV_NI_ENABLE_W<0> {
        ADV_NI_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - This bit is used to enable the SCAN NI timer and is valid when MMMS_ENABLE=1. 0 - SCAN_NI timer is disabled 1 - SCAN_NI timer is enabled In this mode, the scan engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn scan_ni_enable(&mut self) -> SCAN_NI_ENABLE_W<1> {
        SCAN_NI_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - This bit is used to enable the INIT NI timer and is valid when MMMS_ENABLE=1. 0 - INIT_NI timer is disabled 1 - INIT_NI timer is enabled In this mode, the init engine next instant is scheduled by firmware"]
    #[inline(always)]
    pub fn init_ni_enable(&mut self) -> INIT_NI_ENABLE_W<2> {
        INIT_NI_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable bits for ADV_NI, SCAN_NI and INIT_NI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_enable](index.html) module"]
pub struct MMMS_ADVCH_NI_ENABLE_SPEC;
impl crate::RegisterSpec for MMMS_ADVCH_NI_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_advch_ni_enable::R](R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_enable::W](W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_ADVCH_NI_ENABLE to value 0"]
impl crate::Resettable for MMMS_ADVCH_NI_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
