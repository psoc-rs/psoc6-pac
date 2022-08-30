#[doc = "Register `CLK_IMO_CONFIG` reader"]
pub struct R(crate::R<CLK_IMO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_CONFIG` writer"]
pub struct W(crate::W<CLK_IMO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_IMO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_IMO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_config](index.html) module"]
pub struct CLK_IMO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_IMO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_config::R](R) reader structure"]
impl crate::Readable for CLK_IMO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_config::W](W) writer structure"]
impl crate::Writable for CLK_IMO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_IMO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
