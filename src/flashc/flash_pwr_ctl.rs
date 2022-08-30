#[doc = "Register `FLASH_PWR_CTL` reader"]
pub struct R(crate::R<FLASH_PWR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PWR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PWR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PWR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_PWR_CTL` writer"]
pub struct W(crate::W<FLASH_PWR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PWR_CTL_SPEC>;
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
impl From<crate::W<FLASH_PWR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PWR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Controls 'enable' pin of the Flash memory."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Controls 'enable' pin of the Flash memory."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PWR_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLE_HV` reader - Controls 'enable_hv' pin of the Flash memory."]
pub type ENABLE_HV_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_HV` writer - Controls 'enable_hv' pin of the Flash memory."]
pub type ENABLE_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_PWR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&self) -> ENABLE_HV_R {
        ENABLE_HV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&mut self) -> ENABLE_HV_W<1> {
        ENABLE_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pwr_ctl](index.html) module"]
pub struct FLASH_PWR_CTL_SPEC;
impl crate::RegisterSpec for FLASH_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_pwr_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_PWR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_pwr_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_PWR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_PWR_CTL to value 0x03"]
impl crate::Resettable for FLASH_PWR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
