#[doc = "Register `ADV_CH_TX_POWER_LVL_MS` reader"]
pub struct R(crate::R<ADV_CH_TX_POWER_LVL_MS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_CH_TX_POWER_LVL_MS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_CH_TX_POWER_LVL_MS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_CH_TX_POWER_LVL_MS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_CH_TX_POWER_LVL_MS` writer"]
pub struct W(crate::W<ADV_CH_TX_POWER_LVL_MS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_CH_TX_POWER_LVL_MS_SPEC>;
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
impl From<crate::W<ADV_CH_TX_POWER_LVL_MS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_CH_TX_POWER_LVL_MS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_TRANSMIT_POWER_LVL_MS` reader - Advertising channel transmit power setting Most Significant 2 bits."]
pub type ADV_TRANSMIT_POWER_LVL_MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADV_TRANSMIT_POWER_LVL_MS` writer - Advertising channel transmit power setting Most Significant 2 bits."]
pub type ADV_TRANSMIT_POWER_LVL_MS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADV_CH_TX_POWER_LVL_MS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Advertising channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ms(&self) -> ADV_TRANSMIT_POWER_LVL_MS_R {
        ADV_TRANSMIT_POWER_LVL_MS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Advertising channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ms(&mut self) -> ADV_TRANSMIT_POWER_LVL_MS_W<0> {
        ADV_TRANSMIT_POWER_LVL_MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertising channel transmit power setting extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_ch_tx_power_lvl_ms](index.html) module"]
pub struct ADV_CH_TX_POWER_LVL_MS_SPEC;
impl crate::RegisterSpec for ADV_CH_TX_POWER_LVL_MS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_ch_tx_power_lvl_ms::R](R) reader structure"]
impl crate::Readable for ADV_CH_TX_POWER_LVL_MS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_ch_tx_power_lvl_ms::W](W) writer structure"]
impl crate::Writable for ADV_CH_TX_POWER_LVL_MS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_CH_TX_POWER_LVL_MS to value 0"]
impl crate::Resettable for ADV_CH_TX_POWER_LVL_MS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
