#[doc = "Register `ADV_CONFIG` reader"]
pub struct R(crate::R<ADV_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_CONFIG` writer"]
pub struct W(crate::W<ADV_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_CONFIG_SPEC>;
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
impl From<crate::W<ADV_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_STRT_EN` reader - Enable advertising event start interrupt."]
pub type ADV_STRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_STRT_EN` writer - Enable advertising event start interrupt."]
pub type ADV_STRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_CLS_EN` reader - Enable advertising event stop interrupt."]
pub type ADV_CLS_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_CLS_EN` writer - Enable advertising event stop interrupt."]
pub type ADV_CLS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_TX_EN` reader - Enable adv packet transmitted interrupt."]
pub type ADV_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_TX_EN` writer - Enable adv packet transmitted interrupt."]
pub type ADV_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_RSP_TX_EN` reader - Enable scan response packet transmitted interrupt."]
pub type SCN_RSP_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_RSP_TX_EN` writer - Enable scan response packet transmitted interrupt."]
pub type SCN_RSP_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_SCN_REQ_RX_EN` reader - Enable scan request packet received interrupt."]
pub type ADV_SCN_REQ_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_SCN_REQ_RX_EN` writer - Enable scan request packet received interrupt."]
pub type ADV_SCN_REQ_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_CONN_REQ_RX_EN` reader - Enable connect request packet received interrupt."]
pub type ADV_CONN_REQ_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_CONN_REQ_RX_EN` writer - Enable connect request packet received interrupt."]
pub type ADV_CONN_REQ_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `SLV_CONNECTED_EN` reader - Enable slave connected interrupt."]
pub type SLV_CONNECTED_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CONNECTED_EN` writer - Enable slave connected interrupt."]
pub type SLV_CONNECTED_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_TIMEOUT_EN` reader - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
pub type ADV_TIMEOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_TIMEOUT_EN` writer - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
pub type ADV_TIMEOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_RAND_DISABLE` reader - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
pub type ADV_RAND_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `ADV_RAND_DISABLE` writer - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
pub type ADV_RAND_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_SCN_PEER_RPA_UNMCH_EN` reader - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type ADV_SCN_PEER_RPA_UNMCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_SCN_PEER_RPA_UNMCH_EN` writer - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type ADV_SCN_PEER_RPA_UNMCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_CONN_PEER_RPA_UNMCH_EN` reader - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type ADV_CONN_PEER_RPA_UNMCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_CONN_PEER_RPA_UNMCH_EN` writer - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type ADV_CONN_PEER_RPA_UNMCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_PKT_INTERVAL` reader - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
pub type ADV_PKT_INTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADV_PKT_INTERVAL` writer - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
pub type ADV_PKT_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADV_CONFIG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Enable advertising event start interrupt."]
    #[inline(always)]
    pub fn adv_strt_en(&self) -> ADV_STRT_EN_R {
        ADV_STRT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&self) -> ADV_CLS_EN_R {
        ADV_CLS_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&self) -> ADV_TX_EN_R {
        ADV_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&self) -> SCN_RSP_TX_EN_R {
        SCN_RSP_TX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&self) -> ADV_SCN_REQ_RX_EN_R {
        ADV_SCN_REQ_RX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&self) -> ADV_CONN_REQ_RX_EN_R {
        ADV_CONN_REQ_RX_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&self) -> SLV_CONNECTED_EN_R {
        SLV_CONNECTED_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&self) -> ADV_TIMEOUT_EN_R {
        ADV_TIMEOUT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&self) -> ADV_RAND_DISABLE_R {
        ADV_RAND_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&self) -> ADV_SCN_PEER_RPA_UNMCH_EN_R {
        ADV_SCN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&self) -> ADV_CONN_PEER_RPA_UNMCH_EN_R {
        ADV_CONN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
    #[inline(always)]
    pub fn adv_pkt_interval(&self) -> ADV_PKT_INTERVAL_R {
        ADV_PKT_INTERVAL_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable advertising event start interrupt."]
    #[inline(always)]
    pub fn adv_strt_en(&mut self) -> ADV_STRT_EN_W<0> {
        ADV_STRT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&mut self) -> ADV_CLS_EN_W<1> {
        ADV_CLS_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&mut self) -> ADV_TX_EN_W<2> {
        ADV_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&mut self) -> SCN_RSP_TX_EN_W<3> {
        SCN_RSP_TX_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&mut self) -> ADV_SCN_REQ_RX_EN_W<4> {
        ADV_SCN_REQ_RX_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&mut self) -> ADV_CONN_REQ_RX_EN_W<5> {
        ADV_CONN_REQ_RX_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&mut self) -> SLV_CONNECTED_EN_W<6> {
        SLV_CONNECTED_EN_W::new(self)
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&mut self) -> ADV_TIMEOUT_EN_W<7> {
        ADV_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&mut self) -> ADV_RAND_DISABLE_W<8> {
        ADV_RAND_DISABLE_W::new(self)
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&mut self) -> ADV_SCN_PEER_RPA_UNMCH_EN_W<9> {
        ADV_SCN_PEER_RPA_UNMCH_EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&mut self) -> ADV_CONN_PEER_RPA_UNMCH_EN_W<10> {
        ADV_CONN_PEER_RPA_UNMCH_EN_W::new(self)
    }
    #[doc = "Bits 11:15 - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
    #[inline(always)]
    pub fn adv_pkt_interval(&mut self) -> ADV_PKT_INTERVAL_W<11> {
        ADV_PKT_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertiser configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_config](index.html) module"]
pub struct ADV_CONFIG_SPEC;
impl crate::RegisterSpec for ADV_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_config::R](R) reader structure"]
impl crate::Readable for ADV_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_config::W](W) writer structure"]
impl crate::Writable for ADV_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_CONFIG to value 0x20ff"]
impl crate::Resettable for ADV_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20ff
    }
}
