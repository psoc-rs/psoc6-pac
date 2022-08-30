#[doc = "Register `ADV_INTR` reader"]
pub struct R(crate::R<ADV_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_INTR` writer"]
pub struct W(crate::W<ADV_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_INTR_SPEC>;
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
impl From<crate::W<ADV_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_STRT_INTR` reader - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_STRT_INTR_R = crate::BitReader<bool>;
#[doc = "Field `ADV_STRT_INTR` writer - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_STRT_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `ADV_CLOSE_INTR` reader - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_CLOSE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `ADV_CLOSE_INTR` writer - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_CLOSE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `ADV_TX_INTR` reader - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_TX_INTR_R = crate::BitReader<bool>;
#[doc = "Field `ADV_TX_INTR` writer - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_TX_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `SCAN_RSP_TX_INTR` reader - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type SCAN_RSP_TX_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_RSP_TX_INTR` writer - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type SCAN_RSP_TX_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `SCAN_REQ_RX_INTR` reader - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type SCAN_REQ_RX_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_REQ_RX_INTR` writer - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type SCAN_REQ_RX_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `CONN_REQ_RX_INTR` reader - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type CONN_REQ_RX_INTR_R = crate::BitReader<bool>;
#[doc = "Field `CONN_REQ_RX_INTR` writer - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
pub type CONN_REQ_RX_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `SLV_CONNECTED` reader - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
pub type SLV_CONNECTED_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CONNECTED` writer - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
pub type SLV_CONNECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `ADV_TIMEOUT` reader - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `ADV_TIMEOUT` writer - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
pub type ADV_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `ADV_ON` reader - Advertiser procedure is ON in hardware. Indicates that advertiser procedure is ON in hardware. 1 - ON 0 - OFF"]
pub type ADV_ON_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CONN_PEER_RPA_UNMCH_INTR` reader - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SLV_CONN_PEER_RPA_UNMCH_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CONN_PEER_RPA_UNMCH_INTR` writer - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SLV_CONN_PEER_RPA_UNMCH_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `SCAN_REQ_RX_PEER_RPA_UNMCH_INTR` reader - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_REQ_RX_PEER_RPA_UNMCH_INTR` writer - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `INIT_ADDR_MATCH_PRIV_MISMATCH_INTR` reader - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INIT_ADDR_MATCH_PRIV_MISMATCH_INTR` writer - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
#[doc = "Field `SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR` reader - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR` writer - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
pub type SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADV_INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_strt_intr(&self) -> ADV_STRT_INTR_R {
        ADV_STRT_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_close_intr(&self) -> ADV_CLOSE_INTR_R {
        ADV_CLOSE_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_tx_intr(&self) -> ADV_TX_INTR_R {
        ADV_TX_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_rsp_tx_intr(&self) -> SCAN_RSP_TX_INTR_R {
        SCAN_RSP_TX_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_req_rx_intr(&self) -> SCAN_REQ_RX_INTR_R {
        SCAN_REQ_RX_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn conn_req_rx_intr(&self) -> CONN_REQ_RX_INTR_R {
        CONN_REQ_RX_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
    #[inline(always)]
    pub fn slv_connected(&self) -> SLV_CONNECTED_R {
        SLV_CONNECTED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_timeout(&self) -> ADV_TIMEOUT_R {
        ADV_TIMEOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Advertiser procedure is ON in hardware. Indicates that advertiser procedure is ON in hardware. 1 - ON 0 - OFF"]
    #[inline(always)]
    pub fn adv_on(&self) -> ADV_ON_R {
        ADV_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_unmch_intr(&self) -> SLV_CONN_PEER_RPA_UNMCH_INTR_R {
        SLV_CONN_PEER_RPA_UNMCH_INTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_req_rx_peer_rpa_unmch_intr(&self) -> SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R {
        SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn init_addr_match_priv_mismatch_intr(&self) -> INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_addr_match_priv_mismatch_intr(&self) -> SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R {
        SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates a new advertising event started after interval expiry. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_strt_intr(&mut self) -> ADV_STRT_INTR_W<0> {
        ADV_STRT_INTR_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set it indicates current advertising event is closed. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_close_intr(&mut self) -> ADV_CLOSE_INTR_W<1> {
        ADV_CLOSE_INTR_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set it indicates ADV packet is transmitted. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_tx_intr(&mut self) -> ADV_TX_INTR_W<2> {
        ADV_TX_INTR_W::new(self)
    }
    #[doc = "Bit 3 - If this bit is set it indicates scan response packet transmitted in response to previous scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_rsp_tx_intr(&mut self) -> SCAN_RSP_TX_INTR_W<3> {
        SCAN_RSP_TX_INTR_W::new(self)
    }
    #[doc = "Bit 4 - If this bit is set it indicates scan request packet received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn scan_req_rx_intr(&mut self) -> SCAN_REQ_RX_INTR_W<4> {
        SCAN_REQ_RX_INTR_W::new(self)
    }
    #[doc = "Bit 5 - If this bit is set it indicates connect request packet is received. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn conn_req_rx_intr(&mut self) -> CONN_REQ_RX_INTR_W<5> {
        CONN_REQ_RX_INTR_W::new(self)
    }
    #[doc = "Bit 6 - If this bit is set it indicates that connection is created as slave. Write to the register with this bit set to 1, clears the interrupt source. Note: On a slave connection creation, the link layer cannot enter deepsleep mode in the same slot . It can enter deepsleep mode only in the subsequent slots."]
    #[inline(always)]
    pub fn slv_connected(&mut self) -> SLV_CONNECTED_W<6> {
        SLV_CONNECTED_W::new(self)
    }
    #[doc = "Bit 7 - If this bit is set it indicates that the directed advertising event has timed out after 1.28 seconds. Applicable in adv_direct_ind advertising. Write to the register with this bit set to 1, clears the interrupt source."]
    #[inline(always)]
    pub fn adv_timeout(&mut self) -> ADV_TIMEOUT_W<7> {
        ADV_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - If this bit is set it indicates that connection is created as slave, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. If the address is not resolved prior to connection establishment, the connection will be terminated. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn slv_conn_peer_rpa_unmch_intr(&mut self) -> SLV_CONN_PEER_RPA_UNMCH_INTR_W<9> {
        SLV_CONN_PEER_RPA_UNMCH_INTR_W::new(self)
    }
    #[doc = "Bit 10 - If this bit is set it indicates scan request packet received, but the peer device Resolvable Private Address is not resolved/ ID or NRPA are not matched yet. Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_req_rx_peer_rpa_unmch_intr(&mut self) -> SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W<10> {
        SCAN_REQ_RX_PEER_RPA_UNMCH_INTR_W::new(self)
    }
    #[doc = "Bit 11 - If this bit is set it indicates that an Identity address is received from a Scanner and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the Scanner Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn init_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W<11> {
        INIT_ADDR_MATCH_PRIV_MISMATCH_INTR_W::new(self)
    }
    #[doc = "Bit 12 - If this bit is set it indicates that an Identity address is received from an initiator and matches an entry in the resolving list, but peer IRK is set and hence a corresponding RPA is expected from the initiator Write to the register with this bit set to 1, clears the interrupt source. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn scan_addr_match_priv_mismatch_intr(
        &mut self,
    ) -> SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W<12> {
        SCAN_ADDR_MATCH_PRIV_MISMATCH_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertising interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_intr](index.html) module"]
pub struct ADV_INTR_SPEC;
impl crate::RegisterSpec for ADV_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_intr::R](R) reader structure"]
impl crate::Readable for ADV_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_intr::W](W) writer structure"]
impl crate::Writable for ADV_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_INTR to value 0"]
impl crate::Resettable for ADV_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
