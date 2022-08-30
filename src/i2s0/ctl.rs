#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ENABLED` reader - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TX_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `TX_ENABLED` writer - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
pub type TX_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RX_ENABLED` reader - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RX_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `RX_ENABLED` writer - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
pub type RX_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&self) -> TX_ENABLED_R {
        TX_ENABLED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&self) -> RX_ENABLED_R {
        RX_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enables the I2S TX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_enabled(&mut self) -> TX_ENABLED_W<30> {
        TX_ENABLED_W::new(self)
    }
    #[doc = "Bit 31 - Enables the I2S RX component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_enabled(&mut self) -> RX_ENABLED_W<31> {
        RX_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
