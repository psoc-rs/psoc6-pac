#[doc = "Reader of register ADV_CONFIG"]
pub type R = crate::R<u32, super::ADV_CONFIG>;
#[doc = "Writer for register ADV_CONFIG"]
pub type W = crate::W<u32, super::ADV_CONFIG>;
#[doc = "Register ADV_CONFIG `reset()`'s with value 0x20ff"]
impl crate::ResetValue for super::ADV_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20ff
    }
}
#[doc = "Reader of field `ADV_STRT_EN`"]
pub type ADV_STRT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_STRT_EN`"]
pub struct ADV_STRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_STRT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADV_CLS_EN`"]
pub type ADV_CLS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_CLS_EN`"]
pub struct ADV_CLS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CLS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADV_TX_EN`"]
pub type ADV_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_TX_EN`"]
pub struct ADV_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SCN_RSP_TX_EN`"]
pub type SCN_RSP_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCN_RSP_TX_EN`"]
pub struct SCN_RSP_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCN_RSP_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADV_SCN_REQ_RX_EN`"]
pub type ADV_SCN_REQ_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_SCN_REQ_RX_EN`"]
pub struct ADV_SCN_REQ_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_SCN_REQ_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADV_CONN_REQ_RX_EN`"]
pub type ADV_CONN_REQ_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_CONN_REQ_RX_EN`"]
pub struct ADV_CONN_REQ_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CONN_REQ_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SLV_CONNECTED_EN`"]
pub type SLV_CONNECTED_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CONNECTED_EN`"]
pub struct SLV_CONNECTED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CONNECTED_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADV_TIMEOUT_EN`"]
pub type ADV_TIMEOUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_TIMEOUT_EN`"]
pub struct ADV_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TIMEOUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADV_RAND_DISABLE`"]
pub type ADV_RAND_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_RAND_DISABLE`"]
pub struct ADV_RAND_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_RAND_DISABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADV_SCN_PEER_RPA_UNMCH_EN`"]
pub type ADV_SCN_PEER_RPA_UNMCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_SCN_PEER_RPA_UNMCH_EN`"]
pub struct ADV_SCN_PEER_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_SCN_PEER_RPA_UNMCH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ADV_CONN_PEER_RPA_UNMCH_EN`"]
pub type ADV_CONN_PEER_RPA_UNMCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_CONN_PEER_RPA_UNMCH_EN`"]
pub struct ADV_CONN_PEER_RPA_UNMCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CONN_PEER_RPA_UNMCH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADV_PKT_INTERVAL`"]
pub type ADV_PKT_INTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADV_PKT_INTERVAL`"]
pub struct ADV_PKT_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_PKT_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable advertising event start interrupt."]
    #[inline(always)]
    pub fn adv_strt_en(&self) -> ADV_STRT_EN_R {
        ADV_STRT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&self) -> ADV_CLS_EN_R {
        ADV_CLS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&self) -> ADV_TX_EN_R {
        ADV_TX_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&self) -> SCN_RSP_TX_EN_R {
        SCN_RSP_TX_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&self) -> ADV_SCN_REQ_RX_EN_R {
        ADV_SCN_REQ_RX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&self) -> ADV_CONN_REQ_RX_EN_R {
        ADV_CONN_REQ_RX_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&self) -> SLV_CONNECTED_EN_R {
        SLV_CONNECTED_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&self) -> ADV_TIMEOUT_EN_R {
        ADV_TIMEOUT_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&self) -> ADV_RAND_DISABLE_R {
        ADV_RAND_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&self) -> ADV_SCN_PEER_RPA_UNMCH_EN_R {
        ADV_SCN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&self) -> ADV_CONN_PEER_RPA_UNMCH_EN_R {
        ADV_CONN_PEER_RPA_UNMCH_EN_R::new(((self.bits >> 10) & 0x01) != 0)
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
    pub fn adv_strt_en(&mut self) -> ADV_STRT_EN_W {
        ADV_STRT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable advertising event stop interrupt."]
    #[inline(always)]
    pub fn adv_cls_en(&mut self) -> ADV_CLS_EN_W {
        ADV_CLS_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable adv packet transmitted interrupt."]
    #[inline(always)]
    pub fn adv_tx_en(&mut self) -> ADV_TX_EN_W {
        ADV_TX_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enable scan response packet transmitted interrupt."]
    #[inline(always)]
    pub fn scn_rsp_tx_en(&mut self) -> SCN_RSP_TX_EN_W {
        SCN_RSP_TX_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable scan request packet received interrupt."]
    #[inline(always)]
    pub fn adv_scn_req_rx_en(&mut self) -> ADV_SCN_REQ_RX_EN_W {
        ADV_SCN_REQ_RX_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable connect request packet received interrupt."]
    #[inline(always)]
    pub fn adv_conn_req_rx_en(&mut self) -> ADV_CONN_REQ_RX_EN_W {
        ADV_CONN_REQ_RX_EN_W { w: self }
    }
    #[doc = "Bit 6 - Enable slave connected interrupt."]
    #[inline(always)]
    pub fn slv_connected_en(&mut self) -> SLV_CONNECTED_EN_W {
        SLV_CONNECTED_EN_W { w: self }
    }
    #[doc = "Bit 7 - Enable adv_timeout interrupt. Applicable in adv_direct_ind advertising."]
    #[inline(always)]
    pub fn adv_timeout_en(&mut self) -> ADV_TIMEOUT_EN_W {
        ADV_TIMEOUT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Disable randomization of adv interval. When disabled, interval is same as programmed in adv_interval register."]
    #[inline(always)]
    pub fn adv_rand_disable(&mut self) -> ADV_RAND_DISABLE_W {
        ADV_RAND_DISABLE_W { w: self }
    }
    #[doc = "Bit 9 - Enable scan request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_scn_peer_rpa_unmch_en(&mut self) -> ADV_SCN_PEER_RPA_UNMCH_EN_W {
        ADV_SCN_PEER_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bit 10 - Enable connect request packet received with peer device address unmatched interrupt. This bit is valid only if PRIV_1_2 and PRIV_1_2_ADV are set."]
    #[inline(always)]
    pub fn adv_conn_peer_rpa_unmch_en(&mut self) -> ADV_CONN_PEER_RPA_UNMCH_EN_W {
        ADV_CONN_PEER_RPA_UNMCH_EN_W { w: self }
    }
    #[doc = "Bits 11:15 - Time between the beginning of two consecutive advertising PDU's. Time = N * 0.625 msec Time Range: <=10msec."]
    #[inline(always)]
    pub fn adv_pkt_interval(&mut self) -> ADV_PKT_INTERVAL_W {
        ADV_PKT_INTERVAL_W { w: self }
    }
}
