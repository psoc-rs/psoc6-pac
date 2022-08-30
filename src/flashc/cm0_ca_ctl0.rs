#[doc = "Register `CM0_CA_CTL0` reader"]
pub struct R(crate::R<CM0_CA_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CA_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CA_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CA_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_CA_CTL0` writer"]
pub struct W(crate::W<CM0_CA_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_CA_CTL0_SPEC>;
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
impl From<crate::W<CM0_CA_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_CA_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAY` reader - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAY` writer - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_CA_CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SET_ADDR` reader - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SET_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SET_ADDR` writer - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SET_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM0_CA_CTL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM0_CA_CTL0_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM0_CA_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&mut self) -> WAY_W<16> {
        WAY_W::new(self)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&mut self) -> SET_ADDR_W<24> {
        SET_ADDR_W::new(self)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W<30> {
        PREF_EN_W::new(self)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
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
#[doc = "CM0+ cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_ca_ctl0](index.html) module"]
pub struct CM0_CA_CTL0_SPEC;
impl crate::RegisterSpec for CM0_CA_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_ca_ctl0::R](R) reader structure"]
impl crate::Readable for CM0_CA_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_ca_ctl0::W](W) writer structure"]
impl crate::Writable for CM0_CA_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_CA_CTL0 to value 0xc000_0000"]
impl crate::Resettable for CM0_CA_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
