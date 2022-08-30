#[doc = "Register `SCAN_CONFIG` reader"]
pub struct R(crate::R<SCAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_CONFIG` writer"]
pub struct W(crate::W<SCAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_CONFIG_SPEC>;
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
impl From<crate::W<SCAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCN_STRT_EN` reader - Enable scan event start interrupt."]
pub type SCN_STRT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_STRT_EN` writer - Enable scan event start interrupt."]
pub type SCN_STRT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_CLOSE_EN` reader - Enable scan event close interrupt."]
pub type SCN_CLOSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_CLOSE_EN` writer - Enable scan event close interrupt."]
pub type SCN_CLOSE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_TX_EN` reader - Enable scan request packet transmitted interrupt."]
pub type SCN_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_TX_EN` writer - Enable scan request packet transmitted interrupt."]
pub type SCN_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_RX_EN` reader - Enable adv packet received interrupt ."]
pub type ADV_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADV_RX_EN` writer - Enable adv packet received interrupt ."]
pub type ADV_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_RSP_RX_EN` reader - Enable scan_rsp packet received interrupt ."]
pub type SCN_RSP_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_RSP_RX_EN` writer - Enable scan_rsp packet received interrupt ."]
pub type SCN_RSP_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN` reader - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN` writer - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN` reader - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN` writer - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCANA_TX_ADDR_NOT_SET_INTR_EN` reader - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCANA_TX_ADDR_NOT_SET_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCANA_TX_ADDR_NOT_SET_INTR_EN` writer - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type SCANA_TX_ADDR_NOT_SET_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN` reader - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R = crate::BitReader<bool>;
#[doc = "Field `RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN` writer - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
pub type RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `BACKOFF_ENABLE` reader - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
pub type BACKOFF_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BACKOFF_ENABLE` writer - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
pub type BACKOFF_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `SCAN_CHANNEL_MAP` reader - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
pub type SCAN_CHANNEL_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCAN_CHANNEL_MAP` writer - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
pub type SCAN_CHANNEL_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCAN_CONFIG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable scan event start interrupt."]
    #[inline(always)]
    pub fn scn_strt_en(&self) -> SCN_STRT_EN_R {
        SCN_STRT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable scan event close interrupt."]
    #[inline(always)]
    pub fn scn_close_en(&self) -> SCN_CLOSE_EN_R {
        SCN_CLOSE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable scan request packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_tx_en(&self) -> SCN_TX_EN_R {
        SCN_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable adv packet received interrupt ."]
    #[inline(always)]
    pub fn adv_rx_en(&self) -> ADV_RX_EN_R {
        ADV_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable scan_rsp packet received interrupt ."]
    #[inline(always)]
    pub fn scn_rsp_rx_en(&self) -> SCN_RSP_RX_EN_R {
        SCN_RSP_RX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_peer_rpa_unmch_en(&self) -> SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R {
        SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_self_rpa_unmch_en(&self) -> SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R {
        SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr_en(&self) -> SCANA_TX_ADDR_NOT_SET_INTR_EN_R {
        SCANA_TX_ADDR_NOT_SET_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_scn(&self) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
    #[inline(always)]
    pub fn backoff_enable(&self) -> BACKOFF_ENABLE_R {
        BACKOFF_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn scan_channel_map(&self) -> SCAN_CHANNEL_MAP_R {
        SCAN_CHANNEL_MAP_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable scan event start interrupt."]
    #[inline(always)]
    pub fn scn_strt_en(&mut self) -> SCN_STRT_EN_W<0> {
        SCN_STRT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable scan event close interrupt."]
    #[inline(always)]
    pub fn scn_close_en(&mut self) -> SCN_CLOSE_EN_W<1> {
        SCN_CLOSE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enable scan request packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_tx_en(&mut self) -> SCN_TX_EN_W<2> {
        SCN_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enable adv packet received interrupt ."]
    #[inline(always)]
    pub fn adv_rx_en(&mut self) -> ADV_RX_EN_W<3> {
        ADV_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable scan_rsp packet received interrupt ."]
    #[inline(always)]
    pub fn scn_rsp_rx_en(&mut self) -> SCN_RSP_RX_EN_W<4> {
        SCN_RSP_RX_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable ADV peer address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_peer_rpa_unmch_en(&mut self) -> SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W<5> {
        SCN_ADV_RX_INTR_PEER_RPA_UNMCH_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enable ADV self address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scn_adv_rx_intr_self_rpa_unmch_en(&mut self) -> SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W<6> {
        SCN_ADV_RX_INTR_SELF_RPA_UNMCH_EN_W::new(self)
    }
    #[doc = "Bit 7 - Enable SCANA RPA TX not set interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn scana_tx_addr_not_set_intr_en(&mut self) -> SCANA_TX_ADDR_NOT_SET_INTR_EN_W<7> {
        SCANA_TX_ADDR_NOT_SET_INTR_EN_W::new(self)
    }
    #[doc = "Bit 8 - This bit controls the SCAN engine behavior when an self address match occurs but a privacy mismatch occurs 0 - The packet is aborted 1 - The packet is received and reported to the Link Layer firmware This bit is valid only if PRIV_1_2 and PRIV_1_2_SCAN are set."]
    #[inline(always)]
    pub fn rpt_self_addr_match_priv_mismatch_scn(
        &mut self,
    ) -> RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W<8> {
        RPT_SELF_ADDR_MATCH_PRIV_MISMATCH_SCN_W::new(self)
    }
    #[doc = "Bit 11 - Enable random backoff feature in scanner. 1 - enable 0 - disable"]
    #[inline(always)]
    pub fn backoff_enable(&mut self) -> BACKOFF_ENABLE_W<11> {
        BACKOFF_ENABLE_W::new(self)
    }
    #[doc = "Bits 13:15 - Advertising channels that are enabled for scanning operation. Bit 15: setting 1 - enables channel 39 for use. Bit 14: setting 1 - enables channel 38 for use. Bit 13: setting 1 - enables channel 37 for use."]
    #[inline(always)]
    pub fn scan_channel_map(&mut self) -> SCAN_CHANNEL_MAP_W<13> {
        SCAN_CHANNEL_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_config](index.html) module"]
pub struct SCAN_CONFIG_SPEC;
impl crate::RegisterSpec for SCAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_config::R](R) reader structure"]
impl crate::Readable for SCAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_config::W](W) writer structure"]
impl crate::Writable for SCAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_CONFIG to value 0xe01f"]
impl crate::Resettable for SCAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe01f
    }
}
