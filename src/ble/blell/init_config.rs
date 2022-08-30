#[doc = "Register `INIT_CONFIG` reader"]
pub struct R(crate::R<INIT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_CONFIG` writer"]
pub struct W(crate::W<INIT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_CONFIG_SPEC>;
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
impl From<crate::W<INIT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_STRT_EN` reader - Enable Initiator event start interrupt."]
pub type INIT_STRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INIT_STRT_EN` writer - Enable Initiator event start interrupt."]
pub type INIT_STRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `INIT_CLOSE_EN` reader - Enable Initiator event close interrupt."]
pub type INIT_CLOSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `INIT_CLOSE_EN` writer - Enable Initiator event close interrupt."]
pub type INIT_CLOSE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `CONN_REQ_TX_EN` reader - Enables connection request packet transmission start interrupt."]
pub type CONN_REQ_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONN_REQ_TX_EN` writer - Enables connection request packet transmission start interrupt."]
pub type CONN_REQ_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `CONN_CREATED` reader - Enable master connection created interrupt"]
pub type CONN_CREATED_R = crate::BitReader<bool>;
#[doc = "Field `CONN_CREATED` writer - Enable master connection created interrupt"]
pub type CONN_CREATED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN` reader - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R = crate::BitReader<bool>;
#[doc = "Field `INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN` writer - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN` reader - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R = crate::BitReader<bool>;
#[doc = "Field `INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN` writer - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `INITA_TX_ADDR_NOT_SET_INTR_EN` reader - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INITA_TX_ADDR_NOT_SET_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `INITA_TX_ADDR_NOT_SET_INTR_EN` writer - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
pub type INITA_TX_ADDR_NOT_SET_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INIT_CONFIG_SPEC, bool, O>;
#[doc = "Field `INIT_CHANNEL_MAP` reader - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
pub type INIT_CHANNEL_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INIT_CHANNEL_MAP` writer - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
pub type INIT_CHANNEL_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INIT_CONFIG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable Initiator event start interrupt."]
    #[inline(always)]
    pub fn init_strt_en(&self) -> INIT_STRT_EN_R {
        INIT_STRT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Initiator event close interrupt."]
    #[inline(always)]
    pub fn init_close_en(&self) -> INIT_CLOSE_EN_R {
        INIT_CLOSE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables connection request packet transmission start interrupt."]
    #[inline(always)]
    pub fn conn_req_tx_en(&self) -> CONN_REQ_TX_EN_R {
        CONN_REQ_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable master connection created interrupt"]
    #[inline(always)]
    pub fn conn_created(&self) -> CONN_CREATED_R {
        CONN_CREATED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_self_rpa_unres_en(&self) -> INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R {
        INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_peer_rpa_unres_en(&self) -> INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R {
        INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr_en(&self) -> INITA_TX_ADDR_NOT_SET_INTR_EN_R {
        INITA_TX_ADDR_NOT_SET_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn init_channel_map(&self) -> INIT_CHANNEL_MAP_R {
        INIT_CHANNEL_MAP_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Initiator event start interrupt."]
    #[inline(always)]
    pub fn init_strt_en(&mut self) -> INIT_STRT_EN_W<0> {
        INIT_STRT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable Initiator event close interrupt."]
    #[inline(always)]
    pub fn init_close_en(&mut self) -> INIT_CLOSE_EN_W<1> {
        INIT_CLOSE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enables connection request packet transmission start interrupt."]
    #[inline(always)]
    pub fn conn_req_tx_en(&mut self) -> CONN_REQ_TX_EN_W<2> {
        CONN_REQ_TX_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable master connection created interrupt"]
    #[inline(always)]
    pub fn conn_created(&mut self) -> CONN_CREATED_W<4> {
        CONN_CREATED_W::new(self)
    }
    #[doc = "Bit 5 - Enable ADV self address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_self_rpa_unres_en(
        &mut self,
    ) -> INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W<5> {
        INIT_ADV_RX_INTR_SELF_RPA_UNRES_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enable ADV peer address RPA unresolved interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn init_adv_rx_intr_peer_rpa_unres_en(
        &mut self,
    ) -> INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W<6> {
        INIT_ADV_RX_INTR_PEER_RPA_UNRES_EN_W::new(self)
    }
    #[doc = "Bit 7 - Enable INITA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_INIT are set."]
    #[inline(always)]
    pub fn inita_tx_addr_not_set_intr_en(&mut self) -> INITA_TX_ADDR_NOT_SET_INTR_EN_W<7> {
        INITA_TX_ADDR_NOT_SET_INTR_EN_W::new(self)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for initiator scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn init_channel_map(&mut self) -> INIT_CHANNEL_MAP_W<13> {
        INIT_CHANNEL_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initiator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_config](index.html) module"]
pub struct INIT_CONFIG_SPEC;
impl crate::RegisterSpec for INIT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_config::R](R) reader structure"]
impl crate::Readable for INIT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_config::W](W) writer structure"]
impl crate::Writable for INIT_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_CONFIG to value 0"]
impl crate::Resettable for INIT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
