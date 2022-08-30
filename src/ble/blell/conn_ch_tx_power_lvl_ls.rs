#[doc = "Register `CONN_CH_TX_POWER_LVL_LS` reader"]
pub struct R(crate::R<CONN_CH_TX_POWER_LVL_LS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CH_TX_POWER_LVL_LS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CH_TX_POWER_LVL_LS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CH_TX_POWER_LVL_LS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CH_TX_POWER_LVL_LS` writer"]
pub struct W(crate::W<CONN_CH_TX_POWER_LVL_LS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CH_TX_POWER_LVL_LS_SPEC>;
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
impl From<crate::W<CONN_CH_TX_POWER_LVL_LS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CH_TX_POWER_LVL_LS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONNCH_TRANSMIT_POWER_LVL_LS` reader - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Connection channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Connection channel transmit power code 4 bits."]
pub type CONNCH_TRANSMIT_POWER_LVL_LS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONNCH_TRANSMIT_POWER_LVL_LS` writer - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Connection channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Connection channel transmit power code 4 bits."]
pub type CONNCH_TRANSMIT_POWER_LVL_LS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CH_TX_POWER_LVL_LS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Connection channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Connection channel transmit power code 4 bits."]
    #[inline(always)]
    pub fn connch_transmit_power_lvl_ls(&self) -> CONNCH_TRANSMIT_POWER_LVL_LS_R {
        CONNCH_TRANSMIT_POWER_LVL_LS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Connection channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Connection channel transmit power code 4 bits."]
    #[inline(always)]
    pub fn connch_transmit_power_lvl_ls(&mut self) -> CONNCH_TRANSMIT_POWER_LVL_LS_W<0> {
        CONNCH_TRANSMIT_POWER_LVL_LS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection channel transmit power setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ch_tx_power_lvl_ls](index.html) module"]
pub struct CONN_CH_TX_POWER_LVL_LS_SPEC;
impl crate::RegisterSpec for CONN_CH_TX_POWER_LVL_LS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ch_tx_power_lvl_ls::R](R) reader structure"]
impl crate::Readable for CONN_CH_TX_POWER_LVL_LS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_ch_tx_power_lvl_ls::W](W) writer structure"]
impl crate::Writable for CONN_CH_TX_POWER_LVL_LS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CH_TX_POWER_LVL_LS to value 0"]
impl crate::Resettable for CONN_CH_TX_POWER_LVL_LS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
